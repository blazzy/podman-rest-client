use proc_macro2::TokenStream;
use quote::quote;

use crate::lang::rust::{
    model_type, or_default, parameter_to_str, struct_name, to_doc_comment, var_name,
};
use crate::{error::Error, spec::Spec, tag::Tag};

pub fn api(spec: &Spec, tag: &Tag) -> Result<String, Error> {
    let struct_name = struct_name(&tag.name);
    let operations = operations(spec, tag)?;

    let code = quote! {
        use super::super::config::HasConfig;
        use super::super::Error;
        use super::super::request;

        #[async_trait::async_trait]
        pub trait #struct_name: HasConfig + Send + Sync {
            #(#operations)*
        }
    };

    let syn_file: syn::File = syn::parse2(code).unwrap();
    Ok(prettyplease::unparse(&syn_file))
}

pub fn operations(spec: &Spec, tag: &Tag) -> Result<Vec<TokenStream>, Error> {
    tag
        .operations
        .iter()
        .map(|operation| {
            let fn_name = var_name(&operation.name);

            let success = operation.success_response();
            let response = success
                .map(|m| model_type(m, &spec.models))
                .unwrap_or_else(|| Ok(quote! { () }))?;

            let summary = operation
                .summary
                .as_ref()
                .map(|s| to_doc_comment(s))
                .unwrap_or_default();
            let description = operation
                .description
                .as_ref()
                .map(|s| to_doc_comment(s))
                .unwrap_or_default();
            let title = format!(" {} {}", operation.method, operation.path);
            let path_params = operation
                .path_params
                .iter()
                .map(|param| {
                    let var_name = var_name(&param.name);
                    let param_type = param.type_as_token_stream();
                    Ok(quote! { #var_name: #param_type, })
                })
                .collect::<Result<Vec<_>, Error>>()?;

            let params_struct = if operation.should_use_params_struct() {
                let struct_type = operation.params_struct_as_token_stream();
                quote! { params: #struct_type, }
            } else {
                TokenStream::new()
            };
            let (body_param, create_body) = if let Some(body) = &operation.body_param {
                let var_name = var_name(&body.name);
                let body_type = model_type(&body.model, &spec.models)?;
                let body_param = quote! { #var_name: #body_type, };
                let create_body =  quote! {
                    let body = serde_json::to_string(&#var_name)?;
                    req_builder = req_builder.header(hyper::header::CONTENT_TYPE, "application/json");
                    req_builder = req_builder.header(hyper::header::CONTENT_LENGTH, body.len());
                    let request = req_builder.body(body)?;
                };
                (body_param, create_body)
            } else {
                (TokenStream::new(), quote! { let request = req_builder.body(String::new())?;})
            };

            let execute_request = if let Some(success) = success {
                if success.resolve_model(&spec.models)?.data.is_no_value() {
                    quote! { request::execute_request_unit(self.get_config(), request).await }
                } else {
                    quote! {request::execute_request_json(self.get_config(), request).await }
                }
            } else {
                quote! { request::execute_request_unit(self.get_config(), request).await }
            };

            let set_path_params = if !operation.path_params.is_empty() {
                let iter = operation.path_params.iter().map(|param| {
                    let var_name = var_name(&param.name);
                    let replace_string = format!("{{{}}}", param.name);
                    quote! {
                        request_path = request_path.replace(#replace_string, #var_name);
                    }
                });
                quote! { #(#iter)* }
            } else {
                TokenStream::new()
            };


            let set_query_params = if !operation.query_params.is_empty() {
                let set_pairs = operation.query_params.iter().map(|param| {
                    let name = &param.name;
                    let var_name = var_name(&param.name);

                    if param.required {
                        let to_string = parameter_to_str(&quote! { params.#var_name }, param);
                        quote! { query_pairs.append_pair(#name, #to_string); }
                    } else {
                        let or_default = or_default(&param.x_client_default);
                        let to_string = parameter_to_str(&quote! { #var_name }, param);
                        quote! {
                            if let Some(#var_name) = params.#var_name #or_default {
                                query_pairs.append_pair(#name, #to_string);
                            }
                        }
                    }
                });

                quote! {
                    let mut query_pairs = request_url.query_pairs_mut();
                    #(#set_pairs)*
                }
            } else {
                TokenStream::new()
            };


            let set_header_params = if !operation.header_params.is_empty() {
                let set_pairs = operation.header_params.iter().map(|param| {
                    let name = &param.name;
                    let var_name = var_name(&param.name);

                    if param.required {
                        quote! { req_builder = req_builder.header(#name, params.#var_name); }
                    } else {
                        let or_default = or_default(&param.x_client_default);
                        quote! {
                            if let Some(#var_name) = params.#var_name #or_default {
                                req_builder = req_builder.header(#name, #var_name);
                            }
                        }
                    }
                });
                quote! { #(#set_pairs)* }
            } else {
                TokenStream::new()
            };

            let process_params_struct = if operation.should_use_params_struct() && operation.is_optional_params_struct() {
                quote! {
                    if let Some(params) = params {
                        #set_query_params
                        #set_header_params
                    }
                }
            } else {
                quote! { #set_query_params #set_header_params }
            };

            let path = &operation.path;
            let method = &operation.method.to_string();
            let fn_lifetime = if operation.params_struct_has_str() {
                quote! { <'a> }
            } else {
                TokenStream::new()
            };

            Ok(quote! {
                #[doc = #title]
                #(#summary)*
                #(#description)*
                async fn #fn_name #fn_lifetime(
                    &self,
                    #(#path_params)*
                    #params_struct #body_param
                ) -> Result<#response, Error> {
                    let mut request_url = url::Url::parse(self.get_config().get_base_path())?;

                    let mut request_path = request_url.path().to_owned();
                    if request_path.ends_with('/') {
                        request_path.pop();
                    }
                    request_path.push_str(#path);
                    #set_path_params
                    request_url.set_path(&request_path);

                    let mut req_builder = self.get_config().req_builder(#method)?;

                    #process_params_struct

                    let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
                    req_builder = req_builder.uri(hyper_uri);

                    #create_body
                    #execute_request
                }
            })
        })
        .collect::<Result<Vec<_>, Error>>()
}
