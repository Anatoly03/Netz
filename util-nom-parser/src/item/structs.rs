use crate::attr::Rule;
use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use std::collections::HashMap;
use syn::{Fields, FieldsNamed, ItemStruct, Type};

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

pub fn parse_struct(context: Rule, input: ItemStruct) -> TokenStream {
    let name = input.ident.to_string();
    let name_ident = format_ident!("{}", name);

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

    // TODO currently expect all structs to be empty and rely generation only on attr
    if !fields.is_empty() {
        panic!("The macro [grammar] does not support field parameters")
    }

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

    let definition = quote! {
        struct #name_ident {
            local: String
        }
    };

    // let definition = input.to_token_stream();
    let implementation = proc_macro2::TokenStream::from(context.construct(name));

    // println!("D: {definition}");
    // println!("I: {implementation}");

    quote! {
        #definition
        #implementation
    }.into()
}
