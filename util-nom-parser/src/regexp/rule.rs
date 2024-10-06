
use super::RegexpRange;

/// A Rule is an AST of a regular expression, which is
/// declared in the scope of the macro as an attribute.
///
/// ```rs
/// #[grammar{ RULE }] // Here the production `<STRUCT> -> RULE` is declared
/// struct STRUCT;
/// ```
#[derive(Debug, PartialEq)]
pub enum Rule {
    /// A sequence of whitespace characters is declared
    /// with `~`, and a sequence of optional whitespace
    /// characters with `~?`. The first whitespace
    /// character will trim the input whitespace, so two
    /// tildas in a row `~~` will never consume.
    Whitespace,
    /// A keyword (or a symbol) is declared by defining
    /// the required characters (or symbols) in `"quotes"`.
    /// The parser will peek if the input starts with the
    /// keyword and consume it or abort.
    Keyword(String),
    /// TODO
    Identifier(String), // snake_case
    /// TODO
    TypeReference(String), // PascalCase
    /// TODO
    Scope(Vec<Rule>), // concatenation
    /// TODO
    Option(Box<Rule>), // (option) ?
    /// TODO
    Repetition(Box<Rule>), // () *
    /// TODO
    Range(RegexpRange), // range - range
    /// TODO
    Branch(Vec<Rule>), // ... | ...
}
