use proc_macro2::TokenStream;
use quote::quote;

use crate::lang::rust::{ident, struct_name, to_doc_comment, var_name};
use crate::{error::Error, spec::Spec};

pub fn client(
    spec: &Spec,
    common_module: &syn::Path,
    api_module_str: &str,
) -> Result<String, Error> {
    let api_module: syn::Path = syn::parse_str(api_module_str)?;
    let api_implementations = api_implementations(spec, &api_module);
    let api_functions = api_functions(spec);
    let apis = spec.tags.values().map(|tag| {
        let struct_name = struct_name(&tag.name);
        quote! {
            apis::#struct_name
        }
    });
    let macro_name = ident(&format!("impl_{}_traits", &var_name(api_module_str)));
    log::warn!("{}", macro_name);

    let code = quote! {
        use std::marker::Sized;
        use super::apis;
        use #common_module::config::HasConfig;
        use #common_module::config::ClientConfig;
        use #common_module::config::Connector;
        use #common_module::config::Config;
        use crate::#macro_name;

        pub struct APIClient {
            config: Box<dyn ClientConfig>,
        }

        impl APIClient {
            pub fn new<C: Connector>(config: Config<C>) -> APIClient {
                APIClient {
                    config: Box::new(config),
                }
            }
        }

        impl HasConfig for APIClient {
            fn get_config(&self) -> &dyn ClientConfig {
                &*self.config
            }
        }

        pub trait Client: HasConfig + Send + Sync + Sized + #(#apis+)* {
            #(#api_functions)*
        }

        #macro_name!(APIClient);

        #[macro_export]
        macro_rules! #macro_name {
            ($struct_name:ident) => {
                impl #api_module::Client for $struct_name {}
                #(#api_implementations)*
            };
        }

    };

    let syn_file: syn::File = syn::parse2(code)?;
    Ok(prettyplease::unparse(&syn_file))
}

fn api_implementations<'a>(
    spec: &'a Spec,
    api_module: &'a syn::Path,
) -> impl Iterator<Item = TokenStream> + 'a {
    spec.tags.values().map(move |tag| {
        let struct_name = struct_name(&tag.name);

        quote! {
            impl #api_module::apis::#struct_name for $struct_name {}
        }
    })
}

fn api_functions<'a>(spec: &'a Spec) -> impl Iterator<Item = TokenStream> + 'a {
    spec.tags.values().map(move |tag| {
        let struct_name = struct_name(&tag.name);
        let var_name = var_name(&tag.name);
        let doc_comments = to_doc_comment(&tag.description);

        quote! {
            #(#doc_comments)*
            fn #var_name(&self) -> &dyn apis::#struct_name {
                self
            }
        }
    })
}
