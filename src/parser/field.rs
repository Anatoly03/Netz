use super::{general::{expect_space, trim}, identifier::NetworkIdentifier, interface::NetworkParser};
use nom::{bytes::complete::tag, IResult};

/// A network field is an entry 
#[derive(Debug, PartialEq)]
pub struct NetworkField {
    field_type: String,
    field_name: String,
}

impl NetworkField {
    pub fn new(field_type: String, field_name: String) -> Self {
        NetworkField {
            field_type,
            field_name,
        }
    }
}

impl NetworkParser for NetworkField {
    fn parse(input: &str) -> IResult<&str, NetworkField> {
        let (input, _) = trim(input)?;
        let (input, field_type) = NetworkIdentifier::parse(input)?;
        let (input, _) = expect_space(input)?;
        let (input, field_name) = NetworkIdentifier::parse(input)?;
        let (input, _) = trim(input)?;
        let (input, _) = tag(";")(input)?;
        IResult::Ok((
            input,
            Self::new(field_type.identity, field_name.identity),
        ))
    }
}

#[cfg(test)]
mod field_test {
    use super::*;

    #[test]
    fn simple_field() {
        let (_, field) = NetworkField::parse("FieldType simple;").unwrap();
        assert_eq!(field.field_type, "FieldType");
        assert_eq!(field.field_name, "simple");
    }

    #[test]
    fn many_spaces() {
        let (_, field) = NetworkField::parse("   string   name   ;   ").unwrap();
        assert_eq!(field.field_type, "string");
        assert_eq!(field.field_name, "name");
    }
}
