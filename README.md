# Rust CLI Template By RustReadyDev

A lightweight, open-source Rust CLI template for 
building command-line applications. Perfect for
quick prototyping or learning Rust.

Includes argument parsing, configuration management, logging, and error handling out of the box.

## Features
- **Argument Parsing**: Powered by 'clap' with subcommand support.

- **Configuration Management**: JSON-based config with 'serde' for persistence.

- **Logging**: Configurable logging with 'env_logger'.

- **Error Handling**: Simplified with 'anyhow' for clean error propagation.

- **Extensible**: Modular structure for adding new commands and features.

## Prerequisites
- Rust (stable, 1.80 or later) 
- Cargo

## Installation
1. clone or download the template:
- https://github.com/rustreadydev/rustready-rust-cli-core
3. cd rustready-rust-cli-core
4. cargo build

## Usage

Run the CLI with: Cargo run -- --help (To see all commands)

Example Commands

cargo run -- run Alice  (Output: Hello, Alice! Config updated. Can change Alice to a different name if you want.)

cargo run -- version  (To check current version) 

## Configuration

The template uses a JSON config file (templents/config.json) to store persistant data.

Specify a custom config path with --config  (cargo run -- --config custom.json run Bob)

## Customization

To add a new command:
1. Edit src/cli.rs to define a new subcommand in the Command enu.
2. Update src/main.rs to handle new subcommand.
3. See templates/example.rs for an example implementation.

## Contributating

This project is open-source under the MIT License. 
Pull requests welcome! See CONTRIBUTING.md.

## Premium version

Want More Features?

Check out premium template rustready-rust-cli-premium with advance features like:

* Async command execution with tokio.

* TUI

* Plugins

* Built-in unit and integration tests.

* Custom logging formats.

* JSON logging

* And shell completions

* Available at: https://rustreadydev.gumroad.com/l/rustready-rust-cli-premium

SUPPORT: 

For community support, file issues at github.com/rustreadydev/rustready-rust-cli-core.

