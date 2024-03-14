use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::call_graph::CallGraph;

/// Represents a call stack, which is a tree-like structure of function calls.
///
/// The `CallStack` is used to represent the hierarchy of function calls in a program.
/// It contains a map of `CallStackNode`s, where each node represents a function and its
/// associated metadata, such as the file path, class name (if applicable), and child nodes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallStack {
    nodes: HashMap<String, CallStackNode>,
}

/// Represents a node in the call stack, which corresponds to a function call.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallStackNode {
    /// The path of the file containing the function.
    pub file_path: String,
    /// The name of the class containing the function, if applicable.
    pub class_name: Option<String>,
    /// The name of the function.
    pub function_name: String,
    /// The keys of the child nodes (i.e., functions called by this function).
    pub children: Vec<String>,
}

impl CallStack {
    /// Creates a new, empty `CallStack`.
    pub fn new() -> Self {
        CallStack {
            nodes: HashMap::new(),
        }
    }

    /// Adds a new node to the call stack.
    ///
    /// # Arguments
    ///
    /// * `node_key` - The unique key for the node.
    /// * `node` - The `CallStackNode` to add.
    pub fn add_node(&mut self, node_key: String, node: CallStackNode) {
        self.nodes.insert(node_key, node);
    }

    /// Retrieves a node from the call stack by its key.
    ///
    /// # Arguments
    ///
    /// * `node_key` - The unique key for the node.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the `CallStackNode` if it exists,
    /// or `None` if the node was not found.
    pub fn get_node(&self, node_key: &str) -> Option<&CallStackNode> {
        self.nodes.get(node_key)
    }

    /// Adds a child node to a parent node in the call stack.
    ///
    /// # Arguments
    ///
    /// * `parent_key` - The unique key for the parent node.
    /// * `child_key` - The unique key for the child node.
    pub fn add_child(&mut self, parent_key: &str, child_key: &str) {
        if let Some(parent_node) = self.nodes.get_mut(parent_key) {
            parent_node.children.push(child_key.to_string());
        }
    }

    /// Converts the `CallStack` to a `CallGraph`.
    ///
    /// # Returns
    ///
    /// A `CallGraph` representing the same information as the `CallStack`.
    pub fn to_call_graph(&self) -> CallGraph {
        let mut graph = CallGraph::new();

        for (node_key, node) in &self.nodes {
            graph.add_node(node_key.clone(), node.clone());
        }

        for (node_key, node) in &self.nodes {
            for child_key in &node.children {
                graph.add_edge(node_key.clone(), child_key.clone());
            }
        }

        graph
    }
}
