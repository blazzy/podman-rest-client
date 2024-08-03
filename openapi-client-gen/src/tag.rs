use std::collections::BTreeMap;
use std::fmt;

use convert_case::{Case, Casing};
use yaml_rust2::Yaml;

use crate::error::Error;
use crate::model::Model;
use crate::parse;

pub struct Tag {
    pub description: String,
    pub safe_name: String,
    pub operations: Vec<Operation>,
}

impl Tag {
    pub fn var_name(&self) -> String {
        self.safe_name.to_case(Case::Snake)
    }

    pub fn struct_name(&self) -> String {
        self.safe_name.to_case(Case::UpperCamel)
    }

    pub fn file_name(&self) -> String {
        format!("{}.rs", self.var_name())
    }
}

pub struct Operation {
    pub path: String,
    pub method: Method,
    pub operation_id: String,
    pub description: Option<String>,
    pub summary: Option<String>,
    pub responses: BTreeMap<String, Model>,
    pub params: Vec<Parameter>,
}

impl Operation {
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

pub struct Parameter {
    pub name: String,
    pub spec_name: String,
    pub description: Option<String>,
    pub request_part: RequestPart,
}

impl Parameter {
    pub fn type_string(&self, models: &BTreeMap<String, Model>) -> String {
        match &self.request_part {
            RequestPart::Path(field) | RequestPart::Query(field) | RequestPart::Header(field) => {
                let base_type = match &field.r#type {
                    Type::Just(base_type) => base_type.to_string(),
                    Type::Array(base_type) => format!("Vec<{}>", base_type),
                };
                if field.required {
                    base_type
                } else {
                    format!("Option<{}>", base_type)
                }
            }
            RequestPart::Body(model) => model.type_string(models),
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

    pub fn is_query(&self) -> bool {
        matches!(self.request_part, RequestPart::Query(_))
    }

    pub fn is_path(&self) -> bool {
        matches!(self.request_part, RequestPart::Path(_))
    }

    pub fn is_header(&self) -> bool {
        matches!(self.request_part, RequestPart::Header(_))
    }

    pub fn is_body(&self) -> bool {
        matches!(self.request_part, RequestPart::Body(_))
    }

    pub fn is_required(&self) -> bool {
        match &self.request_part {
            RequestPart::Path(field) | RequestPart::Query(field) | RequestPart::Header(field) => {
                field.required
            }
            RequestPart::Body(_) => true,
        }
    }

    pub fn to_string_string(&self) -> String {
        match &self.request_part {
            RequestPart::Path(field) | RequestPart::Query(field) | RequestPart::Header(field) => {
                match &field.r#type {
                    Type::Just(_) => format!("{}.to_string()", self.var_name()),
                    Type::Array(_) => format!(
                        "{}.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(\",\")",
                        self.var_name()
                    ),
                }
            }
            RequestPart::Body(_) => "TODO!".into(),
        }
    }
}

pub enum RequestPart {
    Path(RequestPartField),
    Query(RequestPartField),
    Header(RequestPartField),
    Body(Model),
}

pub struct RequestPartField {
    r#type: Type,
    required: bool,
}

impl TryFrom<&Yaml> for RequestPartField {
    type Error = Error;

    fn try_from(value: &Yaml) -> Result<Self, Self::Error> {
        Ok(RequestPartField {
            r#type: Type::try_from(value)?,
            required: value["required"].as_bool().unwrap_or(false),
        })
    }
}

pub enum BaseType {
    String,
    Boolean,
    Integer,
}

impl std::fmt::Display for BaseType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use BaseType::*;
        match self {
            String => write!(f, "&str"),
            Boolean => write!(f, "bool"),
            Integer => write!(f, "i64"),
        }
    }
}

impl TryFrom<&Yaml> for BaseType {
    type Error = Error;

    fn try_from(yaml: &Yaml) -> Result<Self, Self::Error> {
        let value: &str = parse::string(yaml, "request part type")?;

        match value {
            "string" => Ok(BaseType::String),
            "boolean" => Ok(BaseType::Boolean),
            "integer" => Ok(BaseType::Integer),
            _ => Err(Error::UnrecognizedRequestPartType(value.into())),
        }
    }
}

pub enum Type {
    Just(BaseType),
    Array(BaseType),
}

impl TryFrom<&Yaml> for Type {
    type Error = Error;

    fn try_from(yaml: &Yaml) -> Result<Self, Self::Error> {
        let value: &str = parse::string(&yaml["type"], "request part type")?;

        match value {
            "array" => {
                let items = &yaml["items"]["type"];
                Ok(Type::Array(items.try_into()?))
            }
            _ => Ok(Type::Just(BaseType::try_from(&yaml["type"])?)),
        }
    }
}

impl RequestPart {
    pub fn from_yaml(
        yaml: &Yaml,
        base_name: &str,
        path_ref: String,
        models: &mut BTreeMap<String, Model>,
    ) -> Result<Self, Error> {
        let part_name = parse::string(&yaml["in"], "parameter in:")?;

        match part_name {
            "body" => {
                let schema = &yaml["schema"];
                let model_ref = format!("{}/{}", path_ref, "body");
                let model = Model::new(format!("{}Body", base_name), schema, &model_ref, models)?;
                Ok(RequestPart::Body(model))
            }
            "path" => Ok(RequestPart::Path(yaml.try_into()?)),
            "header" => Ok(RequestPart::Header(yaml.try_into()?)),
            "query" => Ok(RequestPart::Query(yaml.try_into()?)),
            _ => Err(Error::UnrecognizedRequestPart(part_name.into())),
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
