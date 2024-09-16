use nom::{bytes::complete::tag, multi::many0, IResult};

use super::{
    comment::Comment, field::StructField, identifier::NetworkIdentifier, interface::NetworkParser, tag::Tag,
};

#[derive(Debug, PartialEq)]
pub struct NetworkStruct {
    identity: String,
    fields: Vec<StructField>,
    tags: Vec<Tag>,
}

// impl NetworkStruct {
//     pub fn new(identity: String) -> Self {
//         Self {
//             identity,
//             fields: vec![],
//         }
//     }
// }

impl NetworkParser for NetworkStruct {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, _comment) = Comment::parse(input)?;

        // read optionally several tags
        let (input, tags) = many0(Tag::parse)(input)?;

        // read the 'struct' keyword
        let (input, _) = Comment::parse(input)?;
        let (input, _) = tag("struct")(input)?;

        // expect structure name
        let (input, struct_name) = NetworkIdentifier::parse(input)?;

        // expect '{' symbol
        let (input, _) = Comment::parse(input)?;
        let (input, _) = tag("{")(input)?;

        // expect field declarations
        let (input, fields) = many0(StructField::parse)(input)?;

        let (input, _) = Comment::parse(input)?;
        let (input, _) = tag("}")(input)?;

        Ok((
            input,
            Self {
                identity: struct_name.identity,
                fields,
                tags
            },
        ))
    }
}

#[cfg(test)]
mod struct_test {
    use super::*;

    /// Tests a simple structure without declared fields.
    #[test]
    fn simple_struct() {
        let (input, network_struct) = NetworkStruct::parse("struct Struct {}").unwrap();
        assert_eq!(network_struct.identity, "Struct");
    }

    /// Tests a simple structure with two declared fields of types
    /// `Foo` and `Bar` with the respective field names `foo` and `bar`
    #[test]
    fn foobar_struct() {
        let (input, network_struct) =
            NetworkStruct::parse("struct FooBar { Foo foo; Bar bar; }").unwrap();
        assert_eq!(input, "");
        assert_eq!(network_struct.identity, "FooBar");
        assert_eq!(network_struct.fields.len(), 2);
        assert_eq!(network_struct.fields.first().unwrap().name(), "foo");
        assert_eq!(network_struct.fields.first().unwrap().field_type(), "Foo");
        assert_eq!(network_struct.fields.get(1).unwrap().name(), "bar");
        assert_eq!(network_struct.fields.get(1).unwrap().field_type(), "Bar");
    }

    /// Tests a more complex structure with two annotated tags
    #[test]
    fn annotated_struct() {
        let (input, network_struct) =
            NetworkStruct::parse("@special @annotation struct FooBar { @deprecated Foo foo; }").unwrap();
        assert_eq!(input, "");

        assert_eq!(network_struct.fields.len(), 1);
        assert_eq!(network_struct.fields.first().unwrap().name(), "foo");
        assert_eq!(network_struct.fields.first().unwrap().field_type(), "Foo");
        assert_eq!(network_struct.fields.first().unwrap().tags().next().unwrap().name(), "deprecated");

        assert_eq!(network_struct.tags.len(), 2);
        assert_eq!(network_struct.tags.first().unwrap().name(), "special");
    }
}
