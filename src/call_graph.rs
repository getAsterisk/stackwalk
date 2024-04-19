use crate::call_stack::CallStackNode;
use std::collections::{HashMap, HashSet};

/// Represents a call graph, which is a directed graph of function calls.
///
/// The `CallGraph` is used to represent the relationships between functions in a program,
/// where each node corresponds to a function and each edge represents a function call.
pub struct CallGraph {
    /// A map of node keys to their corresponding `CallStackNode`s.
    nodes: HashMap<String, CallStackNode>,
    /// A vector of edges, where each edge is a tuple of the caller and callee node keys.
    edges: Vec<(String, String)>,
}

impl CallGraph {
    /// Creates a new, empty `CallGraph`.
    pub fn new() -> Self {
        CallGraph {
            nodes: HashMap::new(),
            edges: Vec::new(),
        }
    }

    /// Adds a new node to the call graph.
    ///
    /// # Arguments
    ///
    /// * `node_key` - The unique key for the node.
    /// * `node` - The `CallStackNode` to add.
    pub fn add_node(&mut self, node_key: String, node: CallStackNode) {
        self.nodes.insert(node_key, node);
    }

    /// Adds a new edge to the call graph.
    ///
    /// # Arguments
    ///
    /// * `from` - The key of the caller node.
    /// * `to` - The key of the callee node.
    pub fn add_edge(&mut self, from: String, to: String) {
        self.edges.push((from, to));
    }

    /// Converts the `CallGraph` to a Graphviz DOT format string.
    ///
    /// # Returns
    ///
    /// A string containing the Graphviz DOT representation of the call graph.
    pub fn to_graphviz(&self) -> String {
        let mut graphviz = String::from("digraph CallGraph {\n");
        graphviz.push_str("  rankdir=LR;\n");
        graphviz.push_str("  node [shape=box];\n");

        for (node_key, node) in &self.nodes {
            let file_name = node.file_path.split('/').last().unwrap_or("");
            let mut node_label = format!("{}::{}", file_name, node.function_name);
            if let Some(class_name) = &node.class_name {
                node_label = format!(
                    "{}::{}",
                    file_name,
                    format!("{}::{}", class_name, node.function_name)
                );
            }
            graphviz.push_str(&format!("  \"{}\" [label=\"{}\"];\n", node_key, node_label));
        }

        for (from, to) in &self.edges {
            graphviz.push_str(&format!("  \"{}\" -> \"{}\";\n", from, to));
        }

        graphviz.push('}');
        graphviz
    }

    /// Retrieves a list of potential entry points in the call graph.
    ///
    /// Defines an entry point as a node with no incoming edges and at least one outgoing edge, 
    /// representing functions that could initiate execution paths.
    ///
    /// # Returns
    ///
    /// A `Vec<String>` containing the keys of all potential entry point nodes.
    pub fn get_entry_points(&self) -> Vec<String> {
        let mut incoming_edges = HashSet::new();
        let mut candidates = HashSet::new();

        for (from, to) in &self.edges {
            incoming_edges.insert(to.clone());
            if !incoming_edges.contains(from) {
                candidates.insert(from.clone());
            }
        }

        candidates.retain(|candidate| !incoming_edges.contains(candidate));

        candidates.into_iter().collect()
    }
}