mod structs;

use crate::attr::Rule;
use proc_macro::TokenStream;
use syn::{parse_macro_input, spanned::Spanned, Item};

pub enum IdentifierCounter {
    // None,// default
    Scalar, // defined
    Option, // optionally defined
    Many,   // multiple defined
}

pub fn parse(context: Rule, items: TokenStream) -> TokenStream {
    let input = parse_macro_input!(items as Item);

    match input {
        Item::Struct(s) => structs::parse_struct(context, s),
        _ => syn::Error::new(
            input.span(),
            "The macro [grammar] is only supported with structs",
        )
        .to_compile_error()
        .into(),
    }
}
