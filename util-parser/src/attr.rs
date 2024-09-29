use std::fmt::Display;

use proc_macro::{Delimiter, TokenStream, TokenTree};
// use util_cases::CaseStyles;

#[derive(Debug)]
pub enum Context {
    Keyword(String),
    Identifier(String),
    TypeReference(String),
    Scope(Vec<Context>),
    Option(Box<Context>),
    Repetition(Box<Context>),
}

impl From<TokenStream> for Context {
    fn from(attrs: TokenStream) -> Self {
        let mut vec = Vec::new();

        for attr in attrs.clone().into_iter() {
            // let span = attr.span().into();
            // let error = |msg: &dyn Display| syn::Error::new(span, msg).to_compile_error();

            match attr {
                TokenTree::Literal(literal) => {
                    if let Some(source) = literal.span().source_text() {
                        println!("Tag:   {source}");
                        vec.push(Context::Keyword(source));
                        continue;
                    }
                    // TODO error: could not unwrap literal
                }
                TokenTree::Punct(punct) => {
                    let c = punct.as_char();
                    match c {
                        '?' => {
                            if let Some(v) = vec.pop() {
                                vec.push(Context::Option(Box::new(v)));
                            }
                            // TODO error: `?` was used at the beginning of a scope
                        }
                        '*' => {
                            if let Some(v) = vec.pop() {
                                vec.push(Context::Repetition(Box::new(v)));
                            }
                            // TODO error: `*` was used at the beginning of a scope
                        }
                        _ => {
                            // TODO error: expected one of `?` or `*`, got `{c}`
                        }
                    }
                    let symbol = punct.as_char();
                    println!("Punct: {symbol}");
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
                        vec.push(Context::Identifier(st))
                    } else {
                        vec.push(Context::TypeReference(st))
                    };

                    println!("Ident: {}", ident.to_string());
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
        Context::Scope(vec)

        // compile_error!("Not implemented")
    }
}
