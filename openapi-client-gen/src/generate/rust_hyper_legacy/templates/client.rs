use proc_macro2::TokenStream;
use quote::quote;

use crate::lang::rust::to_doc_comment;
use crate::lang::rust::{struct_name, var_name};
use crate::{error::Error, spec::Spec};

pub fn client(spec: &Spec) -> Result<String, Error> {
    let api_functions = api_functions(spec);
    let api_implementations = api_implementations(spec);

    let code = quote! {
        use super::apis;
        use super::config::HasConfig;
        use super::config::ClientConfig;
        use super::config::Connector;
        use super::config::Config;
        use crate::impl_api_client;

        pub struct Client {
            config: Box<dyn ClientConfig>,
        }

        impl Client {
            pub fn new<C: Connector>(config: Config<C>) -> Client {
                Client {
                    config: Box::new(config),
                }
            }
        }

        impl_api_client!(Client, config);

        #[macro_export]
        macro_rules! impl_api_client {
            ($struct_name:ident, $config_field:ident) => {
                impl HasConfig for $struct_name {
                    fn get_config(&self) -> &dyn ClientConfig {
                        &*self.$config_field
                    }
                }
                impl $struct_name {
                    #(#api_functions)*
                }
                #(#api_implementations)*
            };
        }

    };

    let syn_file: syn::File = syn::parse2(code)?;
    Ok(prettyplease::unparse(&syn_file))
}

fn api_implementations<'a>(spec: &'a Spec) -> impl Iterator<Item = TokenStream> + 'a {
    spec.tags.values().map(|tag| {
        let struct_name = struct_name(&tag.name);

        quote! {
            impl apis::#struct_name for $struct_name {}
        }
    })
}

fn api_functions<'a>(spec: &'a Spec) -> impl Iterator<Item = TokenStream> + 'a {
    spec.tags.values().map(|tag| {
        let struct_name = struct_name(&tag.name);
        let var_name = var_name(&tag.name);
        let doc_comments = to_doc_comment(&tag.description);

        quote! {
            #(#doc_comments)*
            pub fn #var_name(&self) -> &dyn apis::#struct_name {
                self
            }
        }
    })
}
