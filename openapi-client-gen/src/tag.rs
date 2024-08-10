use crate::operation::Operation;

pub struct Tag {
    pub description: String,
    pub name: String,
    pub operations: Vec<Operation>,
}
