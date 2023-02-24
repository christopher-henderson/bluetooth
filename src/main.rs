mod bluetoothctl;
mod profile;

use std::path::PathBuf;

use crate::bluetoothctl::Pairing;
use crate::profile::ProfileConfig;
use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Bluetooth {
    #[arg(long, short)]
    profile: String,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Connect,
    Disconnect,
}

impl Bluetooth {
    pub fn run(&self) {
        match self.command {
            Commands::Connect => ProfileConfig::load(&self.profile).connect(),
            Commands::Disconnect => ProfileConfig::load(&self.profile).disconnect(),
        }
    }
}

fn main() {
    Bluetooth::parse().run();
}
