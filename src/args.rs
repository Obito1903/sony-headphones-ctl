use clap::{self, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "sony-ctl")]
#[command(author, version, about = "Sony Headphones CLI", long_about = None)]

pub struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    // #[clap(subcommand)]
    // Report(Report),
    #[clap(subcommand)]
    Config(Config),
}

#[derive(Subcommand)]
pub enum Report {
    #[clap(subcommand)]
    // TODO: Add subcommand
    Battery,
    #[clap(subcommand)]
    // TODO: Add subcommand
    DeviceInfo,
    // TODO: Add subcommand
    #[clap(subcommand)]
    RegisteredDevices,
}

#[derive(Subcommand)]
pub enum Config {
    #[clap(subcommand)]
    ANC(AmbientSoundControl),
}

#[derive(Subcommand)]
pub enum AmbientSoundControl {
    #[command(about = "Set Ambient Sound Mode")]
    Ambient {
        #[arg(short, long)]
        level: u8,
        #[arg(short, long)]
        voice: bool,
    },
    #[command(about = "Set Noise Canceling Mode")]
    NC {
        #[arg(short, long)]
        wind: bool,
    },
    #[command(about = "Disable Ambient Sound Control")]
    Off,
}
