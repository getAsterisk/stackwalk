use asterisk::config::Config;
/// Example CLI application demonstrating usage of the asterisk library.
///
/// This example:
/// 1. Reads the asterisk configuration from the `asterisk.toml` file
/// 2. Indexes the directory specified by the first command-line argument
/// 3. Generates a JSON output file with the indexed blocks and call stack
/// 4. Generates a Graphviz DOT file representing the call graph
///
/// To run the example, use:
/// ```
/// cargo run --example cli -- /path/to/directory/to/index
/// ```
use asterisk::indexer::index_directory;
use serde::Serialize;
use serde_json::json;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;

#[derive(Serialize)]
struct Output {
    blocks: Vec<asterisk::block::Block>,
    call_stack: asterisk::call_stack::CallStack,
}

fn main() {
    let toml_str = fs::read_to_string("asterisk.toml").expect("Unable to read file");
    let config = Config::from_toml(&toml_str).unwrap();

    let args: Vec<String> = env::args().collect();
    let dir_path = &args[1];
    let (blocks, call_stack, call_graph) = index_directory(&config, dir_path);

    let output = Output { blocks, call_stack };

    let json_output = json!({
        "blocks": output.blocks,
        "call_stack": output.call_stack
    });

    let pretty_json = serde_json::to_string_pretty(&json_output).unwrap();

    let project_name = dir_path.split('/').last().unwrap_or("blockoli");
    let output_file_name = format!("{}.json", project_name);

    let mut output_file = File::create(&output_file_name).expect("Failed to create output file");
    write!(output_file, "{}", pretty_json).expect("Failed to write to output file");
    println!("Indexing completed. Output written to {}", output_file_name);

    let graphviz = call_graph.to_graphviz();
    let graphviz_file_name = format!("{}_call_graph.dot", project_name);
    let mut graphviz_file =
        File::create(&graphviz_file_name).expect("Failed to create Graphviz file");
    write!(graphviz_file, "{}", graphviz).expect("Failed to write to Graphviz file");

    println!(
        "Call graph generated. Graphviz file written to {}",
        graphviz_file_name
    );

    let entry_points = call_graph.get_entry_points();
    if !entry_points.is_empty() {
        for entry_point in entry_points {
            println!("Potential Entry Point: {}", entry_point);
        }
    } else {
        println!("No entry points detected.");
    }
}
