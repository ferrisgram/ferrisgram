use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiDescription {
    pub types: HashMap<String, TypeDescription>,
    pub methods: HashMap<String, MethodDescription>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TypeDescription {
    pub name: String,
    pub description: Vec<String>,
    pub fields: Option<Vec<Field>>,
    pub href: String,
    pub subtypes: Option<Vec<String>>,
    pub subtype_of: Option<Vec<String>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MethodDescription {
    pub name: String,
    pub fields: Option<Vec<Field>>,
    pub returns: Vec<String>,
    pub description: Vec<String>,
    pub href: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Field {
    pub name: String,
    pub types: Vec<String>,
    pub required: bool,
    pub description: String
}