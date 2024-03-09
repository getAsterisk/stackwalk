use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::call_graph::CallGraph;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallStack {
    nodes: HashMap<String, CallStackNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallStackNode {
    pub file_path: String,
    pub class_name: Option<String>,
    pub function_name: String,
    pub children: Vec<String>,
}

impl CallStack {
    pub fn new() -> Self {
        CallStack {
            nodes: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node_key: String, node: CallStackNode) {
        self.nodes.insert(node_key, node);
    }

    pub fn get_node(&self, node_key: &str) -> Option<&CallStackNode> {
        self.nodes.get(node_key)
    }

    pub fn add_child(&mut self, parent_key: &str, child_key: &str) {
        if let Some(parent_node) = self.nodes.get_mut(parent_key) {
            parent_node.children.push(child_key.to_string());
        }
    }

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
