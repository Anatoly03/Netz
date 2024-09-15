struct NetworkFileReader {
    primitives: Vec<String>,
}

static PRIMITIVE_TYPES: &'static [&'static str] = &[
    "u8", "i8", "u16", "i16", "u32", "i32", "u64", "i64", "bool", "string",
];

impl NetworkFileReader {
    pub fn new() -> Self {
        Self {
            primitives: PRIMITIVE_TYPES.iter().map(|str| str.to_string()).collect(),
        }
    }
}
