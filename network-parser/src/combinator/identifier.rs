//! This module contains the identifier combinator. It will parse

use nom::{
    self, bytes::complete::is_a, character::complete::alphanumeric1, combinator::recognize,
    multi::many0_count, sequence::pair, Parser,
};
use util_cases::CaseStyles;

use crate::combinator::whitespace::read_ignored;

///
#[derive(Clone)]
pub struct Identifier(Vec<String>);

impl Identifier {
    /// Create a new, empty Identifier
    pub fn new() -> Self {
        Self(vec![])
    }

    /// Parse a new identifier from input.
    pub fn parse(input: &str) -> nom::IResult<&str, Self> {
        Ok(Self::new().parse(input)?)
    }
}

impl CaseStyles for Identifier {
    fn to_split_case(&self) -> Vec<String> {
        self.0.clone()
    }
}

impl<'a, E: nom::error::ParseError<&'a str>> Parser<&'a str, Self, E> for Identifier {
    fn parse(&mut self, input: &'a str) -> nom::IResult<&'a str, Self, E> {
        // TODO let (input, _comment) = Comment::parse(input)?;
        // let (input, _) = read_ignored(input)?;

        let (input, identity) = recognize(pair(
            is_a("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ-_"),
            many0_count(alphanumeric1),
        ))(input)?;

        self.0.append(&mut identity.to_split_case());

        nom::IResult::Ok((input, self.clone()))
    }
}

// #[macro_export]
// macro_rules! identifier {
//     ($x:expr) => {
//         let (input, $x) = Identifier::parse(input)?;
//     };
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        Identifier::parse("HelloWorld").expect("The identifier should parse `HelloWorld`");
    }

    #[test]
    fn pascal_case_conversions() {
        let (input, identifier) = Identifier::parse("HelloWorld").unwrap();
        assert_eq!(input, "");
        assert_eq!(identifier.to_camel_case(), "helloWorld");
        assert_eq!(identifier.to_constant_case(), "HELLO_WORLD");
        assert_eq!(identifier.to_flat_case(), "helloworld");
        assert_eq!(identifier.to_kebab_case(), "hello-world");
        assert_eq!(identifier.to_pascal_case(), "HelloWorld");
        assert_eq!(identifier.to_snake_case(), "hello_world");
    }
}
