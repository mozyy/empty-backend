use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{self, Nothing, Parser},
    parse_macro_input, ItemStruct,
};

mod schema;

#[proc_macro_attribute]
pub fn add_orm_field(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(input as ItemStruct);
    let mut item_struct_new = item_struct.clone();
    // let _args = parse_macro_input!(args as schema::parse::Attrs);
    let _args = parse_macro_input!(args as Nothing);

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
                        #[serde(with = "crate::utils::timestamp")]
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
                        #[serde(with = "crate::utils::timestamp")]
                        #[schema(value_type = i64)]
                        pub updated_at: chrono::NaiveDateTime
                    })
                    .unwrap(),
            );
        }
    }
    let struct_name = item_struct.ident.to_string();
    // TODO: attr table_name
    let table_name = struct_name.to_case(Case::Snake) + "s";
    let table_name: proc_macro2::TokenStream = table_name.parse().unwrap();
    item_struct_new.ident = syn::Ident::new(
        format!("New{}", struct_name).as_str(),
        item_struct_new.ident.span(),
    );

    quote! {

        #[derive(diesel::prelude::Queryable, diesel::prelude::Identifiable, serde::Serialize, utoipa::ToSchema)]
        #[diesel(table_name = #table_name)]
        #item_struct

        #[derive(diesel::prelude::Insertable,serde::Deserialize, utoipa::ToSchema)]
        #[diesel(table_name = #table_name)]
        #item_struct_new
    }
    .into()
}
