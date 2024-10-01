//! TODO

mod count;
mod from;
mod into;

#[derive(Debug)]
pub enum Rule {
    Whitespace,            // ~
    Keyword(String),       // "strings"
    Identifier(String),    // snake_case
    TypeReference(String), // PascalCase
    Scope(Vec<Rule>),      // concatenation
    Option(Box<Rule>),     // (option) ?
    Repetition(Box<Rule>), // () *
    Branch(Vec<Rule>),     // ... | ...
}

#[derive(Debug)]
pub enum IdentifierCounter {
    // None,// default
    Scalar, // defined
    Option, // optionally defined
    Many,   // multiple defined
}
