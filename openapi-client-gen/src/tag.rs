use convert_case::{Case, Casing};

use crate::operation::Operation;

pub struct Tag {
    pub description: String,
    pub safe_name: String,
    pub operations: Vec<Operation>,
}

impl Tag {
    pub fn var_name(&self) -> String {
        self.safe_name.to_case(Case::Snake)
    }

    pub fn struct_name(&self) -> String {
        self.safe_name.to_case(Case::UpperCamel)
    }

    pub fn file_name(&self) -> String {
        format!("{}.rs", self.var_name())
    }
}
