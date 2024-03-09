use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BlockType {
    Function,
    NonFunction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub node_key: String,
    pub block_type: BlockType,
    pub content: String,
    pub function_name: Option<String>,
    pub class_name: Option<String>,
    pub outgoing_calls: Vec<String>,
}

impl Block {
    pub fn new(
        node_key: String,
        block_type: BlockType,
        content: String,
        function_name: Option<String>,
        class_name: Option<String>,
    ) -> Self {
        Block {
            node_key,
            block_type,
            content,
            function_name,
            class_name,
            outgoing_calls: Vec::new(),
        }
    }
}
