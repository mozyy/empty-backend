use proc_macro::{Ident, TokenStream};
use quote::quote;
use syn::{
    parse::{self, Parser},
    parse_macro_input, ItemStruct,
};

#[proc_macro_attribute]
pub fn add_orm_field(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(input as ItemStruct);
    let mut item_struct_new = item_struct.clone();
    let _ = parse_macro_input!(args as parse::Nothing);

    let mut has_id = false;
    let mut has_created_at = false;
    let mut has_updated_at = false;

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        let fields_iter = fields.clone().named.into_iter();

        fields_iter.for_each(|field| {
            if let Some(name) = field.ident {
                if name == "id" {
                    has_id = true
                } else if name == "created_at" {
                    has_created_at = true
                } else if name == "updated_at" {
                    has_updated_at = true
                }
            }
        });
        if !has_id {
            fields.named.insert(
                0,
                syn::Field::parse_named
                    .parse2(quote! { pub id: i32 })
                    .unwrap(),
            );
        }
        if !has_created_at {
            fields.named.push(
                syn::Field::parse_named
                    .parse2(quote! {
                        // #[serde(with = "timestamp")]
                        #[schema(value_type = i64)]
                        pub created_at: chrono::NaiveDateTime
                    })
                    .unwrap(),
            );
        }
        if !has_updated_at {
            fields.named.push(
                syn::Field::parse_named
                    .parse2(quote! {
                        // #[serde(with = "timestamp")]
                        #[schema(value_type = i64)]
                        pub updated_at: chrono::NaiveDateTime
                    })
                    .unwrap(),
            );
        }
    }
    let struct_name = item_struct.ident.to_string();
    // TODO: attr table_name
    let table_name = struct_name.to_lowercase() + "s";
    item_struct_new.ident = syn::Ident::new(
        format!("New{}", struct_name).as_str(),
        item_struct_new.ident.span(),
    );

    //     #[diesel(table_name = questions)]
    // pub struct NewQuestion {
    //     pub content: String,
    //     pub desc: Option<String>,
    // }

    quote! {
        // use crate::utils::timestamp;
        // use chrono::NaiveDateTime;
        // use diesel::prelude::*;
        // use serde::{Deserialize, Serialize};
        // use utoipa::ToSchema;

        #[derive(diesel::prelude::Queryable, diesel::prelude::Identifiable, serde::Serialize, utoipa::ToSchema)]
        #item_struct

        #[derive(diesel::prelude::Insertable,serde::Deserialize, utoipa::ToSchema)]
        // #[diesel(table_name = #table_name)]
        #item_struct_new
    }
    .into()
}
#[proc_macro_attribute]
pub fn add_orm_field2(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(input as ItemStruct);
    let mut item_struct_new = item_struct.clone();
    let _ = parse_macro_input!(args as parse::Nothing);

    let mut has_id = false;
    let mut has_created_at = false;
    let mut has_updated_at = false;

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        let fields_iter = fields.clone().named.into_iter();

        fields_iter.for_each(|field| {
            if let Some(name) = field.ident {
                if name == "id" {
                    has_id = true
                } else if name == "created_at" {
                    has_created_at = true
                } else if name == "updated_at" {
                    has_updated_at = true
                }
            }
        });
        if !has_id {
            fields.named.insert(
                0,
                syn::Field::parse_named
                    .parse2(quote! { pub id: i32 })
                    .expect("123"),
            );
        }
        if !has_created_at {
            fields.named.push(
                syn::Field::parse_named
                    .parse2(quote! {
                        #[serde(with = "crate::utils::timestamp")]
                        #[schema(value_type = i64)]
                        pub created_at: chrono::NaiveDateTime
                    })
                    .expect("1234"),
            );
        }
        if !has_updated_at {
            fields.named.push(
                syn::Field::parse_named
                    .parse2(quote! {
                        #[serde(with = "crate::utils::timestamp")]
                        #[schema(value_type = i64)]
                        pub updated_at: chrono::NaiveDateTime
                    })
                    .expect("1235"),
            );
        }
    }
    let struct_name = item_struct.ident.to_string();
    // TODO: attr table_name
    let table_name = struct_name.to_lowercase() + "s";
    item_struct_new.ident = syn::Ident::new(
        format!("New{}", struct_name).as_str(),
        item_struct_new.ident.span(),
    );

    //     #[diesel(table_name = questions)]
    // pub struct NewQuestion {
    //     pub content: String,
    //     pub desc: Option<String>,
    // }

    quote! {
        // use crate::utils::timestamp;
        // use chrono::NaiveDateTime;
        // use diesel::prelude::*;
        // use serde::{Deserialize, Serialize};
        // use utoipa::ToSchema;

        #[derive(diesel::prelude::Queryable, diesel::prelude::Identifiable, serde::Serialize, utoipa::ToSchema)]
        #item_struct

        #[derive(diesel::prelude::Insertable, serde::Deserialize, utoipa::ToSchema)]
        // #[diesel(table_name = #table_name)]
        #item_struct_new
    }
    .into()
}
