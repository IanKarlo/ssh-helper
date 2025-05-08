use clap::{Args, Subcommand};

#[derive(Args, Debug)]
pub struct ClientArgs {
    #[command(subcommand)]
    pub command: ClientCommands,
}

#[derive(Subcommand, Debug)]
pub enum ClientCommands {
    Tunnel {
        /// Port number
        #[arg(short = 'l', long)]
        tunnel_local_port: Option<u16>,

        /// Host name
        #[arg(short = 't', long)]
        tunnel_host: Option<String>,

        /// Host port number
        #[arg(short = 'p', long)]
        tunnel_host_port: Option<u16>,

        /// ssh username
        #[arg(short = 'u', long)]
        tunnel_username: Option<String>,

        /// ssh connection password
        #[arg(short = 'P', long)]
        tunnel_password: Option<String>,
    },
}