use std::slice::Iter;

use super::{comment::Comment, identifier::NetworkIdentifier, interface::NetworkParser, tag::Tag};
use nom::{
    bytes::complete::tag,
    multi::{many0, many0_count},
    IResult,
};

/// A struct field is an entry in a structure. It contains
/// the type and field identifier, and the dimension of the
/// array, 0 if none.
///
/// #### Example
///
/// ```net
/// @optional FieldType field;
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct StructField {
    field_type: String,
    field_name: String,
    array_dimension: usize,
    tags: Vec<Tag>,
}

impl StructField {
    // pub fn new(field_type: String, field_name: String) -> Self {
    //     StructField {
    //         field_type,
    //         field_name,
    //         array_dimension: 0,
    //     }
    // }

    pub fn name(&self) -> &str {
        self.field_name.as_str()
    }

    pub fn field_type(&self) -> &str {
        &self.field_type.as_str()
    }

    pub fn tags(&self) -> Iter<'_, Tag> {
        self.tags.iter()
    }
}

impl NetworkParser for StructField {
    fn parse(input: &str) -> IResult<&str, Self> {
        // read optionally several tags
        let (input, tags) = many0(Tag::parse)(input)?;

        // read the field type
        let (input, field_type) = NetworkIdentifier::parse(input)?;

        //
        let (input, _) = Comment::parse(input)?;
        let (input, array_dimension) = many0_count(tag("[]"))(input)?;

        // read the field name
        let (input, field_name) = NetworkIdentifier::parse(input)?;
        let (input, _) = Comment::parse(input)?;
        let (input, _) = tag(";")(input)?;

        IResult::Ok((
            input,
            StructField {
                field_type: field_type.identity,
                field_name: field_name.identity,
                array_dimension,
                tags,
            },
        ))
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
        assert_eq!(field.tags.len(), 0);
    }

    #[test]
    fn many_spaces() {
        let (_, field) = StructField::parse("   string   name   ;   ").unwrap();
        assert_eq!(field.field_type, "string");
        assert_eq!(field.field_name, "name");
    }

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

    #[test]
    fn deprecated_field() {
        let (_, field) = StructField::parse("@deprecated A b;").unwrap();
        assert_eq!(field.field_type, "A");
        assert_eq!(field.field_name, "b");
        assert_eq!(field.tags.len(), 1);
        assert_eq!(field.tags.first().unwrap().name(), "deprecated");
    }
}
