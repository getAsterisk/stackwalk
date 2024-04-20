use crate::block::{Block, BlockType};
use crate::config::{Config, Matchers};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use tree_sitter::{Language, Node, Parser};

use crate::indexer::generate_node_key;

// C FFI bindings to the tree-sitter language libraries.
extern "C" {
    fn tree_sitter_rust() -> Language;
    fn tree_sitter_python() -> Language;
    fn tree_sitter_javascript() -> Language;
    // Add more language bindings here
}

/// Parses a code file and returns a vector of `Block`s representing the code structure.
///
/// # Arguments
///
/// * `file_path` - The path of the file to parse.
/// * `module_name` - The name of the module containing the file.
/// * `config` - The `Config` instance containing language-specific settings.
///
/// # Returns
///
/// A vector of `Block`s representing the code structure of the parsed file.
pub fn parse_file(file_path: &Path, module_name: &str, config: &Config) -> Vec<Block> {
    let code = fs::read_to_string(file_path).unwrap();
    let language = tree_sitter_language(file_path);
    let mut parser = Parser::new();
    parser.set_language(language).unwrap();
    let tree = parser.parse(&code, None).unwrap();

    let mut blocks = Vec::new();
    let mut non_function_blocks = Vec::new();
    let mut imports = HashMap::new();
    let mut cursor = tree.root_node().walk();

    traverse_tree(
        &code,
        &mut cursor,
        &mut blocks,
        &mut non_function_blocks,
        language,
        None,
        module_name,
        &mut imports,
        &config,
    );

    if !non_function_blocks.is_empty() {
        let non_function_block_content = non_function_blocks.join("\n");
        blocks.push(Block::new(
            String::from("non_function_block"),
            BlockType::NonFunction,
            non_function_block_content,
            None,
            None,
        ));
    }

    blocks
}

/// Returns the appropriate tree-sitter `Language` for a given file based on its extension.
///
/// # Arguments
///
/// * `file_path` - The path of the file to get the language for.
///
/// # Returns
///
/// The tree-sitter `Language` corresponding to the file's extension.
///
/// # Panics
///
/// Panics if the file's extension is not supported.
fn tree_sitter_language(file_path: &Path) -> Language {
    let extension = file_path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");
    match extension {
        "rs" => unsafe { tree_sitter_rust() },
        "py" => unsafe { tree_sitter_python() },
        "js" => unsafe { tree_sitter_javascript() },
        // Add more mappings for other supported languages
        _ => panic!("Unsupported language"),
    }
}

/// Recursively traverses the AST and extracts code blocks and call information.
///
/// # Arguments
///
/// * `code` - The code string of the file being parsed.
/// * `cursor` - A mutable reference to the `TreeCursor` used to navigate the AST.
/// * `blocks` - A mutable reference to the vector of `Block`s to populate.
/// * `non_function_blocks` - A mutable reference to the vector of non-function block strings.
/// * `language` - The tree-sitter `Language` of the file being parsed.
/// * `class_name` - An optional string representing the name of the current class, if any.
/// * `module_name` - The name of the module containing the file being parsed.
/// * `imports` - A mutable reference to the map of import aliases to their full module names.
/// * `config` - The `Config` instance containing language-specific settings.
fn traverse_tree(
    code: &str,
    cursor: &mut tree_sitter::TreeCursor,
    blocks: &mut Vec<Block>,
    non_function_blocks: &mut Vec<String>,
    language: Language,
    class_name: Option<String>,
    module_name: &str,
    imports: &mut HashMap<String, String>,
    config: &Config,
) {
    let node = cursor.node();
    let kind = node.kind();

    if is_import_statement(kind, language) {
        let imports_list = parse_import_statement(code, node, language, config);
        for (object_name, module_name) in imports_list {
            imports.insert(object_name, module_name);
        }
    } else if is_class_definition(kind, language) {
        let class_name_node = node.child_by_field_name("name");
        if let Some(class_name_node) = class_name_node {
            let extracted_class_name = class_name_node
                .utf8_text(code.as_bytes())
                .unwrap()
                .to_string();

            if cursor.goto_first_child() {
                loop {
                    traverse_tree(
                        code,
                        cursor,
                        blocks,
                        non_function_blocks,
                        language,
                        Some(extracted_class_name.clone()),
                        module_name,
                        imports,
                        config,
                    );
                    if !cursor.goto_next_sibling() {
                        break;
                    }
                }
                cursor.goto_parent();
            }
        }
    } else if is_function_node(kind, language) {
        let function_name = get_function_name(code, node, language)
            .unwrap_or_else(|| "UnnamedFunction".to_string());
        let block_type = BlockType::Function;
        let block_content = node.utf8_text(code.as_bytes()).unwrap().to_string();

        let node_key = generate_node_key(
            Path::new(module_name),
            class_name.as_deref(),
            &function_name,
        );

        let mut block = Block::new(
            node_key,
            block_type,
            block_content,
            Some(function_name.clone()),
            class_name.clone(),
        );

        block.outgoing_calls = find_calls(code, node, language, module_name, imports);

        blocks.push(block);
    } else if !node.is_named() {
        let block_content = node.utf8_text(code.as_bytes()).unwrap().to_string();
        non_function_blocks.push(block_content);
    }

    if cursor.goto_first_child() {
        loop {
            traverse_tree(
                code,
                cursor,
                blocks,
                non_function_blocks,
                language,
                class_name.clone(),
                module_name,
                imports,
                &config,
            );
            if !cursor.goto_next_sibling() {
                break;
            }
        }
        cursor.goto_parent();
    }
}

/// Finds the function calls made within a given AST node and returns their keys.
///
/// # Arguments
///
/// * `code` - The code string of the file being parsed.
/// * `root` - The AST node to search for function calls.
/// * `language` - The tree-sitter `Language` of the file being parsed.
/// * `module_name` - The name of the module containing the file being parsed.
/// * `imports` - A reference to the map of import aliases to their full module names.
///
/// # Returns
///
/// A vector of strings representing the keys of the called functions.
fn find_calls(
    code: &str,
    root: Node,
    language: Language,
    module_name: &str,
    imports: &HashMap<String, String>,
) -> Vec<String> {
    let mut calls = HashSet::new();
    let mut cursor = root.walk();

    loop {
        let node = cursor.node();

        if is_call_expression(node.kind(), language) {
            if let Some(function_name) = get_call_expression_name(code, node, language) {
                let parts: Vec<&str> = function_name.split('.').collect();

                if parts.len() > 1 {
                    // This is for method calls on an object; the part before '.' is treated as an object, not a module.
                    let object_name = parts[0];
                    let method_name = &parts[1..].join(".");

                    // If the object name matches an alias from the imports, resolve to the correct module.
                    if let Some(imported_module) = imports.get(object_name) {
                        let call_key = generate_node_key(
                            Path::new(imported_module),
                            Some(object_name),
                            method_name,
                        );
                        calls.insert(call_key);
                    } else {
                        let call_key = generate_node_key(
                            Path::new(module_name),
                            Some(object_name),
                            method_name,
                        );
                        calls.insert(call_key);
                    }
                } else {
                    // For global function calls, check if the function name matches an alias from the imports.
                    if let Some(imported_module) = imports.get(&function_name) {
                        let call_key = generate_node_key(
                            Path::new(&format!("test-code-base/{}.py", imported_module)),
                            None,
                            &function_name,
                        );
                        calls.insert(call_key);
                    } else {
                        let function_key =
                            generate_node_key(Path::new(module_name), None, &function_name);
                        calls.insert(function_key);
                    }
                }
            }
        }

        if !cursor.goto_first_child() {
            while !cursor.goto_next_sibling() {
                if !cursor.goto_parent() {
                    return calls.into_iter().collect();
                }
            }
        }
    }
}

/// Checks if an AST node represents an import statement in the given language.
///
/// # Arguments
///
/// * `kind` - The kind (type) of the AST node.
/// * `language` - The tree-sitter `Language` of the file being parsed.
///
/// # Returns
///
/// `true` if the node represents an import statement, `false` otherwise.
fn is_import_statement(kind: &str, language: Language) -> bool {
    match language {
        lang if lang == unsafe { tree_sitter_python() } => {
            kind == "import_statement" || kind == "import_from_statement"
        }
        lang if lang == unsafe { tree_sitter_rust() } => kind == "use_declaration",
        lang if lang == unsafe { tree_sitter_javascript() } => kind == "import_statement",
        // Add more language-specific checks here
        _ => false,
    }
}

/// Filters the children of an import statement node using the provided matchers.
///
/// # Arguments
///
/// * `child` - The child node of the import statement to filter.
/// * `code` - The code string of the file being parsed.
/// * `matchers` - The `Matchers` instance containing the field names and node types to match.
///
/// # Returns
///
/// A tuple containing:
/// - An optional string representing the module name.
/// - An optional string representing the imported object name.
/// - An optional string representing the import alias.
fn filter_import_matchers(
    child: Node,
    code: &str,
    matchers: &Matchers,
) -> (Option<String>, Option<String>, Option<String>) {
    let module = child
        .child_by_field_name(&matchers.module_name.field_name)
        .map(|n| {
            if n.kind() == matchers.module_name.kind {
                return n.utf8_text(code.as_bytes()).unwrap_or_default().to_owned();
            }

            String::default()
        });

    let name = child
        .child_by_field_name(&matchers.object_name.field_name)
        .map(|n| {
            if n.kind() == matchers.object_name.kind {
                return n.utf8_text(code.as_bytes()).unwrap_or_default().to_owned();
            }

            String::default()
        });

    let alias = child
        .child_by_field_name(&matchers.alias.field_name)
        .map(|n| {
            if n.kind() == matchers.alias.kind {
                return n.utf8_text(code.as_bytes()).unwrap_or_default().to_owned();
            }

            String::default()
        });

    (module, name, alias)
}

/// Parses an import statement node and returns the imported module and alias, if any.
///
/// # Arguments
///
/// * `code` - The code string of the file being parsed.
/// * `node` - The import statement AST node to parse.
/// * `language` - The tree-sitter `Language` of the file being parsed.
/// * `config` - The `Config` instance containing language-specific settings.
///
/// # Returns
///
/// An `Option` containing a tuple of the imported module name and alias, if successfully parsed.
fn parse_import_statement(
    code: &str,
    node: Node,
    language: Language,
    config: &Config,
) -> Vec<(String, String)> {
    let mut module_name = String::new();
    let mut object_name = String::new();
    let mut alias_name = String::new();

    match language {
        lang if lang == unsafe { tree_sitter_javascript() } => {
            let mut cursor = node.walk();
            let module_name = node
                .child_by_field_name("source")
                .map(|n| {
                    return n.utf8_text(code.as_bytes()).unwrap_or_default().to_owned();
                })
                .unwrap_or_default();

            let mut import_clause_string = String::default();

            for child in node.named_children(&mut cursor) {
                if child.kind() == "import_clause" {
                    import_clause_string = child
                        .utf8_text(code.as_bytes())
                        .unwrap_or_default()
                        .to_owned();
                }
            }

            let mut imports: Vec<(String, String)> = Vec::new();

            if import_clause_string.contains("as") {
                let alias: Vec<String> = import_clause_string
                    .split("as")
                    .map(|s| {
                        s.trim() // Trim whitespace
                            .to_owned()
                    }) // Convert to owned String
                    .collect();

                imports.push((
                    module_name.clone(),
                    alias.last().unwrap_or(&"".to_owned()).to_owned(),
                ));
            } else if import_clause_string.starts_with("{") {
                let val: Vec<String> = import_clause_string
                    .trim() // Trim whitespace around the string
                    .trim_start_matches('{') // Remove the starting '{'
                    .trim_end_matches('}') // Remove the ending '}'
                    .split(',') // Split the string by commas
                    .map(|s| s.trim().to_string()) // Trim whitespace and convert to String
                    .filter(|s| !s.is_empty()) // Filter out any empty strings
                    .collect();

                imports = val
                    .iter()
                    .map(|num| (module_name.clone(), num.clone()))
                    .collect();
            }

            println!(
                "{}",
                imports
                    .iter()
                    .map(|(key, value)| format!("({}, {})", key, value))
                    .collect::<Vec<_>>()
                    .join(", ")
            );

            return imports;
        }
        lang if lang == unsafe { tree_sitter_python() } => {
            let matchers = &config
                .languages
                .get("python")
                .expect("Failed to get Python matchers from config")
                .matchers;

            if node.kind() == matchers.import_statement {
                let result = filter_import_matchers(node, code, matchers);
                (module_name, object_name, alias_name) = (
                    result.0.unwrap_or(module_name),
                    result.1.unwrap_or(object_name),
                    result.2.unwrap_or(alias_name),
                );

                let mut cursor = node.walk();
                for child in node.named_children(&mut cursor) {
                    let result = filter_import_matchers(child, code, matchers);
                    (module_name, object_name, alias_name) = (
                        result.0.unwrap_or(module_name),
                        result.1.unwrap_or(object_name),
                        result.2.unwrap_or(alias_name),
                    );

                    let mut cursor2 = child.walk();
                    for child2 in child.named_children(&mut cursor2) {
                        let result = filter_import_matchers(child2, code, matchers);
                        (module_name, object_name, alias_name) = (
                            result.0.unwrap_or(module_name),
                            result.1.unwrap_or(object_name),
                            result.2.unwrap_or(alias_name),
                        );
                    }
                }

                println!(
                    "Module: {}, Object: {}, Alias: {}",
                    module_name, object_name, alias_name
                );

                return vec![(module_name, object_name)];
            }
            vec![]
        }
        lang if lang == unsafe { tree_sitter_rust() } => {
            let matchers = &config
                .languages
                .get("rust")
                .expect("Failed to get Python matchers from config")
                .matchers;

            if node.kind() == matchers.import_statement {
                let result = filter_import_matchers(node, code, matchers);
                (module_name, object_name, alias_name) = (
                    result.0.unwrap_or(module_name),
                    result.1.unwrap_or(object_name),
                    result.2.unwrap_or(alias_name),
                );

                let mut cursor = node.walk();
                for child in node.named_children(&mut cursor) {
                    let result = filter_import_matchers(child, code, matchers);
                    (module_name, object_name, alias_name) = (
                        result.0.unwrap_or(module_name),
                        result.1.unwrap_or(object_name),
                        result.2.unwrap_or(alias_name),
                    );

                    let mut cursor2 = child.walk();
                    for child2 in child.named_children(&mut cursor2) {
                        let result = filter_import_matchers(child2, code, matchers);
                        (module_name, object_name, alias_name) = (
                            result.0.unwrap_or(module_name),
                            result.1.unwrap_or(object_name),
                            result.2.unwrap_or(alias_name),
                        );
                    }
                }

                println!(
                    "Module: {}, Object: {}, Alias: {}",
                    module_name, object_name, alias_name
                );
                return vec![(module_name, object_name)];
            }
            vec![]
        }
        _ => vec![],
    }
}

/// Checks if an AST node represents a class definition in the given language.
///
/// # Arguments
///
/// * `kind` - The kind (type) of the AST node.
/// * language - The tree-sitter Language of the file being parsed.
///
/// # Returns
///
/// true if the node represents a class definition, false otherwise.
fn is_class_definition(kind: &str, language: Language) -> bool {
    match language {
        lang if lang == unsafe { tree_sitter_python() } => kind == "class_definition",
        // Add more language-specific checks here
        _ => false,
    }
}

/// Checks if an AST node represents a function definition in the given language.
///
/// # Arguments
///
/// * kind - The kind (type) of the AST node.
/// * language - The tree-sitter Language of the file being parsed.
///
/// # Returns
///
/// true if the node represents a function definition, false otherwise.
fn is_function_node(kind: &str, language: Language) -> bool {
    match language {
        lang if lang == unsafe { tree_sitter_rust() } => kind == "function_item",
        lang if lang == unsafe { tree_sitter_python() } => kind == "function_definition",
        lang if lang == unsafe { tree_sitter_javascript() } => kind == "function_declaration",
        // Add more language-specific checks here
        _ => false,
    }
}

/// Extracts the function name from a function definition AST node.
///
/// # Arguments
///
/// * code - The code string of the file being parsed.
/// * node - The function definition AST node to extract the name from.
/// * language - The tree-sitter Language of the file being parsed.
///
/// # Returns
///
/// An Option containing the function name, if successfully extracted.
fn get_function_name(code: &str, node: Node, language: Language) -> Option<String> {
    match language {
        lang if lang == unsafe { tree_sitter_rust() } => node
            .child_by_field_name("name")
            .and_then(|child| Some(child.utf8_text(code.as_bytes()).unwrap()))
            .map(|s| s.to_string()),
        lang if lang == unsafe { tree_sitter_python() } => node             
            .child_by_field_name("name")
            .and_then(|child| Some(child.utf8_text(code.as_bytes()).unwrap()))
            .map(|s| s.to_string()),
        lang if lang == unsafe { tree_sitter_javascript() } => node
            .child_by_field_name("name")
            .and_then(|child| Some(child.utf8_text(code.as_bytes()).unwrap()))
            .map(|s| s.to_string()),
        lang if lang == unsafe { tree_sitter_javascript() } => node
            .child_by_field_name("name")
            .and_then(|child| Some(child.utf8_text(code.as_bytes()).unwrap()))
            .map(|s| s.to_string()),
        // Add more language-specific checks here
        _ => None,
    }
}

/// Checks if an AST node represents a function call expression in the given language.
///
/// # Arguments
///
/// * kind - The kind (type) of the AST node.
/// * language - The tree-sitter Language of the file being parsed.
///
/// # Returns
///
/// true if the node represents a function call expression, false otherwise.
fn is_call_expression(kind: &str, language: Language) -> bool {
    match language {
        lang if lang == unsafe { tree_sitter_rust() } => kind == "call_expression",
        lang if lang == unsafe { tree_sitter_python() } => kind == "call",
        lang if lang == unsafe { tree_sitter_javascript() } => kind == "call_expression",
        // Add more language-specific checks here
        _ => false,
    }
}

/// Extracts the called function name from a function call expression AST node.
///
/// # Arguments
///
/// * code - The code string of the file being parsed.
/// * node - The function call expression AST node to extract the name from.
/// * language - The tree-sitter Language of the file being parsed.
///
/// # Returns
///
/// An Option containing the called function name, if successfully extracted.
fn get_call_expression_name(code: &str, node: Node, language: Language) -> Option<String> {
    match language {
        lang if lang == unsafe { tree_sitter_rust() } => node
            .child_by_field_name("function")
            .and_then(|child| Some(child.utf8_text(code.as_bytes()).unwrap()))
            .map(|s| s.to_string()),
        lang if lang == unsafe { tree_sitter_python() } => node
            .child_by_field_name("function")
            .and_then(|child| Some(child.utf8_text(code.as_bytes()).unwrap()))
            .map(|s| s.to_string()),
        lang if lang == unsafe { tree_sitter_javascript() } => node
            .child_by_field_name("function")
            .and_then(|child| Some(child.utf8_text(code.as_bytes()).unwrap()))
            .map(|s| s.to_string()),
        // Add more language-specific checks here
        _ => None,
    }
}
