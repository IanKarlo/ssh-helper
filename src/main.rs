mod commands;
mod utils;
mod types;

use clap::Parser;
use commands::client::{types::ClientCommands, tunnel as client_tunnel};
use utils::configure_ssh;
use types::{Cli, Commands};

fn main() {
    let cli = Cli::try_parse();

    if let Err(e) = cli {
        println!("Error: {}", e);
        return;
    }

    let cli = cli.unwrap();

    configure_ssh();

    match cli.command {
        Commands::Client(client_args) => {
            match client_args.command {
                ClientCommands::Tunnel {
                    tunnel_local_port,
                    tunnel_host,
                    tunnel_host_port,
                    tunnel_username,
                    tunnel_password,
                } => {
                    client_tunnel::run_tunnel(
                        tunnel_local_port,
                        tunnel_host,
                        tunnel_host_port,
                        tunnel_username,
                        tunnel_password,
                    );
                }
            }
        }
    }
}
