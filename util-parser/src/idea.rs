
#[grammar{ ( '_'* 'a'-'z' ('a'-'Z' | '_') * ) : String }]
pub struct Identifier;

#[grammar{ ( '0'-'9' + ) : usize }]
pub struct Integer;

//
// Records
//
#[grammar{ Struct | Enum }]
pub enum Record {}

//
// Structure
//

#[grammar{ (Identifier ":")? type_name:Identifier }]
pub struct StructField;
// {
//     identifier: Option<String>,
//     type_name: String,
// }

#[grammar{ "struct" Identifier? "{" (StructField (";" StructField) * ";" ?)? "}" }]
pub struct Struct;

//
// Enum
//

// TODO Rust-Style Enums
#[grammar{ Identifier ("=" value:usize) ? }]
pub struct EnumField;
// {
//     identifier: String,
//     value: usize,
// }

#[grammar{ "enum" Identifier? "{"  "}" }]
pub struct Enum;