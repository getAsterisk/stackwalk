use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub languages: HashMap<String, Language>,
}

#[derive(Deserialize, Debug)]
pub struct Language {
    pub matchers: Matchers,
}

#[derive(Deserialize, Debug)]
pub struct Matchers {
    pub import_statement: String,
    pub module_name: Matcher,
    pub object_name: Matcher,
    pub alias: Matcher,
}

#[derive(Deserialize, Debug)]
pub struct Matcher {
    pub field_name: String,
    pub kind: String,
}


impl Config {
    pub fn from_toml(toml_str: &str) -> Result<Self, toml::de::Error> {
        let configs: Config = toml::from_str(toml_str).expect("Failed to parse TOML");

        Ok(configs)
    }
}
