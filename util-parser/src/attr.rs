use proc_macro::{Delimiter, TokenStream, TokenTree};

pub enum Context {
    Identifier(String),
    TypeReference(String),
}

impl From<TokenStream> for Context {
    fn from(attrs: TokenStream) -> Self {
        for attr in attrs.clone().into_iter() {
            match attr {
                TokenTree::Literal(literal) => {
                    if let Some(source) = literal.span().source_text() {
                        println!("Tag:   {source}");
                    }
                }
                TokenTree::Punct(punct) => {
                    let symbol = punct.as_char();
                    println!("Punct: {symbol}");
                }
                TokenTree::Ident(ident) => {
                    println!("Ident: {:?}", ident.to_string());
                }
                TokenTree::Group(group) => {
                    println!("->");
                    let _ = Self::from(group.stream());
                    println!("<-");

                    match group.delimiter() {
                        Delimiter::Parenthesis => {
                            // TODO
                        }
                        del => panic!("expected parenthesis, got {del:?}"),
                    }
                    // println!("Group: {group:?}");
                }
            }
            // if let Some(token) = item.
        }

        // println!("attr: \"{attrs:?}\"");

        // todo!()
        Context::Identifier(String::from("Hi!"))

        // compile_error!("Not implemented")
    }
}
