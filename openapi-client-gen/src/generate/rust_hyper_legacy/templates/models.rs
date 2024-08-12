use std::collections::BTreeMap;

use proc_macro2::TokenStream;
use quote::quote;

use crate::error::Error;
use crate::lang::rust::{property_type, struct_name, to_doc_comment, var_name};
use crate::model::{Model, Property};

pub fn models(
    model: &Model,
    properties: &[Property],
    models: &BTreeMap<String, Model>,
) -> Result<String, Error> {
    let title = model
        .title
        .as_ref()
        .map(|s| to_doc_comment(s))
        .unwrap_or_default();

    let description = if model.title != model.description {
        model
            .description
            .as_ref()
            .map(|s| to_doc_comment(s))
            .unwrap_or_default()
    } else {
        Vec::new()
    };

    let struct_name = struct_name(&model.name);
    let properties = render_properties(properties, models)?;

    let code = quote! {
        use serde::{Serialize, Deserialize};

        #[derive(Default, Debug, Serialize, Deserialize)]
        #(#title)*
        #(#description)*
        pub struct #struct_name {
            #(#properties)*
        }
    };

    let syn_file: syn::File = syn::parse2(code)?;
    Ok(prettyplease::unparse(&syn_file))
}

fn render_properties(
    properties: &[Property],
    models: &BTreeMap<String, Model>,
) -> Result<Vec<TokenStream>, Error> {
    properties
        .iter()
        .map(|property| {
            let title = property
                .model
                .title
                .as_ref()
                .map(|s| to_doc_comment(s))
                .unwrap_or_default();

            let description = property
                .model
                .description
                .as_ref()
                .map(|s| to_doc_comment(s))
                .unwrap_or_default();

            let var_name = var_name(&property.name);
            let property_type = property_type(property, models)?;
            let serde_annotation = if var_name != property.name {
                let name = &property.name;
                quote! { #[serde(rename = #name)] }
            } else {
                TokenStream::new()
            };

            Ok(quote! {
                #(#title)*
                #(#description)*
                #serde_annotation
                pub #var_name: #property_type,
            })
        })
        .collect::<Result<Vec<_>, Error>>()
}
