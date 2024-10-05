//! This module contains the ruleset parser. A ruleset is an AST, which represents
//! the grammar inside the macro as a production rule. Consider `[grammar{ "@" ident
//! }]` for any struct `T` as a production of form `<T> -> "@" <Ident>`.

use std::str::FromStr;

use super::Rule;
use proc_macro::{Delimiter, TokenStream, TokenTree};

impl From<TokenStream> for Rule {
    fn from(attrs: TokenStream) -> Self {
        let mut vec = Vec::new();

        for attr in attrs.clone().into_iter() {
            // let span = attr.span().into();
            // let error = |msg: &dyn Display| syn::Error::new(span, msg).to_compile_error();

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
                            _ => todo!("unparsable literal: {lit}"),
                        },
                        Err(e) => {
                            todo!("handle error: {e}")
                        }
                    }
                    // if let Some(source) =  {
                    //     // println!("Tag:   {source}");
                    //     continue;
                    // }
                    // TODO error: could not unwrap literal
                }
                TokenTree::Punct(punct) => {
                    match punct.as_char() {
                        '~' => vec.push(Rule::Whitespace),
                        '?' => {
                            if let Some(v) = vec.pop() {
                                vec.push(Rule::Option(Box::new(v)));
                            }
                            // TODO error: `?` was used at the beginning of a scope
                        }
                        '*' => {
                            if let Some(v) = vec.pop() {
                                vec.push(Rule::Repetition(Box::new(v)));
                            }
                            // TODO error: `*` was used at the beginning of a scope
                        }
                        _ => {
                            // TODO error: expected one of `?` or `*`, got `{c}`
                        }
                    }
                    let symbol = punct.as_char();
                    // println!("Punct: {symbol}");
                }
                TokenTree::Ident(ident) => {
                    let st = ident.to_string();
                    let mut iter = st.chars();

                    let is_variable = loop {
                        match iter.next() {
                            Some(c) if c.is_ascii_uppercase() => break false,
                            Some(c) if c.is_ascii_lowercase() => break true,
                            Some(c) => continue,
                            None => break true,
                        }
                    };

                    if is_variable {
                        vec.push(Rule::Identifier(st))
                    } else {
                        vec.push(Rule::TypeReference(st))
                    };

                    // println!("Ident: {}", ident.to_string());
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
