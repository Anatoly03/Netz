//! This crate provides utility methods for the smallest elements used in the
//! template files, such as whitespace, comments and string related methods.

use nom::{
    branch::alt,
    bytes::complete::{is_a, tag, take_until},
    character::complete::{alphanumeric1, anychar, multispace1},
    combinator::{eof, map, recognize, value},
    multi::{many0, many0_count},
    sequence::{delimited, pair},
    IResult, Parser,
};

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
///
/// ```
/// use tmpl_generator::parser::util::whitespace;
///
/// assert!(whitespace(" ").is_ok());
/// assert!(whitespace("\n").is_ok());
/// assert!(whitespace("\t").is_ok());
///
/// // EOF is not be accepted as whitespace to avoid infinite loops in whitespace parsing.
/// assert!(whitespace("").is_err());
/// ```
///
/// https://github.com/rust-bakery/nom/blob/main/doc/nom_recipes.md#whitespace
pub fn whitespace(input: &str) -> IResult<&str, ()> {
    value((), multispace1).parse(input)
}

/// A combinator that takes a tag parser from the C-style comment starting with
/// `//` and reads till a line-breaking character was discovered.
///
/// ```
/// use tmpl_generator::parser::util::c_comment;
///
/// assert!(c_comment("// Hello World!\n").is_ok());
/// ```
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

/// A combinator that reads an identifier as specified by most programming
/// languages, i.e. a literal consisting of letters, numbers and the underscore,
/// but not starting with a number
/// 
/// ```
/// use tmpl_generator::parser::util::identifier;
///
/// assert_eq!(identifier("FooBar").unwrap(), ("", "FooBar"));
/// assert_eq!(identifier("fooBar123").unwrap(), ("", "fooBar123"));
/// assert!(identifier("123fooBar").is_err());
/// ```
pub fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        is_a("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_"),
        many0_count(alphanumeric1),
    ))
    .parse(input)
}

/// A variable is a vector if identifiers separated by a dot
/// 
/// ```
/// use tmpl_generator::parser::util::variable;
/// 
/// assert_eq!(variable("hello.world").unwrap(), ("", vec!["hello", "world"]));
/// assert_eq!(variable("root.child.child").unwrap(), ("", vec!["root", "child", "child"]));
/// assert_eq!(variable("simple").unwrap(), ("", vec!["simple"]));
/// ```
pub fn variable(input: &str) -> IResult<&str, Vec<&str>> {
    let (input, (parent, mut children)) = pair(identifier, many0(map(pair(tag("."), identifier), |(a, b): (&str, &str)| b))).parse(input)?;
    children.insert(0, parent);
    IResult::Ok((input, children))
}

