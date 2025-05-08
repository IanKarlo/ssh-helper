use clap::{Parser, Subcommand};
use crate::commands::client;

/// A CLI tool to help you use ssh client
#[derive(Parser, Debug)]
#[command(version, about, long_about = "A CLI tool to help you use ssh client")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Client commands
    Client(client::ClientArgs),
}