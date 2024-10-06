//! The range module provides a subset of UNIX-stylish regular expressions,
//! which are surrounded by brackets. These expression parts only affect
//! character ranges and no sequencing of those.

use proc_macro2::{Literal, TokenStream, TokenTree};
use syn::Lit;

/// A regular expression range, surrounded by brackets, is a part of
/// the regular expression, which defines a set of valid characters,
/// and removes all rules of sequencing.
///
/// ```rs
/// // Allows zero or more lowercase ascii characters.
/// #[grammar( ['a' - 'z']* )]
/// // Allows parsing variable identifiers which start with a lowercase letter.
/// #[grammar( ['a' - 'z'] ["aA0" - "zZ9" | "_"]* )]
/// ```
#[derive(Debug, PartialEq)]
pub struct RegexpRange {
    ranges: Vec<(char, char)>,
    select: String,
}

impl RegexpRange {
    pub fn new() -> Self {
        Self {
            ranges: Vec::new(),
            select: String::new(),
        }
    }
}

fn literal_to_string(literal: Literal) -> String {
    match Lit::new(literal) {
        Lit::Str(lit_str) => lit_str.value(),
        // TODO Lit::ByteStr(lit_byte_str) => todo!(),
        // TODO Lit::CStr(lit_cstr) => todo!(),
        // TODO Lit::Byte(lit_byte) => todo!(),
        Lit::Char(lit_char) => lit_char.value().to_string(),
        // TODO Lit::Int(lit_int) => todo!(),
        // TODO Lit::Float(lit_float) => todo!(),
        // TODO Lit::Bool(lit_bool) => todo!(),
        // TODO Lit::Verbatim(literal) => todo!(),
        _ => todo!(),
    }
}

impl From<TokenStream> for RegexpRange {
    fn from(attrs: TokenStream) -> Self {
        let mut range = Self::new();
        let mut iter = attrs.clone().into_iter();
        let mut buffer: Option<String> = None;

        while let Some(attr) = iter.next() {
            match attr {
                TokenTree::Literal(literal) => {
                    if let Some(s) = &buffer {
                        range.select += s.as_ref();
                    }
                    buffer = Some(literal_to_string(literal));
                }
                TokenTree::Punct(punct) => match punct.as_char() {
                    '|' => match &buffer {
                        Some(s) => {
                            range.select += s.as_ref();
                            buffer = None;
                        }
                        None => {} // do nothing
                    },
                    '-' => match (&buffer, iter.next()) {
                        (Some(from), Some(TokenTree::Literal(literal))) => {
                            let to = literal_to_string(literal);

                            // TODO proper error message
                            // assert_eq!(from.len(), to.len());

                            for (fr, t) in from.chars().zip(to.chars()) {
                                range.ranges.push((fr, t));
                            }
                        }
                        _ => {} // TODO error: `-` was used at the beginning or end of a scope or the separator `|`
                    },
                    chr => panic!("expected `|` or `-`, got {chr}")
                },
                TokenTree::Ident(ident) => {
                    match ident.to_string().as_str() {
                        "NUM" => range.ranges.push(('0', '9')),
                        "LOWER" => range.ranges.push(('a', 'z')),
                        "UPPER" => range.ranges.push(('A', 'Z')),
                        "ALPHA" => range.ranges.push(('a', 'Z')),
                        "SPACE" => range.select += "\r\n\t ",
                        id => panic!("unknown identifier in regexp range pattern: {id}")
                    }
                }
                TokenTree::Group(group) => {
                    panic!(
                        "a {:?} group expression cannot be inside a regexp range",
                        group.delimiter()
                    )
                }
            }

            // None => break,
        }

        range
    }
}

// impl Into<TokenStream> for RegexpRange {
//     fn into(self) -> TokenStream {
//         todo!()
//     }
// }