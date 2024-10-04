use super::Rule;
use proc_macro::TokenStream;
use quote::{format_ident, quote};

impl Rule {
    pub fn construct(self, ident: String) -> TokenStream {
        let callback: proc_macro2::TokenStream = Into::<TokenStream>::into(self).into();
        let name_ident = format_ident!("{}", ident);

        quote! {
            impl #name_ident {
                pub fn parse(input: &str) -> nom::IResult<&str, ()> {
                    nom::combinator::value((), #callback)(input)
                }
            }
        }
        .into()
    }
}

impl Into<TokenStream> for Rule {
    fn into(self) -> TokenStream {
        match self {
            // Whitespace is an atomic lambda. It will expect one whitespace
            // character and trim the start.
            Rule::Whitespace => {
                // quote! { Self::whitespace1 }
                quote! { nom::character::complete::multispace1 }
            }
            Rule::Keyword(s) => todo!("keyword not implemented"),
            Rule::Identifier(_) => todo!("identifier not implemented"),
            Rule::TypeReference(_) => todo!("typeReference not implemented"),
            Rule::Scope(vec) => todo!("scope not implemented"),
            Rule::Option(rule) => {
                let stream: proc_macro2::TokenStream = Into::<TokenStream>::into(*rule).into();
                // quote! { Self::option( #stream ) }
                quote! { nom::combinator::opt( #stream ) }
            }
            Rule::Repetition(rule) => todo!("repetition not implemented"),
            Rule::Branch(vec) => todo!("branch not implemented"),
        }
        .into()
    }
}
