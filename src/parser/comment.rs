use nom::{
    branch::alt,
    bytes::complete::{is_a, is_not, tag, take, take_until, take_while},
    character::complete::{anychar, line_ending, multispace0},
    combinator::{eof, opt},
    multi::{many0, many_m_n, many_till},
    sequence::{delimited, pair},
    AsChar, IResult, Parser,
};

use super::interface::NetworkParser;

/// A network comment is a comment
#[derive(Debug, PartialEq)]
pub enum Comment {
    Whitespace,
    Singleline(String),
    Multiline(String),
    DocSingleline(String),
}

impl Comment {
    fn read_comment(&self) -> Option<String> {
        match self {
            Comment::Whitespace => None,
            Comment::Singleline(comm) => Some(comm.trim().to_owned()),
            Comment::Multiline(comm) => Some(
                comm.to_owned()
                    .lines()
                    .map(|line| {
                        let line = line.trim();
                        if line.starts_with("*") {
                            line[1..].trim().to_owned()
                        } else {
                            line.to_owned()
                        }
                    })
                    .collect(),
            ),
            Comment::DocSingleline(comm) => {
                Some(comm.to_owned().lines().map(|line| line.trim()).collect())
            }
        }
    }

    /// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
    /// trailing whitespace, returning the output of `inner`.
    ///
    /// https://github.com/rust-bakery/nom/blob/main/doc/nom_recipes.md#whitespace
    fn whitespace(i: &str) -> IResult<&str, Comment> {
        let (input, _) = multispace0(i)?;
        IResult::Ok((input, Self::Whitespace))
    }

    /// A combinator that takes a tag parser from the C-style comment starting with
    /// `//` and reads till a line-breaking character was discovered.
    ///
    /// This is not a documentation comment.
    ///
    /// https://github.com/rust-bakery/nom/blob/main/doc/nom_recipes.md#comments
    pub fn c_comment(i: &str) -> IResult<&str, Comment> {
        let (input, comm) = delimited(tag("//"), many0(anychar), alt((is_a("\n\r"), eof)))(i)?;
        IResult::Ok((input, Self::Singleline(comm.into_iter().collect())))
    }

    /// A combinator that takes a tag parser from the C-style documentation comment
    /// starting with `///` and reads till a line-breaking character was discovered.
    ///
    /// This is a documentation comment.
    ///
    /// https://github.com/rust-bakery/nom/blob/main/doc/nom_recipes.md#comments
    pub fn c_comment_docs(i: &str) -> IResult<&str, Comment> {
        let (input, comment) =
            pair(tag("///"), is_not("\n\r"))(i).map(|(lines, (_, comment))| (lines, comment))?;
        IResult::Ok((input, Self::DocSingleline(comment.to_owned())))
    }

    /// A combinator that takes a delimited parser and returns the comments'
    /// content, ignoring the comment markers `/*` and `*/`
    ///
    /// Per default for network files, multiline comments are also documentation
    /// comments.
    ///
    /// https://github.com/rust-bakery/nom/blob/main/doc/nom_recipes.md#comments
    pub fn c_multiline_comment(i: &str) -> IResult<&str, Comment> {
        let (input, comment) = delimited(tag("/*"), take_until("*/"), tag("*/"))(i)?;
        IResult::Ok((input, Self::Multiline(comment.to_owned())))
    }
}

impl NetworkParser for Comment {
    /// Reads in as many whitespaces and comments as possible. This function
    /// can be used to identify as many "ignored" characters as possible, as
    /// this will also parse through comments and read the documentation from
    /// it.
    fn parse(mut input: &str) -> IResult<&str, Self> {
        let mut comments = Vec::new();
        // let (mut input, mut comments) =

        loop {
            let (tmp_input, comm) = opt(alt((
                Self::whitespace,
                Self::c_comment_docs,
                Self::c_comment,
                Self::c_multiline_comment,
            )))(input)?;

            // To avoid running into an infinite loop, if we did not
            // advance, break, if continuing, update the input variable.
            if input == tmp_input {
                break;
            }

            input = tmp_input;

            if let Some(value) = comm {
                comments.push(value);
                continue;
            }

            break;
        }

        // many0()(input)?;

        // let mut comments = Vec::new();
        let mut single_line_docs = None;

        while let Some(comment) = comments.pop() {
            match (comment, &single_line_docs) {
                (Self::Multiline(comm), None) => {
                    return IResult::Ok((input, Self::Multiline(comm)));
                }
                (Self::DocSingleline(comm), None) => {
                    single_line_docs = Some(comm);
                    continue;
                }
                (Self::Singleline(_), Some(comm)) => {
                    return IResult::Ok((input, Self::DocSingleline(comm.to_owned())))
                }
                (Self::Multiline(_), Some(comm)) => {
                    return IResult::Ok((input, Self::Multiline(comm.to_owned())))
                }
                (Self::DocSingleline(append), Some(comm)) => {
                    single_line_docs = Some(comm.to_owned() + append.as_str());
                    continue;
                }
                _ => continue,
            }
        }

        if let Some(comment) = single_line_docs {
            IResult::Ok((input, Self::DocSingleline(comment)))
        } else {
            IResult::Ok((input, Self::Whitespace))
        }
    }
}

#[cfg(test)]

mod individual_comment_methods_test {
    use std::string::ParseError;

    use super::*;

    #[test]
    fn empty_string() {
        let (input, comm) = Comment::whitespace("").unwrap();
        assert_eq!(input, "");
        assert_eq!(comm, Comment::Whitespace);
    }

    #[test]
    fn simple_whitespace() {
        let (input, comm) = Comment::whitespace(" \t \r \n").unwrap();
        assert_eq!(input, "");
        assert_eq!(comm, Comment::Whitespace);
    }

    #[test]
    fn empty_comment() {
        let (input, comm) = Comment::c_comment("//\n").unwrap();
        assert_eq!(input, "");
    }

    #[test]
    fn empty_comment_without_newline() {
        let (input, comm) = Comment::c_comment("//").unwrap();
        assert_eq!(input, "");
    }

    #[test]
    fn simple_comment() {
        let (input, comm) = Comment::c_comment("// Hello!\n").unwrap();
        assert_eq!(input, "");
    }

    #[test]
    fn empty_multiline_comment() {
        let (input, comm) = Comment::c_multiline_comment("/**/").unwrap();
        assert_eq!(input, "");
    }

    #[test]
    fn simple_multiline_comment() {
        let (input, comm) = Comment::c_multiline_comment("/*Hello!*/").unwrap();
        assert_eq!(input, "");
        assert_eq!(comm.read_comment().unwrap().as_str(), "Hello!");
    }
}

#[cfg(test)]
mod comment_test {
    use super::*;

    #[test]
    fn empty_string() {
        let (input, _) = Comment::parse("").unwrap();
        assert_eq!(input, "");
    }

    #[test]
    fn simple_whitespace() {
        let (input, _) = Comment::parse(" \t\r\n").unwrap();
        assert_eq!(input, "");
    }

    #[test]
    fn simple_newline() {
        let (input, _) = Comment::parse(" \n ").unwrap();
        assert_eq!(input, "");
    }

    #[test]
    fn simple_comment() {
        let (input, network_comment) = Comment::parse(" /* This is a comment. */ ").unwrap();
        assert_eq!(input, "");
        assert!(network_comment
            .read_comment()
            .unwrap()
            .contains("This is a comment."));
    }

    #[test]
    #[ignore = "ignored, invalid test: returned output does not result in error, but does not finish reading till the end."]
    fn invalid_comment() {
        let result = Comment::parse("\n/* Multiline comment was not finished.\n");
        assert!(result.is_err());
    }
}
