//! This module provides a scanner over the rule AST to detect which
//! fields are of which type.

use super::{Dimension, Rule};
use std::collections::HashMap;
use util_cases::CaseStyles;

impl Rule {
    fn qualify_type() {}

    /// Returns a map of all declared fields and their respective
    /// field type as `syn::ItemType`
    pub fn get_fields(&self, map: &mut HashMap<String, Dimension>) {
        match self {
            Rule::Identifier(ident) => {
                let ident = &ident.to_snake_case();
                match map.get(ident) {
                    Some(Dimension::Scalar) => {
                        map.insert(ident.clone(), Dimension::Many);
                    },
                    Some(Dimension::Option) => todo!(),
                    Some(Dimension::Many) => {},
                    None => {
                        map.insert(ident.clone(), Dimension::Scalar);
                    },
                }
            }

            // TODO rules

            // Rule::TypeReference(s) => vec![(s.to_snake_case(), IdentifierCounter::Scalar)]
            //     .into_iter()
            //     .collect(),
            // Scopes just operate on the hashmap in sequence.
            Rule::Scope(rules) => {
                for rl in rules.iter() {
                    rl.get_fields(map);
                }
            }

            // TODO add proper hashmap scanner
            Rule::Branch(vec) => todo!(),

            // Nested rules just pass the hashmap and other ignore.
            Rule::Option(rule) | Rule::Repetition(rule) => rule.get_fields(map),
            _ => (),
        }
    }
}
