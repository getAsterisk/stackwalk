use crate::call_stack::CallStackNode;
use std::collections::HashMap;

pub struct CallGraph {
    nodes: HashMap<String, CallStackNode>,
    edges: Vec<(String, String)>,
}

impl CallGraph {
    pub fn new() -> Self {
        CallGraph {
            nodes: HashMap::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node_key: String, node: CallStackNode) {
        self.nodes.insert(node_key, node);
    }

    pub fn add_edge(&mut self, from: String, to: String) {
        self.edges.push((from, to));
    }

    pub fn to_graphviz(&self) -> String {
        let mut graphviz = String::from("digraph CallGraph {\n");
        graphviz.push_str("  rankdir=LR;\n");
        graphviz.push_str("  node [shape=box];\n");

        for (node_key, node) in &self.nodes {
            let mut node_label = node.function_name.clone();
            if let Some(class_name) = &node.class_name {
                node_label = format!("{}::{}", class_name, node_label);
            }
            graphviz.push_str(&format!("  \"{}\" [label=\"{}\"];\n", node_key, node_label));
        }

        for (from, to) in &self.edges {
            graphviz.push_str(&format!("  \"{}\" -> \"{}\";\n", from, to));
        }

        graphviz.push('}');
        graphviz
    }
}