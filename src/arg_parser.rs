use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[clap(author = "2XL")]
#[clap(version = "0.1.0")]
#[clap(about = "netcrab is a modern netcat", long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub mode: Mode,
}

#[derive(Subcommand)]
pub enum Mode {
    Connect(ConnectCli),
    Listen(ListenCli),
}

#[derive(Args, Debug)]
#[clap(about = "connect mode")]
pub struct ConnectCli {
    /// Connect address
    pub addr: String,
}

#[derive(Args, Debug)]
#[clap(about = "listen mode")]
pub struct ListenCli {
    /// Listen address
    pub addr: String,
    /// Execte command
    pub cmd: Vec<String>,
}
