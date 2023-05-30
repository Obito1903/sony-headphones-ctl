use clap::{self, Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "sony-ctl")]
#[command(author, version, about = "Sony Headphones CLI", long_about = None)]

pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    // #[clap(subcommand)]
    // Report(Report),
    #[command(subcommand)]
    Config(Config),
}

#[derive(Subcommand)]
pub enum Report {
    #[command(subcommand)]
    // TODO: Add subcommand
    Battery,
    #[command(subcommand)]
    // TODO: Add subcommand
    DeviceInfo,
    // TODO: Add subcommand
    #[command(subcommand)]
    RegisteredDevices,
}

#[derive(Subcommand)]
pub enum Config {
    #[command(subcommand, about = "Set Ambient Sound Control")]
    ANC(AmbientSoundControl),
    #[command(subcommand, about = "Set Equalizer")]
    Eq(EqualizerControl),
    #[command(subcommand, about = "Toggle DSEE Extreme")]
    DSEE(Toggle),
    #[command(subcommand, about = "Toggle Speak-to-Chat")]
    Stc(Toggle),
    #[command(subcommand, about = "Toggle Wearing Detection")]
    WearDetection(Toggle),
    #[command(subcommand, about = "Toggle Auto Power Off")]
    AutoPowerOff(Toggle),
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

#[derive(Subcommand)]
pub enum Toggle {
    On,
    Off,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum EqualizerProfile {
    Off,
    Custom1,
    Custom2,
}

// #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
// pub enum Bands {
//     FiveBandsAndBass {
//         bass: i8,
//         b400k: i8,
//         b1k: i8,
//         b2k5: i8,
//         b6k3: i8,
//         b16k: i8,
//     },
// }

#[derive(Subcommand)]
pub enum EqualizerControl {
    Profile {
        profile: EqualizerProfile,
    },
    SixBand {
        profile: EqualizerProfile,
        #[arg(value_parser = clap::value_parser!(i8).range(-10..11))]
        bass: i8,
        #[arg(value_parser = clap::value_parser!(i8).range(-10..11))]
        b400k: i8,
        #[arg(value_parser = clap::value_parser!(i8).range(-10..11))]
        b1k: i8,
        #[arg(value_parser = clap::value_parser!(i8).range(-10..11))]
        b2k5: i8,
        #[arg(value_parser = clap::value_parser!(i8).range(-10..11))]
        b6k3: i8,
        #[arg(value_parser = clap::value_parser!(i8).range(-10..11))]
        b16k: i8,
    },
}
