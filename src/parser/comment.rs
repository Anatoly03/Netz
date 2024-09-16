use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_until, take_while},
    character::complete::multispace0,
    combinator::{eof, fail},
    error::{make_error, ErrorKind},
    multi::many0,
    sequence::{delimited, pair},
    IResult,
};

use super::interface::NetworkParser;
// use super::general::fail;

/// A network comment is a comment
#[derive(Debug, PartialEq)]
pub struct NetworkComment {
    comment: Option<String>,
}

impl NetworkComment {
    pub fn new() -> Self {
        Self { comment: None }
    }

    /// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
    /// trailing whitespace, returning the output of `inner`.
    ///
    /// https://github.com/rust-bakery/nom/blob/main/doc/nom_recipes.md#whitespace
    fn whitespace(i: &str) -> IResult<&str, &str> {
        alt((multispace0, eof))(i)
    }

    /// A combinator that takes a tag parser from the C-style comment start `//`
    /// and reads till a line-breaking character was discovered.
    ///
    /// https://github.com/rust-bakery/nom/blob/main/doc/nom_recipes.md#comments
    pub fn c_comment(i: &str) -> IResult<&str, &str> {
        pair(tag("//"), is_not("\n\r"))(i).map(|(lines, (_, comment))| (lines, comment))
    }

    /// A combinator that takes a delimited parser and returns the comments'
    /// content, ignoring the comment markers `/*` and `*/`
    ///
    /// https://github.com/rust-bakery/nom/blob/main/doc/nom_recipes.md#comments
    pub fn c_multiline_comment(i: &str) -> IResult<&str, &str> {
        delimited(tag("/*"), take_until("*/"), tag("*/"))(i)
    }

    // /// Trim all currently leading whitespace, if any. Under the hood,
    // /// it trims all characters that return positive to `char.is_whitespace()`
    // fn trim(input: &str) -> IResult<&str, &str> {
    //     take_while(|chr: char| chr.is_whitespace() || chr == '\n')(input)
    // }

    // /// Trim all currently leading whitespace, if any. Under the hood,
    // /// it trims all characters that return positive to `char.is_whitespace()`
    // fn read_comment(input: &str) -> IResult<&str, &str> {
    //     TODO implement comments
    //     fail("Not Implemented")
    //     IResult::Err(make_error(input, ErrorKind::Fail))
    // }
}

impl NetworkParser for NetworkComment {
    /// Reads in as many whitespaces and comments as possible. This function
    /// can be used to identify as many "ignored" characters as possible.
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, comment) = Self::whitespace(input)?;
        // let (input, comment) = many0(alt((Self::trim, Self::read_comment)))(input)?;
        // TODO implement comments
        Ok((input, Self::new()))
    }
}

#[cfg(test)]
mod field_test {
    use super::*;

    #[test]
    fn simple_whitespace() {
        let (input, _) = NetworkComment::parse("  ").unwrap();
        assert_eq!(input, "");
    }

    #[test]
    fn simple_newline() {
        let (input, _) = NetworkComment::parse(" \n ").unwrap();
        assert_eq!(input, "");
    }

    #[ignore = "Comments are currently not supported."]
    #[test]
    fn simple_comment() {
        let (input, network_comment) = NetworkComment::parse(" /* This is a comment. */ ").unwrap();
        assert_eq!(input, "");
        assert!(network_comment
            .comment
            .unwrap()
            .contains("This is a comment."));
    }
}
