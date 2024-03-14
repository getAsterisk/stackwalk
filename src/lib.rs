//! # Asterisk
//!
//! Asterisk is a library for parsing and indexing code in various languages.
//! It provides functionality for extracting information about code structure,
//! call graphs, and dependencies.
//!
//! The main components of the library are:
//! - [`block`]: Defines the `Block` struct for representing code blocks.
//! - [`call_graph`]: Defines the `CallGraph` struct for representing call graphs.
//! - [`call_stack`]: Defines the `CallStack` struct for representing call stacks.
//! - [`indexer`]: Provides functions for indexing code directories.
//! - [`parser`]: Provides functions for parsing code files using tree-sitter.
//! - [`utils`]: Provides utility functions used throughout the library.
//! - [`config`]: Defines the `Config` struct for loading library configuration.

pub mod block;
pub mod call_graph;
pub mod call_stack;
pub mod config;
pub mod indexer;
pub mod parser;
pub mod utils;
