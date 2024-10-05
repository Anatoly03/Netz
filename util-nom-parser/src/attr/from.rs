//! This module contains the ruleset parser. A ruleset is an AST, which represents
//! the grammar inside the macro as a production rule. Consider `[grammar{ "@" ident
//! }]` for any struct `T` as a production of form `<T> -> "@" <Ident>`.

use std::str::FromStr;

use super::Rule;
use proc_macro::{Delimiter, TokenStream, TokenTree};

fn error(span: proc_macro::Span, msg: impl std::fmt::Display) -> TokenStream {
    syn::Error::new(span.into(), msg).to_compile_error().into()
}

impl From<TokenStream> for Rule {
    fn from(attrs: TokenStream) -> Self {
        let mut vec = Vec::new();

        for attr in attrs.clone().into_iter() {
            let span = attr.span();

            match attr {
                TokenTree::Literal(literal) => {
                    match proc_macro2::Literal::from_str(literal.to_string().as_ref()) {
                        Ok(lit) => match syn::Lit::new(lit) {
                            syn::Lit::Str(lit_str) => {
                                vec.push(Rule::Keyword(lit_str.value()));
                            }
                            // syn::Lit::ByteStr(lit_byte_str) => todo!(),
                            // syn::Lit::CStr(lit_cstr) => todo!(),
                            // syn::Lit::Byte(lit_byte) => todo!(),
                            // syn::Lit::Char(lit_char) => todo!(),
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
                    '~' => vec.push(Rule::Whitespace),
                    '?' => match vec.pop() {
                        Some(v) => vec.push(Rule::Option(Box::new(v))),
                        None => {}, // TODO error: `?` was used at the beginning of a scope
                    }
                    '*' => match vec.pop() {
                        Some(v) => vec.push(Rule::Repetition(Box::new(v))),
                        None => {}, // TODO error: `*` was used at the beginning of a scope
                    }
                    _ => {}, // TODO error: expected one of `?` or `*`, got `{c}`
                },
                TokenTree::Ident(ident) => {
                    let st = ident.to_string();
                    let mut iter = st.chars();

                    let rl = loop {
                        break match iter.next() {
                            // Identifiers that start with an uppercase letter are type references.
                            Some(c) if c.is_ascii_uppercase() => Rule::TypeReference(st),
                            // Identifiers that start with a lowercase letter are field identifiers.
                            Some(c) if c.is_ascii_lowercase() => Rule::Identifier(st),
                            // Identifiers that start with `_` inherit the kind from the first letter.
                            Some(_) => continue,
                            // TODO If we reach the end, we should decide if it is a variable or panic.
                            None => Rule::Identifier(st),
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
                        del => {} //TODO panic!("expected parenthesis, got {del:?}"),
                    }
                }
            }
        }

        // println!("attr: \"{attrs:?}\"");

        // todo!()
        Rule::Scope(vec)

        // compile_error!("Not implemented")
    }
}
