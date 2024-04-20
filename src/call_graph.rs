use crate::call_stack::CallStackNode;
use std::collections::{HashMap, HashSet};

use serde::{Serialize, Deserialize};
use serde_json::json;

/// Represents a call graph, which is a directed graph of function calls.
///
/// The `CallGraph` is used to represent the relationships between functions in a program,
/// where each node corresponds to a function and each edge represents a function call.
#[derive(Debug, Clone, Serialize, Deserialize)]
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

    /// Converts the `CallGraph` to a Mermaid diagram format string.
    ///
    /// # Returns
    ///
    /// A string containing the Mermaid representation of the call graph.
    pub fn to_mermaid(&self) -> String {
        let mut mermaid = String::from("graph TD;\n");
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
            // convert spaces into underscores
            let node_key = node_key.replace(' ', "_");
            node_label = node_label.replace(' ', "_");
            mermaid.push_str(&format!("  {}[\"{}\"];\n", node_key, node_label));
        }

        for (from, to) in &self.edges {
            // convert spaces into underscores
            let from = from.replace(' ', "_");
            let to = to.replace(' ', "_");
            mermaid.push_str(&format!("  {} --> {};\n", from, to));
        }

        mermaid
    }

    /// Converts the `CallGraph` to a JSON formatted string suitable for generating flowcharts.
    ///
    /// # Returns
    ///
    /// A pretty JSON string representing the call graph with nodes and edges.
    pub fn to_json_flowchart(&self) -> String {
        let nodes: Vec<_> = self.nodes.iter().map(|(key, node)| {
            let file_name = node.file_path.split('/').last().unwrap_or("");
            let node_label = if let Some(class_name) = &node.class_name {
                format!("{}::{}::{}", file_name, class_name, node.function_name)
            } else {
                format!("{}::{}", file_name, node.function_name)
            };
            json!({
                "id": key,
                "label": node_label
            })
        }).collect();

        let edges: Vec<_> = self.edges.iter().map(|(from, to)| {
            json!({
                "from": from,
                "to": to
            })
        }).collect();

        let flowchart = json!({
            "nodes": nodes,
            "edges": edges
        });

        serde_json::to_string_pretty(&flowchart).unwrap()
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

        for (_, to) in &self.edges {
            incoming_edges.insert(to.clone());
        }

        // Detecting only 'main' functions or equivalents as entry points
        for (node_key, node) in &self.nodes {
            if node.function_name == "main" && !incoming_edges.contains(node_key) {
                candidates.insert(node_key.clone());
            } else if !incoming_edges.contains(node_key) {
                candidates.insert(node_key.clone());
            }
        }

        candidates.into_iter().collect()
    }
}