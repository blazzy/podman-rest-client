use convert_case::{Case, Casing};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use regex::Regex;

use crate::{error::Error, parse, spec::Spec};

/// Replace all non alphanumeric characters with _ for safe idents
pub fn safe_name(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect()
}

/// Format name to a conventional rust snake variable and prefix with r# in case it is a keyword
pub fn var_name(name: &str) -> String {
    let var_name = safe_name(name).to_case(Case::Snake);
    if parse::is_keyword(&var_name) {
        format!("r#{}", var_name)
    } else {
        var_name
    }
}

/// Format name to a conventional upper camel rust ident for struct and trait names
pub fn struct_name(name: &str) -> String {
    safe_name(name).to_case(Case::UpperCamel)
}

/// Generate a mod.rs file contents that declares submodules and re-exports structs with the same
/// name as the module
pub fn mod_pub_structs<'a, I>(names: I) -> Result<String, Error>
where
    I: Iterator<Item = &'a String>,
{
    let import_apis = names
        .map(|name| {
            let struct_name = syn::Ident::new(&struct_name(name), Span::call_site());
            let mod_name = syn::Ident::new(&var_name(name), Span::call_site());

            quote! {
                mod #mod_name;
                pub use #mod_name::#struct_name;
            }
        })
        .collect::<Vec<_>>();

    let code = quote! { #(#import_apis)* };
    let syn_file: syn::File = syn::parse2(code)?;
    Ok(prettyplease::unparse(&syn_file))
}

fn to_doc_comment(text: &str) -> Vec<TokenStream> {
    // We find code blocks and add text annotations if to them if they have no
    // annotations lest the be parsed as rust doc strings
    // m - multiline match
    // s - cause `.` match the newline characters too
    // .*? non-greedy match
    let re = Regex::new(r"(?ms)^```\s*$(.*?^```)").unwrap();
    let text = re.replace_all(text, "```text$1").to_string();

    text.lines()
        .map(|line| {
            let line = format!(" {}", line);
            quote! { #[doc = #line] }
        })
        .collect::<Vec<_>>()
}

pub fn client(spec: &Spec) -> Result<String, Error> {
    let functions = spec.tags.values().map(|tag| {
        let struct_name = syn::Ident::new(&struct_name(&tag.safe_name), Span::call_site());
        let var_name = syn::Ident::new(&var_name(&tag.safe_name), Span::call_site());
        let doc_comments = to_doc_comment(&tag.description);

        quote! {
            #(#doc_comments)*
            pub fn #var_name(&self) -> apis::#struct_name {
                apis::#struct_name::new(self.config.clone())
            }
        }
    });

    let code = quote! {
        use std::sync::Arc;

        use super::config::Connector;
        use super::config::ClientConfig;
        use super::config::Config;
        use super::apis;

        pub struct Client {
          config: Arc<dyn ClientConfig>
        }

        impl Client {
            pub fn new<C: Connector>(config: Config<C>) -> Client {
                Client {
                  config: Arc::new(config),
                }
            }

            #(#functions)*
        }
    };

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
