use clap::{self, Args, Parser, Subcommand, ValueEnum};
use sony_headphone_ctl::devices::{self};

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
    Debug(Debug),
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

#[derive(Args)]
pub struct Debug {
    #[arg(short, long)]
    pub listen: bool,
    pub data_type: u8,
    pub hex: String,
}

#[derive(Subcommand)]
pub enum AmbientSoundControl {
    #[command(about = "Set Ambient Sound Mode")]
    Ambient {
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

impl Into<bool> for Toggle {
    fn into(self) -> bool {
        match self {
            Toggle::On => true,
            Toggle::Off => false,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum EqualizerProfile {
    Off,
    Custom1,
    Custom2,
}

impl TryInto<devices::EqualizerProfile> for EqualizerProfile {
    type Error = ();

    fn try_into(self) -> Result<devices::EqualizerProfile, Self::Error> {
        match self {
            EqualizerProfile::Off => Ok(devices::EqualizerProfile::Off),
            EqualizerProfile::Custom1 => Ok(devices::EqualizerProfile::Custom1),
            EqualizerProfile::Custom2 => Ok(devices::EqualizerProfile::Custom2),
        }
    }
}

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

impl TryInto<devices::DeviceMessage> for EqualizerControl {
    type Error = ();

    fn try_into(self) -> Result<devices::DeviceMessage, Self::Error> {
        match self {
            EqualizerControl::Profile { profile } => Ok(devices::DeviceMessage::Equalizer {
                profile: profile.try_into().unwrap(),
                bands: devices::Bands::Zero(),
            }),
            EqualizerControl::SixBand {
                profile,
                bass,
                b400k,
                b1k,
                b2k5,
                b6k3,
                b16k,
            } => Ok(devices::DeviceMessage::Equalizer {
                profile: profile.try_into().unwrap(),
                bands: devices::Bands::FiveBandsAndBass {
                    bass,
                    b400k,
                    b1k,
                    b2k5,
                    b6k3,
                    b16k,
                },
            }),
        }
    }
}
