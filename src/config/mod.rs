use serde::Deserialize;
use std::{
    collections::{ HashMap},
    path::PathBuf,
};

#[derive(Deserialize, Debug)]
pub struct Profile {
    package: Package,
    dependencies: HashMap<String, toml::Value>,
}

#[derive(Deserialize, Debug)]
struct Package {
    name: String,
    version: String,
    edition: String,
}

pub fn load_config(file: &PathBuf) -> Profile {
    let mut contents = std::fs::read_to_string(file).expect("Path not found");
    let parsed = toml::from_str(&mut contents).unwrap();
    println!("{:#?}", parsed);
    parsed
}
