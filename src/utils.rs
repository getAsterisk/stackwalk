use phf::phf_map;

/// A static map of supported file extensions and their corresponding language names.
pub static SUPPORTED_EXTENSIONS: phf::Map<&'static str, &'static str> = phf_map! {
    "rs" => "Rust",
    "py" => "Python",
    "js" => "JavaScript",
    "ts" => "TypeScript",
    // Add more supported extensions and languages
};

/// Returns a vector of supported file extensions.
///
/// # Returns
///
/// A vector of strings representing the supported file extensions.
pub fn get_supported_extensions() -> Vec<String> {
    SUPPORTED_EXTENSIONS
        .keys()
        .map(|&s| s.to_string())
        .collect()
}
