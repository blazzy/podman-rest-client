use std::collections::BTreeMap;

use convert_case::{Case, Casing};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use regex::Regex;
use syn::Ident;

use crate::error::Error;
use crate::model;
use crate::model::Model;
use crate::model::ModelData;
use crate::operation;
use crate::parameter;
use crate::parse;

/// Format name to a conventional upper camel rust ident for struct and trait names
pub fn struct_name(name: &str) -> Ident {
    ident(&safe_name(name).to_case(Case::UpperCamel))
}

/// Format name to a conventional rust snake variable and prefix with r# in case it is a keyword
pub fn var_name(name: &str) -> Ident {
    let var_name = safe_name(name).to_case(Case::Snake);
    ident(&var_name)
}

/// Replace all non alphanumeric characters with _ for safe idents
pub fn safe_name(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect()
}

impl parameter::BaseType {
    pub fn as_token_stream(&self) -> TokenStream {
        use parameter::BaseType::*;
        match &self {
            String => quote! { &str },
            Boolean => quote! { bool },
            Integer => quote! { i64 },
        }
    }
}

impl parameter::Parameter {
    pub fn type_as_token_stream(&self) -> TokenStream {
        let tokens = match &self.r#type {
            parameter::Type::Just(base_type) => base_type.as_token_stream(),
            parameter::Type::Array(base_type) => {
                let base_type = base_type.as_token_stream();
                quote! { Vec<#base_type> }
            }
        };
        if self.required {
            tokens
        } else {
            quote! { Option<#tokens> }
        }
    }
}

impl model::IntegerFormat {
    pub fn as_token_stream(&self) -> TokenStream {
        use model::IntegerFormat::*;
        match self {
            INT64 => quote! { i64 },
            UINT64 => quote! { u64},
            INT32 => quote! { i32 },
            UINT32 => quote! { u32 },
            INT16 => quote! { i16 },
            UINT16 => quote! { u16 },
            INT8 => quote! { i8 },
            UINT8 => quote! { u8 },
        }
    }
}

impl operation::Operation {
    pub fn params_struct_as_token_stream(&self) -> TokenStream {
        let struct_name = struct_name(&self.name);
        let mut base = quote! { super::super::params::#struct_name };

        if self.params_struct_has_str() {
            base = quote! { #base<'a> }
        }

        // If all the params are optional, make the top level struct optional as well
        if self.is_optional_params_struct() {
            base = quote! {Option<#base>};
        }

        base
    }
}

pub fn parameter_to_str(var_name: &TokenStream, parameter: &parameter::Parameter) -> TokenStream {
    match &parameter.r#type {
        parameter::Type::Just(base_type) => match base_type {
            parameter::BaseType::String => quote! { #var_name },
            _ => quote! { &#var_name.to_string() },
        },
        parameter::Type::Array(_) => quote! {
            &#var_name.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(",")
        },
    }
}

// TODO: use syn::parse_str instead. Ident::new panics!
pub fn ident(str: &str) -> Ident {
    if parse::is_keyword(str) {
        Ident::new_raw(str, Span::call_site())
    } else {
        Ident::new(str, Span::call_site())
    }
}

pub fn to_doc_comment(text: &str) -> Vec<TokenStream> {
    // We find code blocks and add text annotations if they have no annotations lest the be parsed
    // as rust doc strings
    //
    // m - multiline match
    // s - `.` matches the newline characters too
    // .*? non-greedy match
    let re = Regex::new(r"(?ms)^```\s*$(.*?^```)").unwrap();
    let text = re.replace_all(text, "```text$1");

    text.lines()
        .map(|line| {
            let line = format!(" {}", line.trim());
            quote! { #[doc = #line] }
        })
        .collect::<Vec<_>>()
}

pub fn model_type(model: &Model, models: &BTreeMap<String, Model>) -> Result<TokenStream, Error> {
    Ok(match &model.data {
        ModelData::String => quote! { String },
        ModelData::Integer(format) => format.as_token_stream(),
        ModelData::Number => quote! { f64 },
        ModelData::Boolean => quote! { f64 },
        ModelData::Array(items) => {
            let items = model_type(items, models)?;
            quote! { Vec<#items> }
        }
        ModelData::ArbitraryValue => quote! { serde_json::Value },
        ModelData::NoValue => quote! { () },
        ModelData::Object(_) => {
            let struct_name = struct_name(&model.name);
            quote! { super::super::models::#struct_name }
        }
        ModelData::HashMap(value, nullable) => {
            let value = model_type(value, models)?;
            let map = quote! { std::collections::HashMap::<String, #value> };
            if *nullable {
                quote! { Option<#map> }
            } else {
                map
            }
        }
        ModelData::Ref(ref_str) => {
            if let Some(ref_model) = models.get(ref_str) {
                model_type(ref_model, models)?
            } else {
                Err(Error::MissingModelRef(ref_str.into()))?
            }
        }
    })
}
