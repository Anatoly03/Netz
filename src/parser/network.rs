use super::{interface::NetworkParser, structure::NetworkStruct, PRIMITIVE_TYPES};
use crate::parser::comment::Comment;
use nom::{branch::Alt, combinator::eof, multi::many0, sequence::pair, IResult};

/// The `NetworkFileReader` defines a grammar state for a network file.
#[derive(Debug, PartialEq)]
pub struct NetworkFileReader {
    primitive_types: Vec<String>,
    structures: Vec<NetworkStruct>,
}

// #[derive(Debug, PartialEq)]
// pub enum NetworkElement {
//     NStruct(NetworkStruct),
// }

impl NetworkFileReader {
    // pub fn new() -> Self {
    //     Self {
    //         primitive_types: PRIMITIVE_TYPES.iter().map(|str| str.to_string()).collect(),
    //         structures: vec![],
    //     }
    // }

    pub fn get_default_primitives() -> Vec<String> {
        PRIMITIVE_TYPES.iter().map(|str| str.to_string()).collect()
    }

    /// Register a new primitive type. The list of primitive types
    /// will be checked by the `nom` parser module. Defining custom
    /// primitives is useful for extrenally defined structures and
    /// classes.
    pub fn register_primitive(&mut self, value: &str) {
        self.primitive_types.push(value.to_string());
    }

    /// Get a copy of the list of registered primitives.
    pub fn primitives(&self) -> Vec<String> {
        self.primitive_types.clone()
    }

    /// Add a new structure
    pub fn register_structure(&mut self, value: NetworkStruct) {
        self.structures.push(value);
    }

    /// Read a structure from the parser contents.
    pub fn structure(&self, name: &str) -> Option<NetworkStruct> {
        self.structures
            .clone()
            .into_iter()
            .find(|structure| structure.name() == name)
    }
}

impl NetworkParser for NetworkFileReader {
    fn parse(input: &str) -> IResult<&str, Self> {
        // read multiple structures
        // TODO add other definition types
        let (input, structures) = many0(NetworkStruct::parse)(input)?;

        // parse and ignore any trailing whitespace
        let (input, _) = pair(Comment::parse, eof)(input)?;

        IResult::Ok((input, NetworkFileReader {
            primitive_types: Self::get_default_primitives(),
            structures,
        }))
    }
}

/// The test module `network_test` tests the entry point of
/// the network file parser.
///
/// ### Contributing Tests
///
/// - At the beginning of the test parse a string with `NetworkFileReader`
/// and unwrap it or assert an error. Longer strings should be defined as
/// a variable.
/// - Since all files have to be consumed, it's expected for
/// every test to include `assert_eq!(input, "");` to make sure,
/// that the entire input was read.
#[cfg(test)]
mod network_test {
    use super::*;

    /// A file defining a very simple structure.
    #[test]
    fn simple_struct() {
        let (input, network_struct) =
            NetworkFileReader::parse("\nstruct Struct {\n\tu8 id;\n}\n").unwrap();
        assert_eq!(input, "");
        assert_eq!(network_struct.structure("NonExistingStructure"), None);
        assert_eq!(network_struct.structure("Struct").unwrap().name(), "Struct");
        assert_eq!(network_struct.structure("Struct").unwrap().fields().len(), 1);
        assert_eq!(network_struct.structure("Struct").unwrap().fields().next().unwrap().name(), "id");
    }

    /// The empty file should not return in an error.
    #[test]
    fn empty_file() {
        let (input, network_struct) = NetworkFileReader::parse("").unwrap();
        assert_eq!(input, "");
    }

    /// A file filled with spaces should not return in an error.
    #[test]
    fn spacious_file() {
        let (input, network_struct) = NetworkFileReader::parse(" \r \t \n ").unwrap();
        assert_eq!(input, "");
    }

    /// A structure with a comment should have the message as a part
    /// of the documentation.
    #[test]
    #[ignore = "comments are not supported."]
    fn commented_struct() {
        let (input, network_struct) =
            NetworkFileReader::parse("/// This is a very simple structure.\nstruct Struct {}\n")
                .unwrap();
        assert_eq!(input, "");
    }
}
