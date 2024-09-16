pub mod comment;
pub mod field;
pub mod identifier;
pub mod interface;
pub mod network;
pub mod structure;
pub mod tag;

/// The primitive types that the network file accepts per default.
/// This includes integers, a boolean and string value.
static PRIMITIVE_TYPES: &'static [&'static str] = &[
    "u8", "i8", "u16", "i16", "u32", "i32", "u64", "i64", "bool", "string",
];

// /// The reserved identifiers are custom keywords that cannot be
// /// chosen for field, type and name identifiers. 
// static RESERVED_IDENTIFIERS: &'static [&'static str] = &[
//     "struct", "enum", "service", "message", "union", "const", "option",
// ];
