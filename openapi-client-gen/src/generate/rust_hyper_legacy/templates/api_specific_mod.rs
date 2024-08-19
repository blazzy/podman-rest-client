use quote::quote;

use crate::error::Error;

pub fn render(skip_default_client: bool) -> Result<String, Error> {
    let maybe_api_client = if skip_default_client {
        None
    } else {
        Some(quote! { pub use client::APIClient; })
    };

    let code = quote! {
        pub mod apis;
        mod client;
        pub mod models;
        pub mod params;

        #maybe_api_client
        pub use client::Client;
    };

    let syn_file: syn::File = syn::parse2(code).unwrap();
    Ok(prettyplease::unparse(&syn_file))
}
