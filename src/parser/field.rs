use super::{
    comment::Comment,
    identifier::NetworkIdentifier,
    interface::NetworkParser,
};
use nom::{bytes::complete::tag, multi::many0_count, IResult};

/// A struct field is an entry in a structure. It contains
/// the type and field identifier, and the dimension of the
/// array, 0 if none.
#[derive(Debug, PartialEq)]
pub struct StructField {
    field_type: String,
    field_name: String,
    array_dimension: usize,
}

// impl StructField {
//     pub fn new(field_type: String, field_name: String) -> Self {
//         StructField {
//             field_type,
//             field_name,
//             array_dimension: 0,
//         }
//     }

//     pub fn name(&self) -> &str {
//         self.field_name.as_str()
//     }

//     pub fn field_type(&self) -> &str {
//         self.field_type.as_str()
//     }
// }

impl NetworkParser for StructField {
    fn parse(input: &str) -> IResult<&str, Self> {
        // read the field type
        let (input, field_type) = NetworkIdentifier::parse(input)?;

        //
        let (input, _) = Comment::parse(input)?;
        let (input, array_dimension) = many0_count(tag("[]"))(input)?;

        // read the field name
        let (input, field_name) = NetworkIdentifier::parse(input)?;
        let (input, _) = Comment::parse(input)?;
        let (input, _) = tag(";")(input)?;

        IResult::Ok((input, StructField {
            field_type: field_type.identity,
            field_name: field_name.identity,
            array_dimension,
        }))
    }
}

#[cfg(test)]
mod field_test {
    use super::*;

    #[test]
    fn simple_field() {
        let (_, field) = StructField::parse("Field field;").unwrap();
        assert_eq!(field.field_type, "Field");
        assert_eq!(field.field_name, "field");
        assert_eq!(field.array_dimension, 0);
    }

    #[test]
    fn many_spaces() {
        let (_, field) = StructField::parse("   string   name   ;   ").unwrap();
        assert_eq!(field.field_type, "string");
        assert_eq!(field.field_name, "name");
    }

    #[ignore = "comments are not supported."]
    #[test]
    fn documented_field() {
        let (_, field) = StructField::parse("/* Good documentation. */ Field field;").unwrap();
        assert_eq!(field.field_type, "Field");
        assert_eq!(field.field_name, "field");
    }

    #[test]
    fn field_array() {
        let (_, field) = StructField::parse("string[] name;").unwrap();
        assert_eq!(field.field_type, "string");
        assert_eq!(field.field_name, "name");
        assert_eq!(field.array_dimension, 1);
    }

    #[test]
    fn field_array_two_dimensional() {
        let (_, field) = StructField::parse("string[][] name;").unwrap();
        assert_eq!(field.field_type, "string");
        assert_eq!(field.field_name, "name");
        assert_eq!(field.array_dimension, 2);
    }
}
