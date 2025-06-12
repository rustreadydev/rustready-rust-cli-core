use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to config file
    #[clap(short, long, default_value = "templates/config.json")]
    pub config: Option<String>,

    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Run the application with a name
    Run {
        /// Name to greet
        name: String,
    },
    /// Show version
    Version,
}