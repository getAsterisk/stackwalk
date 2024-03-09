use crate::block::{Block, BlockType};
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use tree_sitter::{Language, Node, Parser};

extern "C" {
    fn tree_sitter_rust() -> Language;
    fn tree_sitter_python() -> Language;
    // Add more language bindings here
}

pub fn parse_file(file_path: &Path) -> Vec<Block> {
    let code = fs::read_to_string(file_path).unwrap();
    let language = tree_sitter_language(file_path);
    let mut parser = Parser::new();
    parser.set_language(language).unwrap();
    let tree = parser.parse(&code, None).unwrap();

    let mut blocks = Vec::new();
    let mut non_function_blocks = Vec::new();
    let mut cursor = tree.root_node().walk();
    traverse_tree(
        &code,
        &mut cursor,
        &mut blocks,
        &mut non_function_blocks,
        language,
        None,
    );

    if !non_function_blocks.is_empty() {
        let non_function_block_content = non_function_blocks.join("\n");
        blocks.push(Block::new(
            BlockType::NonFunction,
            non_function_block_content,
            None,
            None,
        ));
    }

    blocks
}

fn tree_sitter_language(file_path: &Path) -> Language {
    let extension = file_path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");
    match extension {
        "rs" => unsafe { tree_sitter_rust() },
        "py" => unsafe { tree_sitter_python() },
        // Add more mappings for other supported languages
        _ => panic!("Unsupported language"),
    }
}

fn traverse_tree(
    code: &str,
    cursor: &mut tree_sitter::TreeCursor,
    blocks: &mut Vec<Block>,
    non_function_blocks: &mut Vec<String>,
    language: Language,
    class_name: Option<String>,
) {
    let node = cursor.node();
    let kind = node.kind();

    if is_class_definition(kind, language) {
        // Extract class name
        let class_name_node = node.child_by_field_name("name");
        if let Some(class_name_node) = class_name_node {
            let extracted_class_name = class_name_node
                .utf8_text(code.as_bytes())
                .unwrap()
                .to_string();

            // Dive into class scope with the class_name
            if cursor.goto_first_child() {
                loop {
                    traverse_tree(
                        code,
                        cursor,
                        blocks,
                        non_function_blocks,
                        language,
                        Some(extracted_class_name.clone()),
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

        // Pass `class_name` which could be Some or None depending on context
        let mut block = Block::new(
            block_type,
            block_content,
            Some(function_name.clone()),
            class_name.clone(),
        );

        block.outgoing_calls = find_calls(code, node, language);

        blocks.push(block);
    } else if !node.is_named() {
        let block_content = node.utf8_text(code.as_bytes()).unwrap().to_string();
        non_function_blocks.push(block_content);
    }

    // Recursively traverse the AST
    if cursor.goto_first_child() {
        loop {
            traverse_tree(
                code,
                cursor,
                blocks,
                non_function_blocks,
                language,
                class_name.clone(),
            );
            if !cursor.goto_next_sibling() {
                break;
            }
        }
        cursor.goto_parent();
    }
}

fn find_calls(code: &str, root: Node, language: Language) -> Vec<String> {
    let mut calls = HashSet::new();
    let mut cursor = root.walk();

    loop {
        let node = cursor.node();

        if is_call_expression(node.kind(), language) {
            if let Some(function_name) = get_call_expression_name(code, node, language) {
                calls.insert(function_name);
            }
        }

        // Try to descend to the first child; if there isn't one, move to the next sibling.
        if !cursor.goto_first_child() {
            // No child, try moving to the next sibling.
            while !cursor.goto_next_sibling() {
                // No more siblings, ascend to the parent.
                if !cursor.goto_parent() {
                    // If we're back at the root, break out of the loop.
                    return calls.into_iter().collect();
                }
            }
        }
    }
}

fn is_class_definition(kind: &str, language: Language) -> bool {
    match language {
        lang if lang == unsafe { tree_sitter_python() } => kind == "class_definition",
        // Add more language-specific checks here
        _ => false,
    }
}

fn is_function_node(kind: &str, language: Language) -> bool {
    match language {
        lang if lang == unsafe { tree_sitter_rust() } => kind == "function_item",
        lang if lang == unsafe { tree_sitter_python() } => kind == "function_definition",
        // Add more language-specific checks here
        _ => false,
    }
}

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
        // Add more language-specific checks here
        _ => None,
    }
}

fn is_call_expression(kind: &str, language: Language) -> bool {
    match language {
        lang if lang == unsafe { tree_sitter_rust() } => kind == "call_expression",
        lang if lang == unsafe { tree_sitter_python() } => kind == "call",
        // Add more language-specific checks here
        _ => false,
    }
}

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
        // Add more language-specific checks here
        _ => None,
    }
}
