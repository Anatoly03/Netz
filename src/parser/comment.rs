use nom::{
    branch::alt, bytes::complete::take_while, combinator::{eof, fail}, error::{make_error, ErrorKind}, multi::many0,
    IResult
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

    /// Trim all currently leading whitespace, if any. Under the hood,
    /// it trims all characters that return positive to `char.is_whitespace()`
    fn trim(input: &str) -> IResult<&str, &str> {
        take_while(|chr: char| chr.is_whitespace())(input)
    }

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
        let (input, comment) =Self::trim(input)?;
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

    /// Comments are currently not supported.
    #[ignore]
    #[test]
    fn simple_comment() {
        let input = " /* This is a comment. */ ";
        let (output, network_comment) = NetworkComment::parse(input).unwrap();
        assert_eq!(input, output);
        assert!(network_comment
            .comment
            .unwrap()
            .contains("This is a comment."));
    }
}
