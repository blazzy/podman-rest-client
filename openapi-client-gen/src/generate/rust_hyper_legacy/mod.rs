use std::path::{Path, PathBuf};

use itertools::Itertools;

use crate::error::Error;
use crate::file_tracker::FileTracker;
use crate::lang::rust;
use crate::model::ModelData;
use crate::spec::Spec;

mod templates;

pub struct GeneratorConfig<'a> {
    pub spec: &'a Spec,
    pub is_module: bool,
    pub target_dir: &'a str,
    pub common_dir: Option<String>,
    pub api_module: Option<String>,
    pub common_module: Option<String>,
    pub skip_default_client: bool,
}

pub fn generate<'a>(config: &'a mut GeneratorConfig<'a>) -> Result<(), Error> {
    let mut files = FileTracker::new(config.target_dir);
    let mut common_files: FileTracker = if let Some(common_dir) = &config.common_dir {
        FileTracker::new(common_dir)
    } else {
        FileTracker::new(config.target_dir)
    };

    let common_module = guess_common_module(config).ok_or(Error::CouldNotDetermineCommonModule)?;
    let common_module: syn::Path = syn::parse_str(&common_module)?;

    let api_module_str = guess_api_module(config).ok_or(Error::CouldNotDetermineCommonModule)?;
    let api_module: syn::Path = syn::parse_str(&api_module_str)?;

    let spec = config.spec;

    let src_dir = if config.is_module {
        Path::new("")
    } else {
        Path::new("src")
    };
    let apis_dir = src_dir.join("apis");
    let models_dir = src_dir.join("models");
    let params_dir = src_dir.join("params");

    if !config.is_module {
        files.create(src_dir.join("lib.rs"), include_str!("./templates/lib.rs"))?;
    } else if config.common_dir.is_some() {
        common_files.create(
            src_dir.join("mod.rs"),
            include_str!("./templates/common_mod.rs"),
        )?;
        files.create(
            src_dir.join("mod.rs"),
            templates::api_specific_mod::render(config.skip_default_client)?,
        )?;
    } else {
        files.create(src_dir.join("mod.rs"), include_str!("./templates/lib.rs"))?;
    }

    for tag in spec.tags.values() {
        let file_name = apis_dir.join(rust::file_name(&tag.name));
        files.create(
            file_name,
            templates::apis::api(spec, tag, &common_module, &api_module)?,
        )?;
    }
    files.create(
        apis_dir.join("mod.rs"),
        &templates::mod_pub_structs(spec.tags.values().map(|tag| &tag.name))?,
    )?;

    let models = spec.object_models();
    for model in models.values() {
        if let ModelData::Object(properties) = &model.data {
            files.create(
                models_dir.join(rust::file_name(&model.name)),
                templates::models::models(model, properties, &spec.models, &api_module)?,
            )?;
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
            files.create(
                params_dir.join(rust::file_name(&operation.name)),
                templates::params::params(operation)?,
            )?;
        }
    }
    files.create(
        params_dir.join("mod.rs"),
        templates::mod_pub_structs(operations.iter().map(|op| &op.name))?,
    )?;

    files.create(
        src_dir.join("client.rs"),
        templates::client::client(
            spec,
            &common_module,
            &api_module_str,
            config.skip_default_client,
        )?,
    )?;

    common_files.create(src_dir.join("config.rs"), templates::config::config(spec)?)?;
    let file_name = src_dir.join("error.rs");
    common_files.create(file_name, include_str!("./templates/error.rs"))?;
    let file_name = src_dir.join("request.rs");
    common_files.create(file_name, include_str!("./templates/request.rs"))?;

    Ok(())
}

fn guess_api_module(config: &GeneratorConfig) -> Option<String> {
    if !config.is_module {
        return Some("crate".into());
    }

    if let Some(api_module) = &config.api_module {
        return Some(api_module.to_string());
    }

    guess_module_path(config.target_dir)
}

fn guess_common_module(config: &GeneratorConfig) -> Option<String> {
    if !config.is_module {
        return Some("crate".into());
    }

    if let Some(common_module) = &config.common_module {
        return Some(common_module.to_string());
    }

    if let Some(common_dir) = &config.common_dir {
        return guess_module_path(common_dir);
    }

    guess_module_path(config.target_dir)
}

/// Searches for the src directory and builds module name from path
fn guess_module_path<P: AsRef<str>>(start_dir: P) -> Option<String> {
    let mut path: PathBuf = start_dir.as_ref().into();
    let mut module_parts: Vec<String> = Vec::new();
    loop {
        let file_part = path.file_name()?.to_string_lossy();
        if file_part == "src" {
            module_parts.push("crate".to_string());
            module_parts.reverse();
            return Some(module_parts.iter().map(rust::safe_name).join("::"));
        }
        module_parts.push(file_part.to_string());
        path.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guess_module_path() {
        let res = guess_module_path("/home/blazzy/project/src/foo/bar");
        assert_eq!(res, Some("crate::foo::bar".into()));

        let res = guess_module_path("/home/blazzy/project/src");
        assert_eq!(res, Some("crate".into()));
    }
}
