use phf::phf_map;

pub static SUPPORTED_EXTENSIONS: phf::Map<&'static str, &'static str> = phf_map! {
    "rs" => "Rust",
    "py" => "Python",
    // Add more supported extensions and languages
};

pub fn get_supported_extensions() -> Vec<String> {
    SUPPORTED_EXTENSIONS
        .keys()
        .map(|&s| s.to_string())
        .collect()
}
