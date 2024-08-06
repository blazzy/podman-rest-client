use std::fs;
use std::path::PathBuf;

use askama::Template;
use clap::Parser;
use error::Error;
use model::ModelData;

mod error;
mod generate;
mod model;
mod operation;
mod parameter;
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

fn main() -> Result<(), Error> {
    colog::init();

    let cli = Cli::parse();

    let contents = fs::read_to_string(cli.input_spec_file_path)?;

    let target_directory = PathBuf::from(cli.target_directory);
    let target_path = |name: &str| target_directory.join(name);

    let spec = spec::Spec::from_yaml_string(&contents)?;

    let models = spec.object_models();

    let api_path = target_path("apis");
    fs::create_dir_all(&api_path)?;
    for tag in spec.tags.values() {
        let template = template::ApiTemplate {
            tag,
            models: &spec.models,
        };
        fs::write(api_path.join(tag.file_name()), template.render()?)?;
    }

    let model_path = target_path("models");
    fs::create_dir_all(&model_path)?;
    for model in models.values() {
        if let ModelData::Object(properties) = &model.data {
            let template = template::ModelTemplate {
                model,
                properties,
                models: &spec.models,
            };
            fs::write(model_path.join(model.file_name()), template.render()?)?;
        }
    }

    let mut operations = Vec::new();
    for tag in spec.tags.values() {
        for operation in &tag.operations {
            if !operation.header_params.is_empty() || !operation.query_params.is_empty() {
                operations.push(operation);
            }
        }
    }

    let params_path = target_path("params");
    fs::create_dir_all(&params_path)?;
    for operation in &operations {
        if !operation.header_params.is_empty() || !operation.query_params.is_empty() {
            let template = template::ParamsTemplate { operation };
            fs::write(params_path.join(operation.file_name()), template.render()?)?;
        }
    }

    fs::write(target_path("client.rs"), generate::client(&spec)?)?;

    let config_template = template::ConfigTemplate { spec: &spec };
    fs::write(target_path("config.rs"), config_template.render()?)?;

    fs::write(
        target_path("models/mod.rs"),
        generate::mod_pub_structs(models.values().map(|model| &model.name))?,
    )?;

    fs::write(
        target_path("apis/mod.rs"),
        generate::mod_pub_structs(spec.tags.values().map(|tag| &tag.safe_name))?,
    )?;

    fs::write(
        target_path("params/mod.rs"),
        generate::mod_pub_structs(operations.iter().map(|op| &op.name))?,
    )?;

    let mod_template = template::ModTemplate;
    fs::write(target_path("lib.rs"), mod_template.render()?)?;

    let error_template = include_str!("../templates/error.rs");
    fs::write(target_path("error.rs"), error_template)?;

    let request_template = include_str!("../templates/request.rs");
    fs::write(target_path("request.rs"), request_template)?;

    Ok(())
}
