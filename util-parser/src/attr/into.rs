use super::Rule;
use proc_macro::{Delimiter, TokenStream, TokenTree};
use quote::{format_ident, quote};

impl Rule {
    pub fn construct(self, ident: String) -> TokenStream {
        // let callback: TokenStream = self.into();
        let name_ident = format_ident!("{}", ident);

        quote! {
            impl #name_ident {
                /// The following function parses the production of
                ///
                #[doc = #ident]
                /// over a string input and returns the new pointer in the
                /// string and Self as output.
                fn parse(input: &str) -> Result<(&str, &str), String> {
                //     let (i, o) = #callback (input)?;
                //     (i, o)
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
            Rule::Whitespace => quote! { |input: &str| -> Result<(&str, ()), String> {
                if let Some(c) = input.chars().next() {
                    if c.is_whitespace() {
                        return Ok((input.trim_start(), ()))
                    } else {
                        Err(format!("expected whitespace, got `{}`", c))
                    }
                } else {
                    Err("expected whitespace, got end of string".to_owned())
                }
            } }
            .into(),
            Rule::Keyword(s) => todo!(),
            Rule::Identifier(_) => todo!(),
            Rule::TypeReference(_) => todo!(),
            Rule::Scope(vec) => todo!(),
            Rule::Option(rule) => todo!(),
            Rule::Repetition(rule) => todo!(),
            Rule::Branch(vec) => todo!(),
        }
    }
}
