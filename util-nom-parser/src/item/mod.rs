mod structs;

use crate::regexp::Rule;
use proc_macro::TokenStream;
use syn::{parse_macro_input, spanned::Spanned, Item};

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
