mod from;
mod into;

#[derive(Debug)]
pub enum Rule {
    Keyword(String), // "strings"
    Identifier(String), // snake_case
    TypeReference(String), // PascalCase
    Scope(Vec<Rule>), // concatenation
    Option(Box<Rule>), // (option) ?
    Repetition(Box<Rule>), // () *
    Branch(Vec<Rule>), // ... | ...
}
