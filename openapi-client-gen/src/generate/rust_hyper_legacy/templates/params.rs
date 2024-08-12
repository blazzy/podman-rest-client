use proc_macro2::TokenStream;
use quote::quote;

use crate::error::Error;
use crate::lang::rust::{struct_name, to_doc_comment, var_name};
use crate::operation::Operation;
use crate::parameter::Parameter;

pub fn params(operation: &Operation) -> Result<String, Error> {
    let struct_name = struct_name(&operation.name);
    let header_params = render_params(&operation.header_params)?;
    let query_params = render_params(&operation.query_params)?;
    let lifetime = if operation.params_struct_has_str() {
        Some(quote! { <'a> })
    } else {
        None
    };

    let code = quote! {
            #[derive(Default, Debug)]
            pub struct #struct_name #lifetime {
                #(#header_params)*
                #(#query_params)*
            }

    //#[derive(Default, Debug)]
    //pub struct {{operation.struct_name()}}{% if operation.params_struct_has_str() %}<'a>{% endif %} {
        };

    let syn_file: syn::File = syn::parse2(code)?;
    Ok(prettyplease::unparse(&syn_file))
}

fn render_params(parameters: &[Parameter]) -> Result<Vec<TokenStream>, Error> {
    parameters
        .iter()
        .map(|param| {
            let description = param
                .description
                .as_ref()
                .map(|s| to_doc_comment(s))
                .unwrap_or_default();

            let var_name = var_name(&param.name);
            let lifetime = quote! { 'a };
            let param_type = param.type_as_token_stream(Some(lifetime));

            Ok(quote! {
                #(#description)*
                pub #var_name: #param_type,
            })
        })
        .collect::<Result<Vec<_>, Error>>()
}
