use std::collections::BTreeMap;

use askama::Template;

use crate::model::{Model, Property};
use crate::operation::Operation;

#[derive(Template)]
#[template(path = "models/model.rs.j2", escape = "none")]
pub struct ModelTemplate<'a> {
    pub model: &'a Model,
    pub properties: &'a Vec<Property>,
    pub models: &'a BTreeMap<String, Model>,
}

#[derive(Template)]
#[template(path = "params/params.rs.j2", escape = "none")]
pub struct ParamsTemplate<'a> {
    pub operation: &'a Operation,
}
