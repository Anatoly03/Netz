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
                quote! { nom::character::complete::multispace1 }
            }
            Rule::Keyword(s) => {
                quote! { nom::bytes::complete::tag (#s) }
            }
            Rule::Identifier(_) => todo!("identifier not implemented"),
            Rule::TypeReference(_) => todo!("typeReference not implemented"),
            Rule::Scope(vec) => {
                if vec.len() > 20 {
                    panic!("a sequence of more than twenty elements is currently not supported")
                }

                let mut vec2 = vec
                    .into_iter()
                    .map(|v| Into::<proc_macro2::TokenStream>::into(Into::<TokenStream>::into(v)));

                match vec2.next() {
                    Some(k) => {
                        let s = vec2.fold(k, |a, b| quote! {#a , #b});
                        quote! { nom::sequence::tuple ((#s)) }
                    }
                    None => {
                        panic!("empty scope")
                    }
                }
            }
            Rule::Option(rule) => {
                let stream: proc_macro2::TokenStream = Into::<TokenStream>::into(*rule).into();
                quote! { nom::combinator::opt( #stream ) }
            }
            Rule::Repetition(rule) => {
                let stream: proc_macro2::TokenStream = Into::<TokenStream>::into(*rule).into();
                quote! { nom::multi::many0( #stream ) }
            }
            Rule::Range(_) => todo!("range not implemented"),
            Rule::Branch(vec) => todo!("branch not implemented"),
        }
        .into()
    }
}
