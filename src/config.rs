use serde::Deserialize;
use std::collections::HashMap;

/// Represents the configuration for the asterisk library.
///
/// The configuration is loaded from a TOML file and contains
/// language-specific settings for parsing and indexing code.
#[derive(Deserialize, Debug)]
pub struct Config {
    /// A map of language names to their specific configurations.
    pub languages: HashMap<String, Language>,
}

/// Represents the configuration for a specific language.
#[derive(Deserialize, Debug)]
pub struct Language {
    /// The matchers used to identify and extract information from AST nodes.
    pub matchers: Matchers,
}

/// Represents the matchers used to identify and extract information from AST nodes.
#[derive(Deserialize, Debug)]
pub struct Matchers {
    /// The name of the AST node type that represents an import statement.
    pub import_statement: String,
    /// The matcher for extracting the module name from an import statement.
    pub module_name: Matcher,
    /// The matcher for extracting the object name from an import statement.
    pub object_name: Matcher,
    /// The matcher for extracting the alias from an import statement.
    pub alias: Matcher,
}

/// Represents a matcher used to extract information from an AST node.
#[derive(Deserialize, Debug)]
pub struct Matcher {
    /// The name of the field in the AST node that contains the desired information.
    pub field_name: String,
    /// The kind (type) of the AST node that contains the desired information.
    pub kind: String,
}

impl Config {
    /// Creates a new `Config` instance from a TOML string.
    ///
    /// # Arguments
    ///
    /// * `toml_str` - A string containing the TOML configuration.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `Config` instance if the TOML was successfully parsed,
    /// or an error if parsing failed.
    pub fn from_toml(toml_str: &str) -> Result<Self, toml::de::Error> {
        let configs: Config = toml::from_str(toml_str).expect("Failed to parse TOML");

        Ok(configs)
    }
}
