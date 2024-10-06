//! This module contains the ruleset parser. A ruleset is an AST, which represents
//! the grammar inside the macro as a production rule. Consider `[grammar{ "@" ident
//! }]` for any struct `T` as a production of form `<T> -> "@" <Ident>`.

use std::str::FromStr;
use crate::regexp::{Rule, RegexpRange};
use proc_macro::{Delimiter, TokenStream, TokenTree};

fn error(span: proc_macro::Span, msg: impl std::fmt::Display) -> TokenStream {
    syn::Error::new(span.into(), msg).to_compile_error().into()
}

impl From<TokenStream> for Rule {
    fn from(attrs: TokenStream) -> Self {
        let mut vec = Vec::new();
        let mut iter = attrs.clone().into_iter();

        while let Some(attr) = iter.next() {
            let span = attr.span();

            match attr {
                TokenTree::Literal(literal) => {
                    match proc_macro2::Literal::from_str(literal.to_string().as_ref()) {
                        Ok(lit) => match syn::Lit::new(lit) {
                            syn::Lit::Str(lit_str) => {
                                vec.push(Self::Keyword(lit_str.value()));
                            }
                            // syn::Lit::ByteStr(lit_byte_str) => todo!(),
                            // syn::Lit::CStr(lit_cstr) => todo!(),
                            // syn::Lit::Byte(lit_byte) => todo!(),
                            syn::Lit::Char(lit_char) => {
                                vec.push(Self::Keyword(lit_char.value().to_string()));
                            }
                            // syn::Lit::Int(lit_int) => todo!(),
                            // syn::Lit::Float(lit_float) => todo!(),
                            // syn::Lit::Bool(lit_bool) => todo!(),
                            // syn::Lit::Verbatim(literal) => todo!(),
                            other => todo!("unparsable literal: {other:?}"),
                        },
                        Err(e) => {
                            todo!("handle error: {e}")
                        }
                    }
                }
                TokenTree::Punct(punct) => match punct.as_char() {
                    '~' => vec.push(Self::Whitespace),
                    '?' => match vec.pop() {
                        Some(v) => vec.push(Self::Option(Box::new(v))),
                        None => {} // TODO error: `?` was used at the beginning of a scope
                    },
                    '*' => match vec.pop() {
                        Some(v) => vec.push(Self::Repetition(Box::new(v))),
                        None => {} // TODO error: `*` was used at the beginning of a scope
                    },
                    '-' => match (vec.pop(), iter.next()) {
                        (Some(id), Some(tp)) => {
                            // TODO
                        }
                        _ => {} // TODO error: `-` was used at the beginning or end of a scope
                    },
                    // TODO type cast: "a : B"
                    // TODO range: "c - d"
                    _ => {} // TODO error: expected one of `?` or `*`, got `{c}`
                },
                TokenTree::Ident(ident) => {
                    let st = ident.to_string();
                    let mut iter = st.chars();

                    let rl = loop {
                        break match iter.next() {
                            // Identifiers that start with an uppercase letter are type references.
                            Some(c) if c.is_ascii_uppercase() => Self::TypeReference(st),
                            // Identifiers that start with a lowercase letter are field identifiers.
                            Some(c) if c.is_ascii_lowercase() => Self::Identifier(st),
                            // Identifiers that start with `_` inherit the kind from the first letter.
                            Some(_) => continue,
                            // TODO If we reach the end, we should decide if it is a variable or panic.
                            None => Self::Identifier(st),
                        };
                    };

                    vec.push(rl);
                }
                TokenTree::Group(group) => {
                    match group.delimiter() {
                        Delimiter::Parenthesis => {
                            let subcontext = Self::from(group.stream());
                            vec.push(subcontext);
                        }
                        Delimiter::Brace => todo!("expected paranthesis, got (...)"), // TODO proper error message
                        Delimiter::Bracket => {
                            let range = RegexpRange::from(Into::<proc_macro2::TokenStream>::into(group.stream()));
                            vec.push(Self::Range(range))
                        }
                        Delimiter::None => {
                            todo!("expected paranthesis, got delimiter from macro variable")
                        } // TODO
                    }
                }
            }

            // None => break,
        }

        // for attr in attrs.clone().into_iter() {

        // }

        // println!("attr: \"{attrs:?}\"");

        // todo!()
        Self::Scope(vec)

        // compile_error!("Not implemented")
    }
}
