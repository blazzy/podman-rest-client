use std::collections::BTreeMap;

use askama::Template;

use crate::tag;
use crate::model::Model;
use crate::spec::Spec;

#[derive(Template)]
#[template(path = "api/api.rs.j2", escape = "none")]
pub struct ApiTemplate<'a> {
    pub tag: &'a tag::Tag,
    pub params: &'a BTreeMap<String, Vec<(String, String)>>,
    pub models: &'a BTreeMap<String, Model>,
}

#[derive(Template)]
#[template(path = "model/model.rs.j2", escape = "none")]
pub struct ModelTemplate<'a> {
    pub model: &'a Model,
    pub properties: Vec<(&'a Model, String, String)>,
}

#[derive(Template)]
#[template(path = "api/mod.rs.j2", escape = "none")]
pub struct ApiModTemplate<'a> {
    pub tags: &'a BTreeMap<String, tag::Tag>,
}

#[derive(Template)]
#[template(path = "model/mod.rs.j2", escape = "none")]
pub struct ModelModTemplate<'a> {
    pub models: &'a BTreeMap<&'a String, &'a Model>,
}

#[derive(Template)]
#[template(path = "mod.rs.j2", escape = "none")]
pub struct ModTemplate;

#[derive(Template)]
#[template(path = "client.rs.j2", escape = "none")]
pub struct ClientTemplate<'a> {
    pub tags: &'a BTreeMap<String, tag::Tag>,
}

#[derive(Template)]
#[template(path = "config.rs.j2", escape = "none")]
pub struct ConfigTemplate<'a> {
    pub spec: &'a Spec,
}
