use std::collections::BTreeMap;

use hashlink::LinkedHashMap;
use yaml_rust2::Yaml;
use yaml_rust2::YamlLoader;

use crate::error::Error;
use crate::model::{Model, ModelData};
use crate::operation::{Method, Operation};
use crate::parameter::BodyParameter;
use crate::parse;
use crate::tag::Tag;

#[derive(Default)]
pub struct Spec {
    config: Config,
    pub base_path: String,
    pub models: BTreeMap<String, Model>,
    pub tags: BTreeMap<String, Tag>,
}

#[derive(Default, Clone)]
pub struct Config {
    pub hash_maps_always_nullable: bool,
}

impl Spec {
    pub fn from_yaml_string(string: &str, config: &Config) -> Result<Spec, Error> {
        let yaml = YamlLoader::load_from_str(string)?;
        Spec::from_yaml(&yaml[0], config)
    }

    pub fn from_yaml(yaml: &Yaml, config: &Config) -> Result<Spec, Error> {
        let mut spec = Spec {
            config: config.clone(),
            ..Spec::default()
        };

        let host: String = parse::string(&yaml["host"], "host")?;
        let base_path: String = parse::string(&yaml["basePath"], "basePath").unwrap_or("/".into());
        let scheme: String = parse::string(&yaml["schemes"][0], "host").unwrap_or("https".into());

        spec.base_path = format!("{}://{}{}", scheme, host, base_path);

        if let Some(tags) = yaml["tags"].as_vec() {
            spec.load_tags(tags)?;
        }
        log::info!("{} tags found", spec.tags.len());

        log::info!("looking for definitions");
        if let Some(models) = yaml["definitions"].as_hash() {
            spec.load_definition_models(models)?;
        }
        log::info!("{} models", spec.models.len());

        log::info!("looking for responses");
        if let Some(models) = yaml["responses"].as_hash() {
            spec.load_response_models(models)?;
        }
        log::info!("{} models", spec.models.len());

        log::info!("looking for paths");
        if let Some(paths) = yaml["paths"].as_hash() {
            spec.load_paths(paths)?;
        }
        log::info!("{} models", spec.models.len());

        Ok(spec)
    }

    fn load_tags(&mut self, yaml: &Vec<Yaml>) -> Result<(), Error> {
        for tag in yaml {
            let name: String = parse::string(&tag["name"], "tag name")?;
            let description: String = parse::string(&tag["description"], "tag description")?;

            self.tags.insert(
                name.clone(),
                Tag {
                    name,
                    description,
                    operations: Vec::new(),
                },
            );
        }
        Ok(())
    }

    fn load_response_models(&mut self, yaml: &LinkedHashMap<Yaml, Yaml>) -> Result<(), Error> {
        for (key, value) in yaml {
            let name: String = parse::string(key, "response name")?;
            let model_ref = format!("#/responses/{}", name);
            Model::new(
                name,
                &value["schema"],
                &model_ref,
                &mut self.models,
                &self.config,
            )?;
        }
        Ok(())
    }

    fn load_definition_models(&mut self, yaml: &LinkedHashMap<Yaml, Yaml>) -> Result<(), Error> {
        for (key, value) in yaml {
            let name: String = parse::string(key, "definition name")?;
            let model_ref = format!("#/definitions/{}", name);
            Model::new(name, value, &model_ref, &mut self.models, &self.config)?;
        }
        Ok(())
    }

    fn load_paths(&mut self, yaml: &LinkedHashMap<Yaml, Yaml>) -> Result<(), Error> {
        for (key, value) in yaml {
            let path: String = parse::string(key, "path key")?;

            if let Some(operations) = value.as_hash() {
                for (operation, spec) in operations {
                    let method: Method = parse::try_string(operation, "missing path method")?;

                    let tag: String = parse::string(&spec["tags"][0], "Bad or missing tag")?;

                    let operation_id: &str =
                        parse::string(&spec["operationId"], "Bad or missing operation Id")?;

                    let mut operation = Operation {
                        path: path.clone(),
                        name: operation_id.to_string(),
                        method,
                        description: parse::maybe_string(&spec["description"]),
                        summary: parse::maybe_string(&spec["summary"]),
                        ..Default::default()
                    };

                    if let Some(yaml_params) = spec["parameters"].as_vec() {
                        for param in yaml_params {
                            let part_name = parse::string(&param["in"], "parameter in:")?;
                            match part_name {
                                "path" => operation.path_params.push(param.try_into()?),
                                "query" => operation.query_params.push(param.try_into()?),
                                "header" => operation.header_params.push(param.try_into()?),
                                "body" => {
                                    operation.body_param = Some(BodyParameter::from_yaml(
                                        param,
                                        operation_id,
                                        format!("#paths/{}/parameters", operation_id),
                                        &mut self.models,
                                        &self.config,
                                    )?)
                                }
                                _ => return Err(Error::UnrecognizedRequestPart(part_name.into())),
                            }
                        }
                    }

                    let mut produces = &Vec::<Yaml>::new();
                    produces = spec["produces"].as_vec().unwrap_or(produces);
                    if produces.is_empty() {
                        log::warn!("No response format for operation {}", operation_id);
                    }
                    if produces.len() > 1 {
                        log::warn!(
                            "Multiple response formats for operation {}. Choosing the first one",
                            operation_id
                        );
                    }

                    let produces = produces
                        .first()
                        .map(|y| y.as_str().unwrap())
                        .unwrap_or("text/plain");

                    if let Some(responses) = spec["responses"].as_hash() {
                        for (code, response) in responses {
                            let code: String = parse::string(code, "response code")?;
                            let yaml = if response["schema"].as_hash().is_some() {
                                &response["schema"]
                            } else {
                                response
                            };
                            let name = format!("{}_{}", operation_id, code);
                            let model = match produces {
                                "application/x-tar" => Model {
                                    name,
                                    title: None,
                                    description: None,
                                    data: ModelData::Tarball,
                                },
                                "application/octet-stream" => Model {
                                    name,
                                    title: None,
                                    description: None,
                                    data: ModelData::Bytes,
                                },
                                "text/plain" => Model {
                                    name,
                                    title: None,
                                    description: None,
                                    data: ModelData::PlainString,
                                },
                                "text/vnd.yaml" => Model {
                                    name,
                                    title: None,
                                    description: None,
                                    data: ModelData::PlainString,
                                },
                                "application/json" => Model::new(
                                    name,
                                    yaml,
                                    &format!("#paths/{}/responses/{}", operation_id, code),
                                    &mut self.models,
                                    &self.config,
                                )?,
                                _ => panic!("Unsupported content-type {}", produces),
                            };
                            operation.responses.insert(code.clone(), model);
                        }
                    }

                    self.tags
                        .get_mut(&tag)
                        .ok_or(Error::UnrecognizedTag(tag))?
                        .operations
                        .push(operation);
                }
            } else {
                log::warn!("No methods/operations found for path {}", path);
            }
        }
        Ok(())
    }

    pub fn object_models(&self) -> BTreeMap<&String, &Model> {
        self.models
            .iter()
            .filter(|(_, model)| model.is_object())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn builds_base_path() {
        let spec = Spec::from_yaml_string(
            indoc! {r#"
            basePath: /
            host: example.com
            schemes:
                - http
                - https
            "#},
            &Config::default(),
        )
        .unwrap();
        assert_eq!(spec.base_path, "http://example.com/");
    }
}
