use std::collections::{BTreeMap, HashSet};

use convert_case::{Case, Casing};
use yaml_rust2::Yaml;

use crate::error::Error;
use crate::parse;

#[derive(Clone)]
pub struct Model {
    pub name: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub data: ModelData,
}

#[derive(Clone)]
pub enum ModelData {
    Object(Vec<Property>),
    String,
    Integer(IntegerFormat),
    Number,
    Boolean,
    Array(Box<Model>),
    HashMap(Box<Model>, bool),
    ArbitraryValue,
    NoValue,
    Ref(String),
}

#[derive(Clone)]
pub enum IntegerFormat {
    INT64,
    UINT64,
    INT32,
    UINT32,
    INT16,
    UINT16,
    INT8,
    UINT8,
}

impl TryFrom<&str> for IntegerFormat {
    type Error = Error;

    fn try_from(value: &str) -> Result<IntegerFormat, Self::Error> {
        use IntegerFormat::*;
        match value {
            "int64" => Ok(INT64),
            "uint64" => Ok(UINT64),
            "int32" => Ok(INT32),
            "uint32" => Ok(UINT32),
            "int16" => Ok(INT16),
            "uint16" => Ok(UINT16),
            "int8" => Ok(INT8),
            "uint8" => Ok(UINT8),
            _ => Err(Error::UnrecognizedIntegerFormat(value.into())),
        }
    }
}

impl std::fmt::Display for IntegerFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use IntegerFormat::*;
        match self {
            INT64 => write!(f, "i64"),
            UINT64 => write!(f, "u64"),
            INT32 => write!(f, "i32"),
            UINT32 => write!(f, "u32"),
            INT16 => write!(f, "i16"),
            UINT16 => write!(f, "u16"),
            INT8 => write!(f, "i8"),
            UINT8 => write!(f, "u8"),
        }
    }
}

#[derive(Clone)]
pub struct Property {
    pub name: String,
    pub model: Model,
    required: bool,
}

impl Property {
    pub fn safe_name(&self) -> String {
        self.name
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
            .collect()
    }

    pub fn var_name(&self) -> String {
        let var_name = self.safe_name().to_case(Case::Snake);
        if parse::is_keyword(&var_name) {
            format!("r#{}", var_name)
        } else {
            var_name
        }
    }

    pub fn type_string(&self, models: &BTreeMap<String, Model>) -> String {
        if self.required {
            self.model.type_string(models)
        } else {
            format!("Option<{}>", self.model.type_string(models))
        }
    }
}

impl Model {
    pub fn new(
        name: String,
        yaml: &Yaml,
        model_ref: &String,
        models: &mut BTreeMap<String, Model>,
    ) -> Result<Model, Error> {
        let data: ModelData = from_yaml(yaml, &name, model_ref, models)?;
        let model = Model {
            name,
            title: yaml["title"].as_str().map(|d| d.to_string()),
            description: yaml["description"].as_str().map(|d| d.to_string()),
            data,
        };
        let model_ret = if model.is_ref() {
            model.clone()
        } else {
            Model::new_ref(model_ref.to_string(), model_ref.clone(), yaml)
        };
        models.insert(model_ref.clone(), model);
        Ok(model_ret)
    }

    pub fn new_ref(name: String, model_ref: String, yaml: &Yaml) -> Model {
        Model {
            name,
            title: yaml["title"].as_str().map(|d| d.to_string()),
            description: yaml["description"].as_str().map(|d| d.to_string()),
            data: ModelData::Ref(model_ref),
        }
    }

    pub fn safe_name(&self) -> String {
        self.name
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
            .collect()
    }

    pub fn var_name(&self) -> String {
        let var_name = self.safe_name().to_case(Case::Snake);
        if parse::is_keyword(&var_name) {
            format!("r#{}", var_name)
        } else {
            var_name
        }
    }

    pub fn struct_name(&self) -> String {
        self.safe_name().to_case(Case::UpperCamel)
    }

    pub fn file_name(&self) -> String {
        format!("{}.rs", self.name.to_case(Case::Snake))
    }

    pub fn is_object(&self) -> bool {
        matches!(self.data, ModelData::Object(_))
    }

    pub fn is_ref(&self) -> bool {
        matches!(self.data, ModelData::Ref(_))
    }

    pub fn type_string(&self, models: &BTreeMap<String, Model>) -> String {
        match &self.data {
            ModelData::String => "String".into(),
            ModelData::Integer(format) => format.to_string(),
            ModelData::Number => "f64".into(),
            ModelData::Boolean => "bool".into(),
            ModelData::Array(items) => format!("Vec<{}>", items.type_string(models)),
            ModelData::ArbitraryValue => "serde_json::Value".into(),
            ModelData::NoValue => "()".into(),
            ModelData::Object(_) => {
                format!("super::super::models::{}", self.struct_name())
            }
            ModelData::HashMap(value, nullable) => {
                let value = value.type_string(models);
                format!(
                    "std::collections::HashMap::<String, {}>",
                    if *nullable {
                        format!("Option<{}>", value)
                    } else {
                        value
                    }
                )
            }
            ModelData::Ref(ref_str) => {
                if let Some(ref_model) = models.get(ref_str) {
                    ref_model.type_string(models)
                } else {
                    log::warn!("Could not find model reference: {}", ref_str,);
                    "serde_json::Value".into()
                }
            }
        }
    }
}

fn from_yaml(
    yaml: &Yaml,
    parent_name: &String,
    model_ref: &String,
    models: &mut BTreeMap<String, Model>,
) -> Result<ModelData, Error> {
    if !yaml["$ref"].is_badvalue() && !yaml["$ref"].is_null() {
        return Ok(ModelData::Ref(parse::string(
            &yaml["$ref"],
            "definition $ref",
        )?));
    }

    if yaml["type"].is_badvalue() || yaml["type"].is_null() {
        return Ok(ModelData::NoValue);
    }

    let type_name = yaml["type"]
        .as_str()
        .ok_or(Error::StringParse("definition type".into()))?;

    match type_name {
        "object" => {
            let additional_properties = &yaml["additionalProperties"];

            if !additional_properties.is_null() && !additional_properties.is_badvalue() {
                let nullable = Some(true) == additional_properties["nullable"].as_bool()
                    || Some(true) == additional_properties["x-nullable"].as_bool();

                return Ok(ModelData::HashMap(
                    Box::new(Model::new(
                        "additionalProperties".into(),
                        additional_properties,
                        &format!("{}/{}", model_ref, "additionalProperties"),
                        models,
                    )?),
                    nullable,
                ));
            }

            if let Some(yaml_props) = yaml["properties"].as_hash() {
                let mut properties = Vec::new();
                let required_props: HashSet<String> = yaml["required"]
                    .as_vec()
                    .map(|vec| {
                        vec.iter()
                            .map(|yaml| parse::string(yaml, "required prop").unwrap_or("".into()))
                            .collect()
                    })
                    .unwrap_or_default();
                for (name, yaml) in yaml_props {
                    let name: String = parse::string(name, "property name")?;
                    let new_model = Model::new(
                        format!("{}_{}", parent_name, name),
                        yaml,
                        &format!("{}/{}", model_ref, name),
                        models,
                    )?;

                    properties.push(Property {
                        name: name.clone(),
                        model: new_model,
                        required: required_props.contains(&name),
                    });
                }
                return Ok(ModelData::Object(properties));
            }

            // An object without any properties is treated as an arbitrary json value.
            Ok(ModelData::ArbitraryValue)
        }
        "string" => Ok(ModelData::String),
        "integer" => {
            if let Some(format) = yaml["format"].as_str() {
                Ok(ModelData::Integer(format.try_into()?))
            } else {
                Ok(ModelData::Integer(IntegerFormat::INT64))
            }
        }
        "array" => {
            let items: Box<Model> = Box::new(Model::new(
                format!("{}{}", parent_name, "_items"),
                &yaml["items"],
                &format!("{}/{}", model_ref, "items"),
                models,
            )?);
            Ok(ModelData::Array(items))
        }
        "boolean" => Ok(ModelData::Boolean),
        "number" => Ok(ModelData::Number),
        _ => Err(Error::UnrecognizedModelType(type_name.into())),
    }
}
