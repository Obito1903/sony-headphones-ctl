pub mod args;

use args::{Cli, Commands};
use clap::Parser;
use sony_headphone_ctl::devices::{
    wf1000xm4::Wf1000xm4, Anc, Bands, Equalizer, EqualizerProfile, SonyDevice,
};

async fn process<D: SonyDevice>(args: Cli, mut device: D) {
    match args.command {
        Commands::Config(config) => match config {
            args::Config::ANC(ambient_sound) => match ambient_sound {
                args::AmbientSoundControl::Ambient { level, voice } => {
                    device
                        .set_anc(Anc::AmbientSound { level, voice })
                        .await
                        .unwrap();
                }
                args::AmbientSoundControl::NC { wind } => {
                    device.set_anc(Anc::NoiseCanceling { wind }).await.unwrap();
                }
                args::AmbientSoundControl::Off => {
                    device.set_anc(Anc::Off).await.unwrap();
                }
            },
            args::Config::DSEE(dsee) => match dsee {
                args::DseeControl::On => device.set_dsee(true).await.unwrap(),
                args::DseeControl::Off => device.set_dsee(false).await.unwrap(),
            },
            args::Config::Stc(stc) => match stc {
                args::SpeekToChatControl::On => device.set_speak_to_chat(true).await.unwrap(),
                args::SpeekToChatControl::Off => device.set_speak_to_chat(false).await.unwrap(),
            },
            args::Config::Eq(eq) => match eq {
                args::EqualizerControl::Profile { profile } => {
                    let eq_profile = match profile {
                        args::EqualizerProfile::Off => EqualizerProfile::Off,
                        args::EqualizerProfile::Custom1 => EqualizerProfile::Custom1,
                        args::EqualizerProfile::Custom2 => EqualizerProfile::Custom2,
                    };
                    device
                        .set_equalizer(Equalizer {
                            profile: eq_profile,
                            bands: Bands::Zero(),
                        })
                        .await
                        .unwrap();
                }
                args::EqualizerControl::SixBand {
                    profile,
                    bass,
                    b400k,
                    b1k,
                    b2k5,
                    b6k3,
                    b16k,
                } => {
                    let eq_profile = match profile {
                        args::EqualizerProfile::Off => EqualizerProfile::Off,
                        args::EqualizerProfile::Custom1 => EqualizerProfile::Custom1,
                        args::EqualizerProfile::Custom2 => EqualizerProfile::Custom2,
                    };
                    device
                        .set_equalizer(Equalizer {
                            profile: eq_profile,
                            bands: Bands::FiveBandsAndBass {
                                bass,
                                b400k,
                                b1k,
                                b2k5,
                                b6k3,
                                b16k,
                            },
                        })
                        .await
                        .unwrap();
                }
            },
            _ => {}
        },
    }
}

#[tokio::main]
async fn main() -> bluer::Result<()> {
    let args = args::Cli::parse();

    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    adapter.set_powered(true).await?;

    let devices_addr = adapter.device_addresses().await?;

    for device_addr in devices_addr {
        let bt_device = adapter.device(device_addr).unwrap();
        match bt_device.name().await.unwrap() {
            Some(name) => match name.as_str() {
                "WF-1000XM4" => {
                    process(args, Wf1000xm4::new(device_addr).await.unwrap()).await;
                    break;
                }
                _ => {}
            },
            _ => {}
        }
    }
    Ok(())
}
