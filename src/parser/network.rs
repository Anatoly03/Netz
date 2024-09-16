
/// The `NetworkFileReader` defines a grammar state for a network file.
#[derive(Debug, PartialEq)]
pub struct NetworkFileReader {
    primitive_types: Vec<String>,
}

/// Parse a structure definition.
///
/// #### Example
///
/// ```net
/// struct FooBar {
///     u8 foo;
///     string bar;
/// }
/// ```
fn parse_struct(input: &str) -> IResult<&str, NetworkStruct> {
    let (input, _) = tag("struct")(input)?;
    let (input, _) = expect_space(input)?;
    let (input, struct_name) = identifier(input)?;
    let (input, _) = trim(input)?;
    let (input, _) = tag("{")(input)?;
    let (input, _) = trim(input)?;
    let (input, _) = tag("}")(input)?;
    Ok((input, NetworkStruct {}))
}

impl NetworkFileReader {
    pub fn new() -> Self {
        Self {
            primitive_types: PRIMITIVE_TYPES.iter().map(|str| str.to_string()).collect(),
        }
    }

    /// Register a new primitive type. The list of primitive types
    /// will be checked by the `nom` parser module. Defining custom
    /// primitives is useful for extrenally defined structures and
    /// classes.
    ///
    /// #### Example
    ///
    /// ```
    /// let mut networkFileReader = NetworkFileReader::new();
    /// networkFileReader.register_primitive("");
    /// ```
    pub fn register_primitive(&mut self, value: &str) {
        self.primitive_types.push(value.to_string());
    }

    /// Get a copy of the list of registered primitives.
    pub fn primitives(&self) -> Vec<String> {
        self.primitive_types.clone()
    }

    // /// Entry point to copy a state from a file's contents.
    // pub fn parse(&self, input: &str) -> IResult<&str, String> {
    //     let (input, struc) = parse_struct(input)?;
    //     Ok(("", String::from("")))
    // }
}
