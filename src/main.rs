use clap::Parser;
use env_logger::Env;
use log::info;
use std::path::Path;

mod cli;
mod config;
mod error;

use cli::Cli;
use config::Config;
use error::Result;

fn main() -> Result<()> {
    // Initialize logging
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("Starting CLI application");

    // Parse CLI arguments
    let cli = Cli::parse();

    // Load or create config
    let config_path = cli.config.unwrap_or_else(|| "templates/config.json".to_string());
    let mut config = if Path::new(&config_path).exists() {
        Config::load(&config_path)?
    } else {
        Config::default(&config_path)?
    };

    // Example command handling
    match cli.command {
        cli::Command::Run { name } => {
            info!("Running with name: {}", name);
            config.last_run = name.clone();
            config.save(&config_path)?;
            println!("Hello, {}! Config updated.", name);
        }
        cli::Command::Version => {
            println!("Version: {}", env!("CARGO_PKG_VERSION"));
        }
    }

    Ok(())
}
