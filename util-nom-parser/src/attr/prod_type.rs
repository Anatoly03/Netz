//! This module contains the production type definition and implementations.
//! A production type is the interpreted type of a field in a 

#[derive(Debug, PartialEq)]
pub enum ProductionType {
    /// The production type `Bool` is a binary value with `true` indicating
    /// presence of the token and `false` indicating its' absence. This type
    /// should be used when the only question is if a grammar part was read or
    /// not, for example modifiers like `pub` and `mut`.
    /// 
    /// ```rs
    /// #[grammar{ is_public : ("pub") ? ~ is_mutable : ("mut") ? }] // pub? mut?
    /// struct Modifiers {
    ///     is_public: bool,
    ///     is_mutable: bool,
    /// }
    /// 
    /// // Order of modifiers doesn't matter.
    /// #[grammar{ ( is_public : ("pub") | is_mutable : ("mut") )* }] // (pub | mut) *
    /// ```
    Bool,

    /// Production Type `String` specifies that only the character sequence of
    /// an element matters.
    /// 
    /// This is the default type for all identifiers.
    String,

    /// Production Type `Int` specifies that a number will be read.
    Int(bool, u8),

    /// Production Type `Float` specifies that a number will be read.
    Float(u8),
}

impl ToString for ProductionType {
    /// Convert the production type into a Rust-syntax type. This method should
    /// be used to convert the the production type.
    fn to_string(&self) -> String {
        match self {
            Self::Bool => "bool".into(),
            Self::String => "String".into(),
            Self::Int(signed, bytes) => [
                if *signed { "i" } else { "u" },
                (bytes * 8).to_string().as_str(),
            ]
            .concat(),
            Self::Float(bytes) => ["f", (bytes * 8).to_string().as_str()].concat(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coverage() {
        // Boolean and string
        assert_eq!(ProductionType::Bool.to_string().as_str(), "bool");
        assert_eq!(ProductionType::String.to_string().as_str(), "String");

        // Integers
        assert_eq!(ProductionType::Int(true, 1).to_string().as_str(), "i8");
        assert_eq!(ProductionType::Int(false, 1).to_string().as_str(), "u8");
        assert_eq!(ProductionType::Int(true, 2).to_string().as_str(), "i16");
        assert_eq!(ProductionType::Int(false, 2).to_string().as_str(), "u16");
        assert_eq!(ProductionType::Int(true, 4).to_string().as_str(), "i32");
        assert_eq!(ProductionType::Int(false, 4).to_string().as_str(), "u32");
        assert_eq!(ProductionType::Int(true, 8).to_string().as_str(), "i64");
        assert_eq!(ProductionType::Int(false, 8).to_string().as_str(), "u64");

        // Floats
        assert_eq!(ProductionType::Float(4).to_string().as_str(), "f32");
        assert_eq!(ProductionType::Float(8).to_string().as_str(), "f64");
    }
}