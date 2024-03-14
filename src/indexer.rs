use crate::config::Config;
use jwalk::WalkDir;
use std::path::Path;

use crate::block::{Block, BlockType};
use crate::call_graph::CallGraph;
use crate::call_stack::{CallStack, CallStackNode};
use crate::parser::parse_file;
use crate::utils::get_supported_extensions;

/// Checks if a file is supported by the indexer based on its extension.
///
/// # Arguments
///
/// * `path` - The path of the file to check.
///
/// # Returns
///
/// `true` if the file's extension is in the list of supported extensions, `false` otherwise.
fn is_supported_file(path: &Path) -> bool {
    let extensions = get_supported_extensions();
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| extensions.contains(&ext.to_lowercase()))
        .unwrap_or(false)
}

/// Generates a unique key for a node in the call stack or call graph.
///
/// # Arguments
///
/// * `file_path` - The path of the file containing the node.
/// * `class_name` - The name of the class containing the node, if applicable.
/// * `function_name` - The name of the function corresponding to the node.
///
/// # Returns
///
/// A string representing the unique key for the node.
pub fn generate_node_key(
    file_path: &Path,
    class_name: Option<&str>,
    function_name: &str,
) -> String {
    let mut key = file_path.to_str().unwrap().to_string();
    if let Some(class) = class_name {
        key.push('.');
        key.push_str(class);
    }
    key.push('.');
    key.push_str(function_name);
    key
}

/// Indexes a directory of code files and generates blocks, a call stack, and a call graph.
///
/// # Arguments
///
/// * `config` - The `Config` instance containing language-specific settings.
/// * `dir_path` - The path of the directory to index.
///
/// # Returns
///
/// A tuple containing:
/// - A vector of `Block`s representing the indexed code blocks.
/// - A `CallStack` representing the hierarchy of function calls.
/// - A `CallGraph` representing the relationships between functions.
pub fn index_directory(config: &Config, dir_path: &str) -> (Vec<Block>, CallStack, CallGraph) {
    let mut blocks = Vec::new();
    let mut call_stack = CallStack::new();

    for entry in WalkDir::new(dir_path) {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() && is_supported_file(&path) {
            let module_name = path.to_str().unwrap();
            let file_blocks = parse_file(&path, module_name, &config);
            blocks.extend(file_blocks.clone());

            for block in &file_blocks {
                match &block.block_type {
                    BlockType::Function => {
                        let function_name = block.function_name.clone().unwrap_or_default();
                        let class_name = block.class_name.clone();

                        let node_key =
                            generate_node_key(&path, class_name.as_deref(), &function_name);
                        let node = CallStackNode {
                            file_path: path.to_str().unwrap().trim_start_matches('/').to_string(),
                            class_name,
                            function_name: function_name.clone(),
                            children: Vec::new(),
                        };

                        call_stack.add_node(node_key.clone(), node);

                        for call in &block.outgoing_calls {
                            call_stack.add_child(&node_key, call);
                        }
                    }
                    BlockType::NonFunction => {
                        // Handle non-function blocks if needed
                    }
                }
            }
        }
    }

    let call_graph = call_stack.to_call_graph();

    (blocks, call_stack, call_graph)
}
