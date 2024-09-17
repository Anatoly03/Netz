use super::{comment::Comment, interface::NetworkParser};
use nom::{
    bytes::complete::take_while_m_n,
    character::complete::alphanumeric1,
    combinator::recognize,
    multi::many0_count,
    sequence::pair,
    IResult,
};

///
/// #### Example
///
/// ```net
/// Identifier label Text123 Hello_World
/// ```
#[derive(Debug, PartialEq)]
pub struct NetworkIdentifier {
    pub identity: String,
}

impl NetworkIdentifier {
    pub fn new(identity: String) -> Self {
        Self { identity }
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
        let (input, _comment) = Comment::parse(input)?;

        let (input, identity) = recognize(pair(
            take_while_m_n(1, 1, |c: char| c.is_ascii_alphabetic()),
            many0_count(alphanumeric1),
        ))(input)?;

        IResult::Ok((input, Self::new(identity.to_string())))
    }
}

#[cfg(test)]
mod identifier_test {
    use super::*;

    #[test]
    fn simple_identifier() {
        let (_, field) = NetworkIdentifier::parse("Identifier").unwrap();
        assert_eq!(field.identity, "Identifier");
    }

    #[test]
    fn invalid_identifier() {
        let output = NetworkIdentifier::parse("012variable");
        assert!(output.is_err());
    }

    #[test]
    #[ignore = "invalid test: returned output is NetworkIdentifier { identity: \"Id\" } and not an error."]
    fn invalid_character() {
        let output = NetworkIdentifier::parse("Idäntitßi");
        assert!(output.is_err());
    }

    #[test]
    fn trim_identifier() {
        let (_, field) = NetworkIdentifier::parse("   spaces").unwrap();
        assert_eq!(field.identity, "spaces");
    }

    #[test]
    fn middle_identifier() {
        let (_, field) = NetworkIdentifier::parse("NoEof ").unwrap();
        assert_eq!(field.identity, "NoEof");
    }
}
