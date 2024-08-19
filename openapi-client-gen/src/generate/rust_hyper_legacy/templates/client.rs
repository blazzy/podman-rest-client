use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::lang::rust::{ident, struct_name, to_doc_comment, var_name};
use crate::{error::Error, spec::Spec};

pub fn client(
    spec: &Spec,
    common_module: &syn::Path,
    api_module_str: &str,
    skip_default_client: bool,
) -> Result<String, Error> {
    let api_module: syn::Path = syn::parse_str(api_module_str)?;
    let api_implementations = api_implementations(spec, &api_module);
    let api_trait_functions = api_trait_functions(spec);
    let api_impl_functions = api_impl_functions(spec, &api_module);
    let macro_name = ident(&format!("impl_{}_traits", &var_name(api_module_str)));
    let default_client = if skip_default_client {
        None
    } else {
        Some(default_client(&macro_name, common_module))
    };
    let apis = spec.tags.values().map(|tag| {
        let struct_name = struct_name(&tag.name);
        quote! {
            apis::#struct_name
        }
    });

    let code = quote! {
        use super::apis;
        use #common_module::config::HasConfig;

        #default_client

        pub trait Client: HasConfig + Send + Sync + #(#apis+)* {
            #(#api_trait_functions)*
        }

        #[macro_export]
        macro_rules! #macro_name {
            ($struct_name:ident) => {
                impl #api_module::Client for $struct_name {
                    #(#api_impl_functions)*
                }
                #(#api_implementations)*
            };
        }

    };

    let syn_file: syn::File = syn::parse2(code)?;
    Ok(prettyplease::unparse(&syn_file))
}

fn default_client(macro_name: &Ident, common_module: &syn::Path) -> TokenStream {
    quote! {
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

        #macro_name!(APIClient);
    }
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

fn api_trait_functions<'a>(spec: &'a Spec) -> impl Iterator<Item = TokenStream> + 'a {
    spec.tags.values().map(move |tag| {
        let struct_name = struct_name(&tag.name);
        let var_name = var_name(&tag.name);
        let doc_comments = to_doc_comment(&tag.description);

        quote! {
            #(#doc_comments)*
            fn #var_name(&self) -> &dyn apis::#struct_name;
        }
    })
}

fn api_impl_functions<'a>(
    spec: &'a Spec,
    api_module: &'a syn::Path,
) -> impl Iterator<Item = TokenStream> + 'a {
    spec.tags.values().map(move |tag| {
        let struct_name = struct_name(&tag.name);
        let var_name = var_name(&tag.name);
        let doc_comments = to_doc_comment(&tag.description);

        quote! {
            #(#doc_comments)*
            fn #var_name(&self) -> &dyn #api_module::apis::#struct_name {
                self
            }
        }
    })
}
