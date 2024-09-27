//!

use nom::{
    self, bytes::complete::is_a, character::complete::alphanumeric1, combinator::recognize,
    multi::many0_count, sequence::pair, Parser,
};

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

/// Generate different case variations for the identifier
impl Identifier {
    /// Converts the identifier to flatcase (`flatcase`).
    pub fn flat_case(&self) -> String {
        self.0.join("").to_lowercase()
    }

    /// Converts the identifier to kebab case (`dash-case`).
    pub fn kebab_case(&self) -> String {
        self.0.join("-").to_lowercase()
    }

    /// Converts the identifier to camel case (`camelCase`).
    pub fn camel_case(&self) -> String {
        let mut s = self.pascal_case();
        s[0..1].make_ascii_lowercase();
        s
    }

    /// Converts the identifier to pascal case (`PascalCase`, `CapitalCamelCase`).
    pub fn pascal_case(&self) -> String {
        let capitalize = |s: &String| {
            let mut out = s.to_lowercase().clone();
            out[0..1].make_ascii_uppercase();
            out
        };

        (&self.0)
            .into_iter()
            .map(capitalize)
            .collect::<Vec<String>>()
            .join("")
    }

    /// Converts the identifier to snake case (`snake_case`).
    pub fn snake_case(&self) -> String {
        self.0.join("_").to_lowercase()
    }

    /// Converts the identifier to constant case (`UPPER_CASE`).
    pub fn constant_case(&self) -> String {
        self.0.join("_").to_uppercase()
    }
}

impl<'a, E: nom::error::ParseError<&'a str>> Parser<&'a str, Self, E> for Identifier {
    fn parse(&mut self, input: &'a str) -> nom::IResult<&'a str, Self, E> {
        // TODO let (input, _comment) = Comment::parse(input)?;

        let (input, identity) = recognize(pair(
            is_a("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_"),
            many0_count(alphanumeric1),
        ))(input)?;

        // TODO use identity to fill in

        nom::IResult::Ok((input, self.clone()))
    }
}

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
        assert_eq!(identifier.camel_case(), "helloWorld");
        assert_eq!(identifier.constant_case(), "HELLO_WORLD");
        assert_eq!(identifier.flat_case(), "helloworld");
        assert_eq!(identifier.kebab_case(), "hello-world");
        assert_eq!(identifier.pascal_case(), "HelloWorld");
        assert_eq!(identifier.snake_case(), "hello_world");
    }
}
