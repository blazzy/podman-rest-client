use std::collections::BTreeMap;
use std::fs;

use askama::Template;
use clap::Parser;
use yaml_rust2::YamlLoader;

use error::ParseError;
use model::ModelData;

mod error;
mod model;
mod parse;
mod spec;
mod tag;
mod template;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    input_spec_file_path: String,
    target_directory: String,
}

fn main() -> Result<(), ParseError> {
    colog::init();

    let cli = Cli::parse();

    let contents = fs::read_to_string(cli.input_spec_file_path)?;

    if let Ok(yaml) = YamlLoader::load_from_str(&contents) {
        let spec = spec::Spec::from_yaml(&yaml[0])?;

        let models = spec.object_models();

        let params: Result<BTreeMap<String, Vec<(String, String)>>, ParseError> = spec
            .params
            .iter()
            .map(|(key, params)| {
                let params: Result<Vec<(String, String)>, ParseError> = params
                    .iter()
                    .map(|param| Ok((param.var_name(), param.type_string(&spec.models)?)))
                    .collect();
                Ok((key.clone(), params?))
            })
            .collect();
        let params = params?;

        let api_path = format!("{}/api", cli.target_directory);
        fs::create_dir_all(api_path)?;
        for tag in spec.tags.values() {
            let path = format!("{}/api/{}", cli.target_directory, tag.file_name());
            let template = template::ApiTemplate {
                tag,
                params: &params,
                models: &spec.models,
            };
            fs::write(path, template.render()?)?;
        }

        let model_path = format!("{}/model", cli.target_directory);
        fs::create_dir_all(model_path)?;
        for model in models.values() {
            if let ModelData::Object(properties) = &model.data {
                let path = format!("{}/model/{}", cli.target_directory, model.file_name());
                let properties: Result<Vec<_>, ParseError> = properties
                    .iter()
                    .map(|property| {
                        Ok((
                            &property.model,
                            property.var_name(),
                            property.type_string(&spec.models)?,
                        ))
                    })
                    .collect();
                let template = template::ModelTemplate {
                    model,
                    properties: properties?,
                };
                fs::write(path, template.render()?)?;
            }
        }

        let client_path = format!("{}/client.rs", cli.target_directory);
        let client_template = template::ClientTemplate { tags: &spec.tags };
        fs::write(client_path, client_template.render()?)?;

        let config_path = format!("{}/config.rs", cli.target_directory);
        let config_template = template::ConfigTemplate { spec: &spec };
        fs::write(config_path, config_template.render()?)?;

        let api_mod_path = format!("{}/api/mod.rs", cli.target_directory);
        let api_mod_template = template::ApiModTemplate { tags: &spec.tags };
        fs::write(api_mod_path, api_mod_template.render()?)?;

        let model_mod_path = format!("{}/model/mod.rs", cli.target_directory);
        let model_mod_template = template::ModelModTemplate { models: &models };
        fs::write(model_mod_path, model_mod_template.render()?)?;

        let mod_path = format!("{}/mod.rs", cli.target_directory);
        let mod_template = template::ModTemplate;
        fs::write(mod_path, mod_template.render()?)?;

        let error_path = format!("{}/error.rs", cli.target_directory);
        let error_template = fs::read_to_string("templates/error.rs")?;
        fs::write(error_path, error_template)?;
    }

    Ok(())
}
