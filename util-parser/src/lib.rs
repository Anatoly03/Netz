mod attr;
use attr::Context;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn grammar(attrs: TokenStream, items: TokenStream) -> TokenStream {
    let context = Context::from(attrs);
    println!();

    for item in items.clone().into_iter() {
        // if let Some(token) = item.
        println!("{item:?}");
    }

    println!();

    println!("item: \"{items}\"");
    items
}

// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
