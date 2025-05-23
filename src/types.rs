use clap::{Parser, Subcommand};
use crate::commands::client::types::ClientArgs;

/// A CLI tool to help you use ssh client
#[derive(Parser, Debug)]
#[command(version, about, long_about = "A CLI tool to help you use ssh client")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(short, long)]
    pub yes: Option<bool>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Client commands
    Client(ClientArgs),
}