mod attr;
mod item;

// #[cfg(test)]
// mod tests;

use attr::Rule;
use item::parse;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn grammar(attrs: TokenStream, items: TokenStream) -> TokenStream {
    let context = Rule::from(attrs);
    // println!("{context:?}");
    let result = parse(context, items);
    println!("{result}");
    result
}
