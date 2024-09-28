
#[grammar{ content }]
pub struct Identifier {
    content: String,
}

//
// Records
//
#[grammar{ Struct | Enum }]
pub enum Record {}

//
// Structure
//

#[grammar{ (identifier ":")? type_name }]
pub struct StructField {
    identifier: Option<String>,
    type_name: String,
}

#[grammar{ "struct" Identifier? "{" (StructField (";" StructField) * ";" ?)? "}" }]
pub struct Struct;

//
// Enum
//

// TODO Rust-Style Enums
#[grammar{ identifier ("=" value) ? }]
pub struct EnumField {
    identifier: String,
    value: usize,
}

#[grammar{ "enum" Identifier? "{"  "}" }]
pub struct Enum;