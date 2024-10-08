use std::collections::BTreeMap;
use std::fmt;

use yaml_rust2::Yaml;

use crate::error::Error;
use crate::model::Model;
use crate::parse;
use crate::spec::Config;

pub struct Parameter {
    pub name: String,
    pub description: Option<String>,
    pub r#type: Type,
    pub required: bool,
    pub x_client_default: Option<XClientDefault>,
}

pub enum XClientDefault {
    Boolean(bool),
    String(String),
    Integer(i64),
}

impl XClientDefault {
    pub fn try_new(yaml: &Yaml, param_type: Type) -> Result<Option<XClientDefault>, Error> {
        let value = &yaml["x-client-default"];
        if value.is_null() || value.is_badvalue() {
            return Ok(None);
        }
        match param_type {
            Type::Just(base_type) => match base_type {
                BaseType::String => Ok(value
                    .as_str()
                    .map(|s| XClientDefault::String(s.to_string()))),
                BaseType::Boolean => Ok(value.as_bool().map(XClientDefault::Boolean)),
                BaseType::Integer => Ok(value.as_i64().map(XClientDefault::Integer)),
            },
            _ => Err(Error::UnsupportedXClientDefaultType),
        }
    }
}

impl TryFrom<&Yaml> for Parameter {
    type Error = Error;

    fn try_from(yaml: &Yaml) -> Result<Self, Error> {
        let r#type: Type = yaml.try_into()?;
        Ok(Parameter {
            name: parse::string(&yaml["name"], "parameter name")?,
            description: parse::maybe_string(&yaml["description"]),
            r#type,
            required: yaml["required"].as_bool().unwrap_or(false),
            x_client_default: XClientDefault::try_new(yaml, r#type)?,
        })
    }
}

#[derive(Copy, Clone)]
pub enum BaseType {
    String,
    Boolean,
    Integer,
}

impl fmt::Display for BaseType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

#[derive(Copy, Clone)]
pub enum Type {
    Just(BaseType),
    Array(BaseType),
}

impl Type {
    pub fn has_string(&self) -> bool {
        matches!(
            self,
            Type::Just(BaseType::String) | Type::Array(BaseType::String)
        )
    }
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

pub struct BodyParameter {
    pub name: String,
    pub description: Option<String>,
    pub model: Model,
}

impl BodyParameter {
    pub fn from_yaml(
        yaml: &Yaml,
        base_name: &str,
        path_ref: String,
        models: &mut BTreeMap<String, Model>,
        config: &Config,
    ) -> Result<Self, Error> {
        let schema = &yaml["schema"];
        let model_ref = format!("{}/{}", path_ref, "body");
        let model = Model::new(
            format!("{}Body", base_name),
            schema,
            &model_ref,
            models,
            config,
        )?;

        Ok(BodyParameter {
            name: parse::string(&yaml["name"], "parameter name")?,
            description: parse::maybe_string(&yaml["description"]),
            model,
        })
    }
}
