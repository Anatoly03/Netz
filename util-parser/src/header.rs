use crate::attr::Context;
use proc_macro::{Delimiter, TokenStream, TokenTree};

/// Reads in a struct declaration based on [Rust grammar](https://doc.rust-lang.org/reference/items/structs.html).
/// 
/// `(pub)? struct (<generics>)? (<where clause>)? (body)?`
pub fn read_struct_header(context: &Context, items: TokenStream) -> TokenStream {
    let mut iter = items.clone().into_iter();
    let mut item = iter.next();

    let mut is_public = false;

    // Try read in "pub" keyword
    let tt = item;

    if let Some(tt) = item {
        if let TokenTree::Ident(c) = tt {
            if c.to_string() == "pub" {
                is_public = true;
                item = iter.next();
            }
        }
    }

    // Read in "struct" keyword
    if let Some(tt) = item {
        if let TokenTree::Ident(c) = tt {
            if c.to_string() == "struct" {
                is_public = true;
                item = iter.next();
            }
        }
    }

    items
}
