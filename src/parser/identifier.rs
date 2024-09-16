use super::interface::NetworkParser;
use nom::{
    bytes::complete::take_while_m_n,
    character::complete::alphanumeric1,
    combinator::{all_consuming, recognize, verify},
    multi::many0_count,
    sequence::pair,
    IResult,
};

#[derive(Debug, PartialEq)]
pub struct NetworkIdentifier {
    pub identity: String,
}

impl NetworkIdentifier {
    pub fn new(identity: String) -> Self {
        NetworkIdentifier { identity }
    }
}

impl NetworkParser for NetworkIdentifier {
    /// Read in an identifier. An identifier is a pair of the first character
    /// combined with an optional rest of the identifier. The first part is
    /// expected to be a letter or the underscore and the rest includes numbers,
    /// too.
    ///
    /// @reference https://stackoverflow.com/a/61329008/16002144
    fn parse(input: &str) -> IResult<&str, Self> {
        recognize(all_consuming(pair(
            take_while_m_n(1, 1, |c: char| c.is_ascii_alphabetic()),
            many0_count(alphanumeric1),
        )))(input)
        .map(|(input, identifier)| {
            (
                input,
                Self {
                    identity: identifier.to_string(),
                },
            )
        })
    }
}

#[cfg(test)]
mod identifier_test {
    // use super::*;

    #[test]
    fn simple_field() {
        let (_, field) = NetworkIdentifier::parse("FieldType simple;").unwrap();
        assert_eq!(field.field_type, "FieldType");
        assert_eq!(field.identifier, "simple");
    }

    #[test]
    fn many_spaces() {
        let (_, field) = NetworkIdentifier::parse("   string   name   ;   ").unwrap();
        assert_eq!(field.field_type, "string");
        assert_eq!(field.identifier, "name");
    }
}
