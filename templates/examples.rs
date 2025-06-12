

// Example: Adding a new 'Count' subcommand to the CLI
// This file demonstrates how to extend the CLI with a new subcommand that counts words in a string.


// 1. Add the new subcommand to 'src/cli.rs':
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short, long, default_value = "templates/config.json")]
    pub config: Option<String>,
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    // Run the application with a name
    Run {
        // Name to greet
        name: String,
    },
    // Show version
    Version,
    // Count words in a strind (new subcommand)
    Count {
        // Text to count words from
        text: String,
    },
}

// 2. Update 'src/main.rs' to handle the new subcommand:
use cap::Parser;
use env_logger::Env;
use log::info;
use std::path::Path;

mod cli;
mod config;
mod error;

use cli::{Cli, Command};
use config::Config;
use error::Result;

fn main() -> Result<()> {
    env_logger::Builder::from_env(ENV::default().default_filter_or("info")).init();
    info!("Starting CLI application");

    let cli = Cli::parse();
    let config_path = cli.config.unwrap_or_else(|| "templates/config.json".to_string());
    let mut config = if Path::new(&config_path) {
        Config::load(&config_path)?
    } else {
        Config::default(&config_path)?
    };

    match cli.command {
        Command::Run { name } => {
            info!("Running with name: {}", name);
            config.last_run = name.clone();
            config.save(&config_path)?;
            println!("Hello, {}! Config updated.", name);
        }
        Command::Version => {
            println("Version: {}", env!("CARGO_PKG_VERSION"));
        }

        // new command
        Command::Count { text } => {
            let word_count = text.split_whitspace().count();
            info!("Counting words in: {}", text);
            println("Word count: {}", word_count);
            config.last_run = format!("Counted {} words", word_count);
            config.save(&config_path)?;
        }
    }

    Ok(())
}