use std::collections::BTreeMap;
use std::collections::HashSet;

use convert_case::{Case, Casing};
use hashlink::LinkedHashMap;
use yaml_rust2::Yaml;
use yaml_rust2::YamlLoader;

use crate::error::Error;
use crate::model::Model;
use crate::parse;
use crate::tag;

#[derive(Default)]
pub struct Spec {
    pub base_path: String,
    pub models: BTreeMap<String, Model>,
    pub tags: BTreeMap<String, tag::Tag>,
}

impl Spec {
    pub fn from_yaml_string(string: &str) -> Result<Spec, Error> {
        let yaml = YamlLoader::load_from_str(&string)?;
        Spec::from_yaml(&yaml[0])
    }

    pub fn from_yaml(yaml: &Yaml) -> Result<Spec, Error> {
        let mut spec = Spec::default();

        let host: String = parse::string(&yaml["host"], "host")?;
        let base_path: String = parse::string(&yaml["host"], "host").unwrap_or("/".into());
        let scheme: String = parse::string(&yaml["host"][0], "host").unwrap_or("https".into());

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

            let safe_name = name
                .chars()
                .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
                .collect();

            self.tags.insert(
                name,
                tag::Tag {
                    safe_name,
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
            Model::new(name, &value["schema"], &model_ref, &mut self.models)?;
        }
        Ok(())
    }

    fn load_definition_models(&mut self, yaml: &LinkedHashMap<Yaml, Yaml>) -> Result<(), Error> {
        for (key, value) in yaml {
            let name: String = parse::string(key, "definition name")?;
            let model_ref = format!("#/definitions/{}", name);
            Model::new(name, value, &model_ref, &mut self.models)?;
        }
        Ok(())
    }

    fn load_paths(&mut self, yaml: &LinkedHashMap<Yaml, Yaml>) -> Result<(), Error> {
        for (key, value) in yaml {
            let path: String = parse::string(key, "path key")?;

            if let Some(operations) = value.as_hash() {
                for (operation, spec) in operations {
                    let method: tag::Method = parse::try_string(operation, "missing path method")?;

                    let tag: String = parse::string(&spec["tags"][0], "Bad or missing tag")?;

                    let operation_id: &str =
                        parse::string(&spec["operationId"], "Bad or missing operation Id")?;
                    let operation_id = operation_id.to_case(Case::Snake);

                    let mut params = Vec::new();
                    if let Some(yaml_params) = spec["parameters"].as_vec() {
                        let mut names = HashSet::<String>::new();
                        for param in yaml_params {
                            let mut name: String = parse::string(&param["name"], "parameter name")?;
                            // Ensure the name is unique among the params
                            let spec_name = name.clone();
                            let mut count = 0;
                            while names.contains(&name) {
                                count += 1;
                                name = format!("{}_{}", name, count);
                            }

                            names.insert(name.clone());
                            params.push(tag::Parameter {
                                name,
                                spec_name,
                                description: parse::maybe_string(&param["description"]),
                                request_part: tag::RequestPart::from_yaml(
                                    param,
                                    &operation_id,
                                    format!("#paths/{}/parameters", operation_id),
                                    &mut self.models,
                                )?,
                            });
                        }
                    }

                    let mut operation = tag::Operation {
                        path: path.clone(),
                        operation_id: operation_id.clone(),
                        method,
                        description: parse::maybe_string(&spec["description"]),
                        summary: parse::maybe_string(&spec["summary"]),
                        responses: BTreeMap::new(),
                        params,
                    };

                    if let Some(responses) = spec["responses"].as_hash() {
                        for (code, response) in responses {
                            let code: String = parse::string(code, "response code")?;
                            let yaml = if response["schema"].as_hash().is_some() {
                                &response["schema"]
                            } else {
                                response
                            };
                            operation.responses.insert(
                                code.clone(),
                                Model::new(
                                    format!("{}_{}", operation_id, code),
                                    yaml,
                                    &format!("#paths/{}/responses/{}", operation_id, code),
                                    &mut self.models,
                                )?,
                            );
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
