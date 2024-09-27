//! This crate provides utility methods for the smallest elements used in the
//! template files, such as whitespace, comments and string related methods.

use nom::{
    branch::alt,
    bytes::complete::{is_a, tag, take_until},
    character::complete::{anychar, multispace0, multispace1},
    combinator::{eof, value},
    multi::many0,
    sequence::delimited,
    IResult, Parser,
};

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
///
/// https://github.com/rust-bakery/nom/blob/main/doc/nom_recipes.md#whitespace
fn whitespace0(input: &str) -> IResult<&str, ()> {
    value((), multispace0).parse(input)
}

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
///
/// https://github.com/rust-bakery/nom/blob/main/doc/nom_recipes.md#whitespace
pub fn whitespace1(input: &str) -> IResult<&str, ()> {
    value((), multispace1).parse(input)
}

/// A combinator that takes a tag parser from the C-style comment starting with
/// `//` and reads till a line-breaking character was discovered.
///
/// https://github.com/rust-bakery/nom/blob/main/doc/nom_recipes.md#comments
pub fn c_comment(input: &str) -> IResult<&str, ()> {
    value(
        (),
        delimited(tag("//"), many0(anychar), alt((is_a("\n\r"), eof))),
    )
    .parse(input)
}

/// A combinator that takes a delimited parser and returns the comments'
/// content, ignoring the comment markers `/*` and `*/`
///
/// Per default for network files, multiline comments are also documentation
/// comments.
///
/// https://github.com/rust-bakery/nom/blob/main/doc/nom_recipes.md#comments
pub fn c_multiline_comment(input: &str) -> IResult<&str, ()> {
    value((), delimited(tag("/*"), take_until("*/"), tag("*/"))).parse(input)
}

/// Reads in as many ignored characters as possible and terminates if the
/// input no longer advances.
pub fn read_ignored(mut input: &str) -> IResult<&str, bool> {
    let tmp_input = input;

    loop {
        let (tmp_input, output) = alt((c_comment, c_multiline_comment, whitespace0))(input)?;

        // To avoid running into an infinite loop, if we did not
        // advance, break, if continuing, update the input variable.
        // output.is_none() ?
        if input == tmp_input {
            break;
        }

        input = tmp_input;
        continue;
    }

    IResult::Ok((input, tmp_input != input))
}

#[macro_export]
macro_rules! whitespace {
    () => {
        let (input, _) = read_ignored(input)?;
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_whitespace() {
        let (input, _) = read_ignored("").unwrap();
        assert_eq!(input, "");
    }

    #[test]
    fn whitespace() {
        let (input, _) = read_ignored(" \n\t").unwrap();
        assert_eq!(input, "");
    }

    #[test]
    fn comment() {
        let (input, _) = read_ignored("//\n/**/\t").unwrap();
        assert_eq!(input, "");
    }
}
