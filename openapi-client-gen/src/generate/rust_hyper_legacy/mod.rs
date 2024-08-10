use std::path::Path;

use askama::Template;

use crate::error::Error;
use crate::file_tracker::FileTracker;
use crate::lang::rust;
use crate::model::ModelData;
use crate::spec::Spec;
use crate::template;

mod templates;

pub fn generate(spec: &Spec, files: &mut FileTracker, is_module: bool) -> Result<(), Error> {
    let src_dir = if is_module {
        Path::new("")
    } else {
        Path::new("src")
    };
    let apis_dir = src_dir.join("apis");
    let models_dir = src_dir.join("models");
    let params_dir = src_dir.join("params");

    let lib_file_name = if is_module {
        src_dir.join("mod.rs")
    } else {
        src_dir.join("lib.rs")
    };
    files.create(lib_file_name, include_str!("./templates/lib.rs"))?;

    for tag in spec.tags.values() {
        let file_name = apis_dir.join(rust::file_name(&tag.name));
        files.create(file_name, templates::apis::api(spec, tag)?)?;
    }
    files.create(
        apis_dir.join("mod.rs"),
        &templates::mod_pub_structs(spec.tags.values().map(|tag| &tag.name))?,
    )?;

    let models = spec.object_models();
    for model in models.values() {
        if let ModelData::Object(properties) = &model.data {
            let template = template::ModelTemplate {
                model,
                properties,
                models: &spec.models,
            };
            files.create(models_dir.join(model.file_name()), template.render()?)?;
        }
    }
    files.create(
        models_dir.join("mod.rs"),
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

    for operation in &operations {
        if !operation.header_params.is_empty() || !operation.query_params.is_empty() {
            let template = template::ParamsTemplate { operation };
            files.create(params_dir.join(operation.file_name()), &template.render()?)?;
        }
    }
    files.create(
        params_dir.join("mod.rs"),
        templates::mod_pub_structs(operations.iter().map(|op| &op.name))?,
    )?;

    files.create(src_dir.join("client.rs"), templates::client::client(spec)?)?;
    files.create(src_dir.join("config.rs"), templates::config::config(spec)?)?;

    let file_name = src_dir.join("error.rs");
    files.create(file_name, include_str!("./templates/error.rs"))?;
    let file_name = src_dir.join("request.rs");
    files.create(file_name, include_str!("./templates/request.rs"))?;

    Ok(())
}
