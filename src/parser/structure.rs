use nom::{
    bytes::complete::{tag, take, take_while, take_while_m_n}, error::{Error, ErrorKind}, multi::many0, Err, IResult
};

use super::{
    field::NetworkField,
    general::{expect_space, identifier, trim},
};

#[derive(Debug, PartialEq)]
pub struct NetworkStruct {
    identifier: String,
    fields: Vec<NetworkField>,
}

impl NetworkStruct {
    pub fn new(identifier: String) -> Self {
        NetworkStruct {
            identifier,
            fields: vec![],
        }
    }

    pub fn parse(input: &str) -> IResult<&str, NetworkStruct> {
        // expect 'struct' keyword
        let (input, _) = trim(input)?;
        let (input, _) = tag("struct")(input)?;

        // expect structure name
        let (input, _) = expect_space(input)?;
        let (input, struct_name) = identifier(input)?;
        let mut network_struct = NetworkStruct::new(struct_name.to_owned());

        // expect '{' symbol
        let (input, _) = trim(input)?;
        let (input, _) = tag("{")(input)?;

        // expect field declarations
        let (input, fields) = many0(|input| NetworkField::parse(input))?;

        while let (input, Some(field)) = opt(NetworkField)(input)? {
            let (input, _) = trim(input)?;
        };

        let (input, _) = trim(input)?;
        let (input, _) = tag("}")(input)?;
        Ok((input, network_struct))
    }
}
