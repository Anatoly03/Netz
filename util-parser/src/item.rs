use std::collections::HashMap;

use crate::rule::Rule;
use proc_macro::{Delimiter, TokenStream, TokenTree};
use quote::ToTokens;
use syn::{parse_macro_input, spanned::Spanned, Field, ItemStruct, Type};
// use util_cases::CaseStyles;

pub enum IdentifierCounter {
    // None,// default
    Scalar, // defined
    Option, // optionally defined
    Many,   // multiple defined
}

fn list_fields(input: &ItemStruct) -> HashMap<String, Type> {
    input
        .fields
        .iter()
        .enumerate()
        .map(|(_idx, field)| {
            if let Some(field_name) = &field.ident {
                return (field_name.to_string(), field.ty.clone());
            }
            panic!(
                "The macro [grammar] does not support tuple structs: `{}`",
                input.ident.to_string()
            );
        })
        .collect()
}

pub fn generate(context: &Rule, items: TokenStream) -> TokenStream {
    let input = parse_macro_input!(items as ItemStruct);
    let name = input.ident.to_string();

    // TODO use where clauses?
    // TODO instead of panic, use: https://stackoverflow.com/questions/57025894/issuing-a-warning-at-compile-time
    if let Some(wher) = input.generics.where_clause {
        panic!("The macro [grammar] does not support `where` clauses, but `{name}` had it implemented.")
    }

    if let Some(c) = input.generics.const_params().next() {
        panic!("The macro [grammar] does not support const generics, but `{name}` had `{}` implemented.", c.ident.to_string())
    }

    if let Some(c) = input.generics.lifetimes().next() {
        panic!(
            "The macro [grammar] does not support lifetimes, but `{name}` had `{}` implemented.",
            c.lifetime.ident.to_string()
        )
    }

    if let Some(c) = input.generics.type_params().next() {
        panic!(
            "The macro [grammar] does not support generics, but `{name}` had `{}` implemented.",
            c.ident.to_string()
        )
    }

    let fields = list_fields(&input);

    // let declared_fields = input
    //     .fields
    //     .iter()
    //     .enumerate()
    //     .map(|(_idx, field)| {
    //         if let Some(field_name) = &field.ident {
    //             return (field_name.to_string(), field.ty.clone());
    //         }
    //         panic!("The macro [grammar] does not support tuple structs: `{name}`");
    //     })
    //     .collect::<HashMap<String, Type>>();

    // TODO

    println!("{fields:?}");

    input.to_token_stream().into()
}
