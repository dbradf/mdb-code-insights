use std::path::Path;

use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub mongo_uri: String,
    pub database: String,
    pub collection: String,
}

impl Config {
    pub fn from_yaml_file(yaml_file: &Path) -> Result<Self> {
        let contents = std::fs::read_to_string(yaml_file)?;
        Ok(serde_yaml::from_str(&contents)?)
    }
}
