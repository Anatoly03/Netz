//! This module provides a scanner over the rule AST to detect which
//! fields are of which type.

use super::{IdentifierCounter, Rule};
use std::collections::HashMap;
// use util_cases::CaseStyles;

impl Rule {
    fn qualify_type() {}

    /// Returns a map of all declared fields and their respective
    /// field type as `syn::ItemType`
    pub fn get_fields(&self) -> HashMap<String, IdentifierCounter> {
        // match self {
        //     Rule::Whitespace => HashMap::new(),
        //     Rule::Keyword(_) => HashMap::new(),
        //     Rule::Identifier(s) => vec![(s, IdentifierCounter::Scalar)],
        //     Rule::TypeReference(s) => vec![(s.to_snake_case(), IdentifierCounter::Scalar)]
        //         .into_iter()
        //         .collect(),
        //     Rule::Scope(vec) => todo!(),
        //     Rule::Option(rule) => todo!(),
        //     Rule::Repetition(rule) => todo!(),
        //     Rule::Branch(vec) => todo!(),
        // }

        todo!()
    }
}
