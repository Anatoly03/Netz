use nom::{
    branch::alt, bytes::complete::take_while, combinator::fail, error::make_error, multi::many0,
    IResult,
    error::ErrorKind
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
        let (input, _) = take_while(|chr: char| chr.is_whitespace())(input)?;
        IResult::Ok((input, ""))
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
        let (input, comment) = many0(Self::trim)(input)?;
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
        let input = "    ";
        let (output, _) = NetworkComment::parse(input).unwrap();
        assert_eq!(input, output);
    }

    #[test]
    fn simple_newline() {
        let input = "   \n ";
        let (output, _) = NetworkComment::parse(input).unwrap();
        assert_eq!(input, output);
    }

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
