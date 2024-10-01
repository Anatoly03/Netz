use super::Rule;
use proc_macro::{Delimiter, TokenStream, TokenTree};
use quote::quote;

impl Into<TokenStream> for Rule {
    fn into(self) -> TokenStream {
        match self {
            Rule::Keyword(_) => todo!(),
            Rule::Identifier(_) => todo!(),
            Rule::TypeReference(_) => todo!(),
            Rule::Scope(vec) => todo!(),
            Rule::Option(rule) => todo!(),
            Rule::Repetition(rule) => todo!(),
            Rule::Branch(vec) => todo!(),
        }
    }
}
