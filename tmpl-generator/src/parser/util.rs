//! This crate provides utility methods for the smallest elements used in the
//! template files, such as whitespace, comments and string related methods.

use nom::{
    branch::alt, bytes::complete::{is_a, is_not, tag, take_until}, character::complete::{alphanumeric1, anychar, multispace1}, combinator::{eof, map, opt, recognize, value}, multi::{many0, many0_count}, sequence::{delimited, pair}, IResult, Parser
};

use super::components::{ForeachScope, TemplateElement};

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

/// Reads in as many ignored characters as possible and terminates if the
/// input no longer advances.
///
/// ```
/// use tmpl_generator::parser::util::read_ignored;
///
/// assert!(read_ignored("// Hello World!\n/* Even More Comments*/\t\n").is_ok());
/// ```
pub fn read_ignored(mut input: &str) -> IResult<&str, ()> {
    loop {
        let (tmp_input, _) = opt(alt((whitespace, c_comment, c_multiline_comment)))(input)?;

        // To avoid running into an infinite loop, if we did not
        // advance, break, if continuing, update the input variable.
        if input == tmp_input {
            break;
        }

        input = tmp_input;
        continue;
    }

    IResult::Ok((input, ()))
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

/// A variable is a vector if identifiers separated by a dot.
///
/// // ```
/// // use tmpl_generator::parser::util::variable;
/// // use tmpl_generator::parser::components::TemplateElement;
/// // 
/// // assert_eq!(variable("hello.world").unwrap(), ("", TemplateElement::Variable(vec!["hello".to_owned(), "world".to_owned()])));
/// // assert_eq!(variable("root.child.child").unwrap(), ("", TemplateElement::Variable(vec!["root".to_owned(), "child".to_owned(), "child".to_owned()])));
/// // assert_eq!(variable("simple").unwrap(), ("", TemplateElement::Variable(vec!["simple".to_owned()])));
/// // ```
pub fn variable(input: &str) -> IResult<&str, Vec<String>> {
    let (input, (parent, mut children)) = pair(
        identifier,
        many0(map(pair(tag("."), identifier), |(_, b): (&str, &str)| b)),
    )
    .parse(input)?;
    children.insert(0, parent);
    IResult::Ok((input, children.into_iter().map(|s| s.to_owned()).collect()))
}

/// Escape a string literal fragment, this function assumes that '\' was
/// already read.
///
/// ```
/// use tmpl_generator::parser::util::escape_string_literal;
///
/// assert_eq!(escape_string_literal("n").unwrap(), ("", "\n"));
/// assert_eq!(escape_string_literal("\\").unwrap(), ("", "\\"));
/// assert_eq!(escape_string_literal("\"").unwrap(), ("", "\""));
/// ```
pub fn escape_string_literal(input: &str) -> IResult<&str, &str> {
    alt((
        value("\n", nom::character::complete::char('n')),
        value("\t", nom::character::complete::char('t')),
        value("\\", nom::character::complete::char('\\')),
        value("\"", nom::character::complete::char('\"')),
    )).parse(input)
}

/// A string literal is a literal starting and closing with quotation marks. The
/// closing quotation mark can be escaped with a backslash, which is used as a
/// general escape character.
/// 
/// // ```
/// // use tmpl_generator::parser::util::string_literal;
/// // use tmpl_generator::parser::components::TemplateElement;
/// //  
/// // assert_eq!(string_literal("\"\"").unwrap(), ("", TemplateElement::StringLiteral("".to_owned())));
/// // assert_eq!(string_literal("\"Hello!\"").unwrap(), ("", TemplateElement::StringLiteral("Hello!".to_owned())));
/// // // "Hello, World!\n"  :  "\"Hello, World!\n\""  :  \"\\\"Hello, World!\\n\\\"\"
/// // assert_eq!(string_literal("\"\\\"Hello, World!\\n\\\"\"").unwrap(), ("", TemplateElement::StringLiteral("\"Hello, World!\n\"".to_owned())));
/// // ```
pub fn string_literal(input: &str) -> IResult<&str, String> {
    let mut value = String::from("");

    // // Recognize the empty string.
    // if let IResult::Ok((i, _)) = tag::<&str, &str, Error<&str>>("\'\'").parse(input) {
    //     return IResult::Ok((i, value));
    // }

    // Discard the first quotation mark.
    let (mut input, _) = tag("\"").parse(input)?;

    // Read in string components
    loop {
        let (tmp_input, fragment) = opt(is_not("\\\"")).parse(input)?;
        input = tmp_input;

        if let Some(part) = fragment {
            value += part;
        }

        let c = input.chars().next().unwrap();
        input = &input[1..];

        match c {
            '\\' => {
                let (tmp_input, part) = escape_string_literal.parse(input)?;
                input = tmp_input;
                value += part;
            }
            '\"' => break,
            _ => unreachable!(),
        }
    }

    IResult::Ok((input, value))
}

pub fn foreach_scope(input: &str) -> IResult<&str, ForeachScope> {
    let (input, _) = tag("foreach").parse(input)?;
    let (input, _) = read_ignored.parse(input)?;
    let (input, value) = identifier.map(|s| s.to_owned()).parse(input)?;
    let (input, _) = read_ignored.parse(input)?;
    let (input, _) = tag(":").parse(input)?;
    let (input, _) = read_ignored.parse(input)?;
    let (input, variable) = variable.parse(input)?;
    let (input, _) = read_ignored.parse(input)?;
    let (input, scope) = surrounded_scope.parse(input)?;

    IResult::Ok((input, ForeachScope {
        value,
        variable,
        scope,
    }))
}

pub fn surrounded_scope(input: &str) -> IResult<&str, Vec<TemplateElement>> {
    delimited(tag("("), scope, tag(")")).parse(input)
}

/// Starts parsing a scope. This menthod itself does not require the parentheses,
/// but will provide them in a recursive call to self.
pub fn scope(input: &str) -> IResult<&str, Vec<TemplateElement>> {
    let (input, structures) = many0(alt((
        surrounded_scope.map(|s| TemplateElement::Scope(s)),
        foreach_scope.map(|foreach| TemplateElement::Foreach(foreach)),
        value(TemplateElement::Ignored, read_ignored),
        string_literal.map(|s| TemplateElement::StringLiteral(s)),
        variable.map(|v| TemplateElement::Variable(v)),
    )))
    .parse(input)?;

    IResult::Ok((input, structures.into_iter().filter(|e| TemplateElement::Ignored != *e).collect()))
}
