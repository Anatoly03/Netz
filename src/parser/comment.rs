use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_until},
    character::complete::multispace0,
    combinator::eof,
    multi::many0,
    sequence::{delimited, pair},
    IResult,
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
        let (input, _) = alt((multispace0, eof))(i)?;
        IResult::Ok((input, Self::Whitespace))
    }

    /// A combinator that takes a tag parser from the C-style comment starting with
    /// `//` and reads till a line-breaking character was discovered.
    ///
    /// This is not a documentation comment.
    ///
    /// https://github.com/rust-bakery/nom/blob/main/doc/nom_recipes.md#comments
    pub fn c_comment(i: &str) -> IResult<&str, Comment> {
        let (input, comment) =
            pair(tag("//"), is_not("\n\r"))(i).map(|(lines, (_, comment))| (lines, comment))?;
        IResult::Ok((input, Self::Singleline(comment.to_owned())))
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
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, mut comments) = many0(alt((
            Self::whitespace,
            Self::c_comment_docs,
            Self::c_comment,
            Self::c_multiline_comment,
        )))(input)?;

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
mod field_test {
    use super::*;

    #[test]
    fn simple_whitespace() {
        let (input, _) = Comment::parse("  ").unwrap();
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
