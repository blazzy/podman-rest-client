use std::collections::BTreeMap;

use convert_case::{Boundary, Case, Casing};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use regex::Regex;
use syn::Ident;

use crate::error::Error;
use crate::model;
use crate::model::ModelData;
use crate::model::{Model, Property};
use crate::operation;
use crate::parameter;
use crate::parameter::XClientDefault;

pub fn is_keyword(var: &str) -> bool {
    RUST_KEYWORDS.iter().any(|k| k == &var)
}

fn boundaries() -> Vec<Boundary> {
    vec![
        Boundary::Acronym,
        Boundary::Underscore,
        Boundary::Hyphen,
        Boundary::Space,
        Boundary::LowerUpper,
        Boundary::UpperDigit,
        Boundary::DigitUpper,
        Boundary::DigitLower,
    ]
}

/// Format name to a conventional upper camel rust ident for struct and trait names
pub fn struct_name(name: &str) -> Ident {
    ident(
        &safe_name(name)
            .with_boundaries(&boundaries())
            .to_case(Case::UpperCamel),
    )
}

/// Format name to a conventional rust snake variable and prefix with r# in case it is a keyword
pub fn var_name(name: &str) -> Ident {
    let var_name = safe_name(name)
        .with_boundaries(&boundaries())
        .to_case(Case::Snake);
    ident(&var_name)
}

/// Replace all non alphanumeric characters with _ for safe idents
pub fn safe_name<S: AsRef<str>>(name: S) -> String {
    name.as_ref()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect()
}

impl parameter::BaseType {
    pub fn as_token_stream(&self, lifetime: Option<TokenStream>) -> TokenStream {
        use parameter::BaseType::*;
        match &self {
            String => quote! { &#lifetime str },
            Boolean => quote! { bool },
            Integer => quote! { i64 },
        }
    }
}

impl parameter::Parameter {
    pub fn type_as_token_stream(&self, lifetime: Option<TokenStream>) -> TokenStream {
        let tokens = match &self.r#type {
            parameter::Type::Just(base_type) => base_type.as_token_stream(lifetime),
            parameter::Type::Array(base_type) => {
                let base_type = base_type.as_token_stream(lifetime);
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
    pub fn params_struct_as_token_stream(&self, module: &syn::Path) -> TokenStream {
        let struct_name = struct_name(&self.name);
        let mut base = quote! { #module::params::#struct_name };

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
    if is_keyword(str) {
        Ident::new_raw(str, Span::call_site())
    } else {
        Ident::new(str, Span::call_site())
    }
}

pub fn to_doc_comment(text: &str) -> Vec<TokenStream> {
    // We find code blocks and add text annotations if they have no annotations lest they be parsed
    // as rust doc strings
    //
    // m - multiline match
    // s - `.` matches the newline characters too
    // .*? non-greedy match
    let re = Regex::new(r"(?ms)^```\s*$(.*?^```)").unwrap();
    let text = re.replace_all(text, "```text$1");

    text.lines()
        .map(|line| {
            let line = format!(" {}", line.trim_end());
            quote! { #[doc = #line] }
        })
        .collect::<Vec<_>>()
}

pub fn property_type(
    property: &Property,
    models: &BTreeMap<String, Model>,
    module: &syn::Path,
) -> Result<TokenStream, Error> {
    let model_type = model_type(&property.model, models, module);
    if property.required {
        model_type
    } else {
        let model_type = model_type?;
        Ok(quote! { Option<#model_type> })
    }
}

pub fn model_type(
    model: &Model,
    models: &BTreeMap<String, Model>,
    module: &syn::Path,
) -> Result<TokenStream, Error> {
    Ok(match &model.data {
        ModelData::String => quote! { String },
        ModelData::Integer(format) => format.as_token_stream(),
        ModelData::Number => quote! { f64 },
        ModelData::Boolean => quote! { bool },
        ModelData::Array(items) => {
            let items = model_type(items, models, module)?;
            quote! { Vec<#items> }
        }
        ModelData::ArbitraryValue => quote! { serde_json::Value },
        ModelData::NoValue => quote! { () },
        ModelData::Object(_) => {
            let struct_name = struct_name(&model.name);
            quote! { #module::models::#struct_name }
        }
        ModelData::HashMap(value, nullable) => {
            let mut value = model_type(value, models, module)?;
            if *nullable {
                value = quote! { Option<#value> }
            }
            quote! { std::collections::HashMap::<String, #value> }
        }
        ModelData::Ref(ref_str) => {
            if let Some(ref_model) = models.get(ref_str) {
                model_type(ref_model, models, module)?
            } else {
                Err(Error::MissingModelRef(ref_str.into()))?
            }
        }
        ModelData::Tarball => {
            quote! { bytes::Bytes }
        }
    })
}

pub fn or_default(default: &Option<XClientDefault>) -> TokenStream {
    if let Some(default) = default {
        match default {
            XClientDefault::String(string) => quote! { .or(Some(#string)) },
            XClientDefault::Boolean(bool) => quote! { .or(Some(#bool)) },
            XClientDefault::Integer(int) => quote! { .or(Some(#int)) },
        }
    } else {
        TokenStream::new()
    }
}

pub fn file_name(name: &str) -> String {
    format!("{}.rs", var_name(name))
}

const RUST_KEYWORDS: [&str; 51] = [
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", "async", "await", "dyn", "abstract", "become", "box", "do", "final", "macro",
    "override", "priv", "try", "typeof", "unsized", "virtual", "yield",
];
