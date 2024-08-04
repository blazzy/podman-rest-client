use std::collections::BTreeMap;
use std::fmt;

use convert_case::{Case, Casing};

use crate::error::Error;
use crate::model::Model;
use crate::parameter::Parameter;
use crate::parse;

pub struct Operation {
    pub path: String,
    pub method: Method,
    pub name: String,
    pub description: Option<String>,
    pub summary: Option<String>,
    pub responses: BTreeMap<String, Model>,
    pub params: Vec<Parameter>,
}

impl Operation {
    pub fn var_name(&self) -> String {
        let var_name = self.name.to_case(Case::Snake);
        if parse::is_keyword(&var_name) {
            format!("r#{}", var_name)
        } else {
            var_name
        }
    }

    pub fn path_params(&self) -> Vec<&Parameter> {
        self.filter_params(|param| param.is_path())
    }

    pub fn query_params(&self) -> Vec<&Parameter> {
        self.filter_params(|param| param.is_query())
    }

    pub fn header_params(&self) -> Vec<&Parameter> {
        self.filter_params(|param| param.is_header())
    }

    pub fn body_params(&self) -> Vec<&Parameter> {
        self.filter_params(|param| param.is_body())
    }

    pub fn filter_params(&self, predicate: fn(&Parameter) -> bool) -> Vec<&Parameter> {
        self.params.iter().filter(|p| predicate(p)).collect()
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

pub enum Method {
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
