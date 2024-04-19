use serde::{Deserialize, Serialize};

/// Represents the type of a code block.
#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum BlockType {
    /// A block that represents a function.
    Function,
    /// A block that does not represent a function.
    NonFunction,
}

/// Represents a block of code, which can be a function or a non-function block.
#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct Block {
    /// The unique key for the block.
    pub node_key: String,
    /// The type of the block (function or non-function).
    pub block_type: BlockType,
    /// The content of the block (i.e., the code).
    pub content: String,
    /// The name of the function, if the block is a function block.
    pub function_name: Option<String>,
    /// The name of the class containing the block, if applicable.
    pub class_name: Option<String>,
    /// The keys of the blocks called by this block.
    pub outgoing_calls: Vec<String>,
}

impl Block {
    /// Creates a new `Block` with the given parameters.
    ///
    /// # Arguments
    ///
    /// * `node_key` - The unique key for the block.
    /// * `block_type` - The type of the block (function or non-function).
    /// * `content` - The content of the block (i.e., the code).
    /// * `function_name` - The name of the function, if the block is a function block.
    /// * `class_name` - The name of the class containing the block, if applicable.
    ///
    /// # Returns
    ///
    /// A new `Block` instance with the specified parameters and an empty `outgoing_calls` vector.
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
