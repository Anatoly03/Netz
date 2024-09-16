use nom::{
    bytes::complete::{tag, take, take_while, take_while_m_n}, error::{Error, ErrorKind}, multi::many0, Err, IResult
};

use super::{
    comment::NetworkComment, field::NetworkField, general::{expect_space, trim}, identifier::NetworkIdentifier, interface::NetworkParser
};

#[derive(Debug, PartialEq)]
pub struct NetworkStruct {
    identity: String,
    fields: Vec<NetworkField>,
}

impl NetworkStruct {
    pub fn new(identity: String) -> Self {
        NetworkStruct {
            identity,
            fields: vec![],
        }
    }
}

impl NetworkParser for NetworkStruct {
    fn parse(input: &str) -> IResult<&str, NetworkStruct> {
        let (input, comment) = NetworkComment::parse(input)?;
        let (input, _) = tag("struct")(input)?;

        // expect structure name
        let (input, struct_name) = NetworkIdentifier::parse(input)?;
        let mut network_struct = NetworkStruct::new(struct_name.identity.to_owned());

        // expect '{' symbol
        let (input, _) = NetworkComment::parse(input)?;
        let (input, _) = tag("{")(input)?;

        // expect field declarations
        // let (input, fields) = many0(|input| NetworkField::parse(input))?;

        // while let (input, Some(field)) = opt(NetworkField)(input)? {
        //     let (input, _) = trim(input)?;
        // };

        let (input, _) = NetworkComment::parse(input)?;
        let (input, _) = tag("}")(input)?;
        Ok((input, network_struct))
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
    /// `Foo` and `Bar`
    #[test]
    #[ignore = "structs do not support fields"]
    fn foobar_struct() {
        let (input, network_struct) = NetworkStruct::parse("struct FooBar { Foo foo; Bar bar; }").unwrap();
        assert_eq!(network_struct.identity, "FooBar");
        assert_eq!(network_struct.fields.len(), 2);
    }
}
