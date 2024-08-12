pub mod apis;
pub mod client;
pub mod config;
pub mod models;
pub mod params;

use quote::quote;

use crate::error::Error;
use crate::lang::rust::{struct_name, var_name};

/// Generate a mod.rs file contents that declares submodules and re-exports structs with the same
/// name as the module
pub fn mod_pub_structs<'a, I>(names: I) -> Result<String, Error>
where
    I: Iterator<Item = &'a String>,
{
    let import_apis = names.map(|name| {
        let struct_name = struct_name(name);
        let mod_name = var_name(name);

        quote! {
            mod #mod_name;
            pub use #mod_name::#struct_name;
        }
    });

    let code = quote! { #(#import_apis)* };
    let syn_file: syn::File = syn::parse2(code)?;
    Ok(prettyplease::unparse(&syn_file))
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_pub_use_struct() {
        let names = ["foo".to_string(), "fooBar".to_string()];
        let string = mod_pub_structs(names.iter()).expect("Failed to generate mod file");
        assert_eq!(
            string,
            indoc! {r#"
                mod foo;
                pub use foo::Foo;
                mod foo_bar;
                pub use foo_bar::FooBar;
            "#}
        );
    }
}
