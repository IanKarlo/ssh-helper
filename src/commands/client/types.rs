use clap::{Args, Subcommand};

#[derive(Args, Debug)]
pub struct ClientArgs {
    #[command(subcommand)]
    pub command: ClientCommands,
}

#[derive(Subcommand, Debug)]
pub enum ClientCommands {
    /// Starts a Tunnel to the server
    Tunnel {
        /// Local Port number
        #[arg(short = 'l', long)]
        tunnel_local_port: u16,

        /// Remote Host name
        #[arg(short = 't', long)]
        tunnel_host: String,

        /// Remote Host port number
        #[arg(short = 'p', long)]
        tunnel_host_port: u16,

        /// Ssh connection username
        #[arg(short = 'u', long)]
        tunnel_username: String,

        /// Ssh connection password
        #[arg(short = 'P', long)]
        tunnel_password: Option<String>,

        /// Ssh connection key
        #[arg(short = 'k', long)]
        tunnel_key: Option<String>,
    },
}