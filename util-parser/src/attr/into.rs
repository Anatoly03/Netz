use super::Rule;
use proc_macro::{Delimiter, TokenStream, TokenTree};
use quote::{format_ident, quote};

impl Rule {
    pub fn construct(self, ident: String) -> TokenStream {
        let callback: proc_macro2::TokenStream = Into::<TokenStream>::into(self).into();
        let name_ident = format_ident!("{}", ident);

        // TODO documentation: `whitespace1` 
        // Expects one whitespace characters and trims the rest,
        // roughly analogous to regex `\s+`

        quote! {
            impl #name_ident {
                // type I = &str;

                // type Out<O> = Result<(&str, O), String>;
                // type Closure<O> = dyn Fn(&str) -> Out<O>;

                fn whitespace1(input: &str) -> Result<(&str, ()), String> {
                    match input.chars().next() {
                        Some(c) if c.is_whitespace() => Ok((input.trim_start(), ())),
                        Some(c) => Err(format!("expected whitespace, got `{}`", c)),
                        None => Err("expected whitespace, got end of string".to_owned()),
                    }
                }

                // fn option(call: Closure<()>) -> Closure<()> {
                //     move |input: &str| {
                //         match call(input) {
                //             Ok(o) => Ok(o),
                //             Err(_) => Ok((input, ())),
                //         }
                //     }
                // }

                // Sequences the parser through a list of grammar functions
                // fn sequence(input: Vec<T>) -> Result<(&str, ()), String> where
                //     T: Fn(&str) -> Result<(&str, ()), String>
                // {
                //     for closure in input.into_iter() {
                //         closure(input)?;
                //     }
                //     ()
                // }

                pub fn parse(input: &str) -> Result<(&str, &str), String> {
                    // let (input, _) = (#callback)(input)?;
                    Ok((input, ""))
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
            Rule::Whitespace => quote! { Self::whitespace1 },
            Rule::Keyword(s) => todo!("keyword not implemented"),
            Rule::Identifier(_) => todo!("identifier not implemented"),
            Rule::TypeReference(_) => todo!("typeReference not implemented"),
            Rule::Scope(vec) => todo!("scope not implemented"),
            Rule::Option(rule) => {
                let stream: proc_macro2::TokenStream = Into::<TokenStream>::into(*rule).into();
                quote! { Self::option( #stream ) }
            },
            Rule::Repetition(rule) => todo!("repetition not implemented"),
            Rule::Branch(vec) => todo!("branch not implemented"),
        }
        .into()
    }
}
