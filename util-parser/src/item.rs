use crate::attr::Context;
use proc_macro::{Delimiter, TokenStream, TokenTree};

pub enum IdentifierCounter {
    None,   // default
    Scalar, // defined
    Option, // optionally defined
    Many,   // multiple defined
}

pub fn generate(context: &Context, items: TokenStream) -> TokenStream {
    for item in items.clone().into_iter() {
        match item {
            TokenTree::Literal(literal) => {
                if let Some(source) = literal.span().source_text() {
                    println!("Tag:   {source}");
                    // vec.push(Context::Keyword(source));
                    continue;
                }
                // TODO error: could not unwrap literal
            }
            TokenTree::Punct(punct) => {
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

                // if is_variable {
                //     vec.push(Context::Identifier(st))
                // } else {
                //     vec.push(Context::TypeReference(st))
                // };

                println!("Ident: {}", ident.to_string());
            }
            TokenTree::Group(group) => {
                match group.delimiter() {
                    Delimiter::Brace => {
                        let subcontext = generate(context, group.stream());
                        // vec.push(subcontext);
                    }
                    Delimiter::Bracket => {
                        let subcontext = generate(context, group.stream());
                        // vec.push(subcontext);
                    }
                    Delimiter::None => {
                        let subcontext = generate(context, group.stream());
                        // vec.push(subcontext);
                    }
                    Delimiter::Parenthesis => {
                        let subcontext = generate(context, group.stream());
                        // vec.push(subcontext);
                    }
                }
            }
        }

        // println!("{item:?}");
    }

    println!();


    todo!()
}
