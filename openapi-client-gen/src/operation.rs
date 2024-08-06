use std::collections::BTreeMap;
use std::fmt;

use convert_case::{Case, Casing};
use regex::Regex;

use crate::error::Error;
use crate::model::Model;
use crate::parameter::{BodyParameter, Parameter};
use crate::parse;

#[derive(Default)]
pub struct Operation {
    pub path: String,
    pub method: Method,
    pub name: String,
    pub description: Option<String>,
    pub summary: Option<String>,
    pub responses: BTreeMap<String, Model>,
    pub path_params: Vec<Parameter>,
    pub query_params: Vec<Parameter>,
    pub header_params: Vec<Parameter>,
    pub body_param: Option<BodyParameter>,
}

impl Operation {
    /// We need to match code blocks and add text annotations if to them if they have no
    /// annotations lest the be parsed as rust doc strings
    pub fn clean_description(&self) -> String {
        if let Some(description) = &self.description {
            // m - multiline match
            // s - cause `.` match the newline characters too
            // .*? non-greedy match
            let re = Regex::new(r"(?ms)^```\s*$(.*?^```)").unwrap();

            re.replace_all(description, "```text$1").to_string()
        } else {
            "".into()
        }
    }

    pub fn var_name(&self) -> String {
        let var_name = self.name.to_case(Case::Snake);
        if parse::is_keyword(&var_name) {
            format!("r#{}", var_name)
        } else {
            var_name
        }
    }

    pub fn params_struct_has_str(&self) -> bool {
        let predicate = |param: &Parameter| param.r#type.has_string();
        self.header_params.iter().any(predicate) || self.query_params.iter().any(predicate)
    }

    pub fn should_use_params_struct(&self) -> bool {
        !self.header_params.is_empty() || !self.query_params.is_empty()
    }

    pub fn is_optional_params_struct(&self) -> bool {
        let predicate = |param: &Parameter| param.required;
        !(self.header_params.iter().any(predicate) && self.query_params.iter().any(predicate))
    }

    pub fn struct_type(&self) -> String {
        let mut name = format!("super::super::params::{}", self.struct_name());
        if self.params_struct_has_str() {
            name = format!("{}<'a>", name);
        }

        if self.is_optional_params_struct() {
            format!("Option<{}>", name)
        } else {
            name
        }
    }

    pub fn struct_name(&self) -> String {
        self.name.to_case(Case::UpperCamel)
    }

    pub fn file_name(&self) -> String {
        format!("{}.rs", self.name.to_case(Case::Snake))
    }

    pub fn success_type(&self, models: &BTreeMap<String, Model>) -> String {
        let successes: Vec<_> = self
            .responses
            .iter()
            .filter(|(key, _)| {
                let code: u16 = key.parse().unwrap_or(0);
                (200..300).contains(&code)
            })
            .collect();

        if successes.len() != 1 {
            "serde_json::Value".into()
        } else {
            successes[0].1.type_string(models)
        }
    }
}

#[derive(Default)]
pub enum Method {
    #[default]
    Get,
    Post,
    Delete,
    Put,
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Get => write!(f, "GET"),
            Self::Put => write!(f, "GET"),
            Self::Post => write!(f, "POST"),
            Self::Delete => write!(f, "DELETE"),
        }
    }
}

impl TryFrom<&str> for Method {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "get" => Ok(Method::Get),
            "put" => Ok(Method::Put),
            "post" => Ok(Method::Post),
            "delete" => Ok(Method::Delete),
            _ => Err(Error::UnsupportedMethod(value.into())),
        }
    }
}
