use std::collections::BTreeMap;
use std::fmt;

use crate::error::Error;
use crate::model::Model;
use crate::parameter::{BodyParameter, Parameter};

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

    pub fn success_response(&self) -> Option<(u16, &Model)> {
        self.responses
            .iter()
            .map(|(code, model)| (code.parse().unwrap_or(0), model))
            .find(|(code, _)| (200..300).contains(code) || code == &101)
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
