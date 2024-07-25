<img src=".assets/stackwalk-logo.png" alt="stackwalk logo" width="250" align="right">

# `stackwalk` 🌟 

StackWalk is a library for parsing and indexing code in various languages. 📚

[![LICENSE](https://img.shields.io/github/license/stitionai/stackwalk.svg?cached)](https://github.com/stitionai/stackwalk/blob/master/LICENSE)

## Table of Contents

* [Features](#features)
* [Installation](#installation)
* [Usage](#usage)
* [Configuration](#configuration)
* [Development](#development)
* [Contribution](#contribution)
* [License](#license)
* [Support The Author](#liked-the-project)

## Features

- Extract code structure and call information 📊
- Generate call graphs 🌐 
- Support for multiple languages 🌍
  - Rust 🦀
  - Python 🐍
- Configurable language-specific settings 🛠️

## Installation

Add `stackwalk` as a dependency in your `Cargo.toml` file:

```toml
[dependencies]
stackwalk = "0.1.0"
```

## Usage

```rust
use stackwalk::config::Config;
use stackwalk::indexer::index_directory;

fn main() {
    let toml_str = fs::read_to_string("stackwalk.toml").expect("Unable to read file");
    let config = Config::from_toml(&toml_str).unwrap();

    let dir_path = "path/to/directory";
    let (blocks, call_stack, call_graph) = index_directory(&config, dir_path);

    // Process the extracted information
    // ...
}
```

## Configuration

StackWalk uses a TOML configuration file to specify language-specific settings. Here's an example configuration:

```toml
[languages]
  [languages.python]
    [languages.python.matchers]
      import_statement = "import_from_statement"
      # ...

  [languages.rust]
    [languages.rust.matchers]
      import_statement = "use_declaration"  
      # ...
```

## Development

To build the project from source:

```bash
$ git clone https://github.com/stitionai/stackwalk.git
$ cd stackwalk/
$ cargo build --release
```

## Contribution

Ways to contribute:
- Suggest a feature
- Report a bug
- Fix something and open a pull request
- Help document the code
- Spread the word

## License

Licensed under the MIT License, see <a href="https://github.com/stitionai/stackwalk/blob/master/LICENSE">LICENSE</a> for more information.

## Liked the project?

Support the project by starring the repository. ⭐
