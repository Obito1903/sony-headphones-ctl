pub mod args;

use args::{Cli, Commands};
use clap::Parser;
use sony_headphone_ctl::devices::{wf1000xm4::Wf1000xm4, Anc, DeviceMessage, SonyDevice};

async fn process<D: SonyDevice>(args: Cli, mut device: D) {
    match args.command {
        Commands::Config(config) => {
            let option = match config {
                args::Config::ANC(anc) => DeviceMessage::Anc(match anc {
                    args::AmbientSoundControl::Off => Anc::Off,
                    args::AmbientSoundControl::Ambient { level, voice } => {
                        Anc::AmbientSound { level, voice }
                    }
                    args::AmbientSoundControl::NC { wind } => Anc::NoiseCanceling { wind },
                }),
                args::Config::Eq(eq) => eq.try_into().unwrap(),
                args::Config::DSEE(dsee) => DeviceMessage::Dsee(dsee.into()),
                args::Config::Stc(stc) => DeviceMessage::SpeakToChat(stc.into()),
                args::Config::AutoPowerOff(apo) => DeviceMessage::AutoPowerOff(apo.into()),
                args::Config::WearDetection(wd) => DeviceMessage::WearDetection(wd.into()),
                // _ => DeviceOption::Unknown(vec![]),
            };

            device.set(option).await.unwrap();
        }
        Commands::Debug(debug) => match debug {
            args::Debug {
                listen,
                data_type,
                hex,
            } => {
                <D as SonyDevice>::send_hex(
                    device.get_stream(),
                    data_type.try_into().unwrap(),
                    hex::decode(hex).unwrap(),
                )
                .await
                .unwrap();

                if listen {
                    <D as SonyDevice>::listen(device.get_stream()).await;
                }
            }
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
