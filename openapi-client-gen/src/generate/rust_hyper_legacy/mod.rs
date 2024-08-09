use std::path::Path;

use askama::Template;

use crate::error::Error;
use crate::file_tracker::FileTracker;
use crate::model::ModelData;
use crate::spec::Spec;
use crate::template;

mod templates;

pub fn generate(spec: &Spec, files: &mut FileTracker) -> Result<(), Error> {
    let apis_path = Path::new("src/apis");
    for tag in spec.tags.values() {
        files.create(
            apis_path.join(tag.file_name()),
            templates::apis::api(spec, tag)?,
        )?;
    }
    files.create(
        apis_path.join("mod.rs"),
        &templates::mod_pub_structs(spec.tags.values().map(|tag| &tag.safe_name))?,
    )?;

    let models = spec.object_models();
    let models_path = Path::new("src/models");
    for model in models.values() {
        if let ModelData::Object(properties) = &model.data {
            let template = template::ModelTemplate {
                model,
                properties,
                models: &spec.models,
            };
            files.create(models_path.join(model.file_name()), template.render()?)?;
        }
    }
    files.create(
        models_path.join("mod.rs"),
        &templates::mod_pub_structs(models.values().map(|model| &model.name))?,
    )?;

    let mut operations = Vec::new();
    for tag in spec.tags.values() {
        for operation in &tag.operations {
            if !operation.header_params.is_empty() || !operation.query_params.is_empty() {
                operations.push(operation);
            }
        }
    }

    let params_path = Path::new("src/params");
    for operation in &operations {
        if !operation.header_params.is_empty() || !operation.query_params.is_empty() {
            let template = template::ParamsTemplate { operation };
            files.create(params_path.join(operation.file_name()), &template.render()?)?;
        }
    }
    files.create(
        params_path.join("mod.rs"),
        templates::mod_pub_structs(operations.iter().map(|op| &op.name))?,
    )?;

    files.create("src/client.rs", &templates::client::client(spec)?)?;

    files.create("src/config.rs", templates::config::config(spec)?)?;

    files.create("src/lib.rs", include_str!("./templates/lib.rs"))?;
    files.create("src/error.rs", include_str!("./templates/error.rs"))?;
    files.create("src/request.rs", include_str!("./templates/request.rs"))?;

    Ok(())
}
