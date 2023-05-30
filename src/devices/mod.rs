use std::{fmt::Debug, time::Duration};

use bluer::{rfcomm::Stream, Address};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    time::timeout,
};

use crate::{DataType, Error, SonyCommand};

pub mod wf1000xm4;

pub const SONY_DEVICES: &[&str] = &["WF-1000XM4"];

pub trait DeviceCommand
where
    Self: Sized + Clone + Debug + TryInto<SonyCommand, Error = Error>,
{
}

pub trait SonyDevice
where
    Self: Sized + Debug,
{
    async fn new(mac: Address) -> Result<Self, Error>;

    async fn send_command<C: DeviceCommand>(stream: &mut Stream, command: C) -> Result<(), Error> {
        let command: SonyCommand = command.try_into()?;
        // println!("Sending {:?}", command);
        let raw_command: Vec<u8> = command.try_into()?;
        // println!("Sending raw {:?}", hex::encode(&raw_command));
        stream
            .write_all(raw_command.as_slice())
            .await
            .map_err(|x| Error::new(x.to_string()))?;
        Ok(())
    }

    async fn send_ack(stream: &mut Stream) -> Result<(), Error> {
        let ack = SonyCommand {
            data_type: DataType::Ack,
            seq_number: 0,
            payload_size: 0,
            payload: vec![],
            checksum: 0,
        };
        let raw_ack: Vec<u8> = ack.try_into()?;

        stream
            .write_all(&raw_ack.as_slice())
            .await
            .map_err(|x| Error::new(x.to_string()))?;

        Ok(())
    }

    async fn read(stream: &mut Stream) -> Result<SonyCommand, Error> {
        let mut buffer = vec![0; 1024];
        let len = stream
            .read(&mut buffer)
            .await
            .map_err(|x| Error::new(x.to_string()))?;

        // print!("Received {:?} bytes:  | {:?}", len, &buffer[0..len]);
        let command = SonyCommand::try_from(&buffer[0..len])?;
        Ok(command)
    }

    async fn wait_ack(stream: &mut Stream) -> Result<(), Error> {
        match timeout(Duration::from_secs(1), Self::read(stream)).await {
            Ok(res) => {
                let cmd = res?;
                match cmd.data_type {
                    DataType::Ack => return Ok(()),
                    _ => {
                        return Err(Error::new(format!("Invalid Ack received: {:?}", cmd)));
                    }
                }
            }
            Err(_) => Err(Error::new("No Ack received".to_string())),
        }
    }

    async fn send_with_ack<C: DeviceCommand>(stream: &mut Stream, command: C) -> Result<(), Error> {
        for _ in 0..3 {
            Self::send_command(stream, command.clone()).await?;
            match Self::wait_ack(stream).await {
                Ok(_) => return Ok(()),
                Err(_) => {}
            }
        }
        Self::send_command(stream, command).await?;
        Self::send_ack(stream).await?;
        Ok(())
    }

    async fn get_device_info(&self) -> Result<DeviceInfo, Error> {
        unimplemented!()
    }
    async fn get_battery_info(&self) -> Result<BatteryInfo, Error> {
        unimplemented!()
    }
    async fn get_registered_devices(&self) -> Result<RegisteredDevices, Error> {
        unimplemented!()
    }

    async fn set_anc(&mut self, _anc: Anc) -> Result<(), Error> {
        unimplemented!()
    }
    async fn get_anc(&mut self) -> Result<Anc, Error> {
        unimplemented!()
    }

    async fn set_equalizer(&mut self, _equalizer: Equalizer) -> Result<(), Error> {
        unimplemented!()
    }
    async fn get_equalizer(&self) -> Result<Equalizer, Error> {
        unimplemented!()
    }

    async fn set_connection_quality(
        &mut self,
        _connection_quality: ConnectionQuality,
    ) -> Result<(), Error> {
        unimplemented!()
    }
    async fn get_connection_quality(&self) -> Result<ConnectionQuality, Error> {
        unimplemented!()
    }

    async fn set_dsee(&mut self, _dsee: bool) -> Result<(), Error> {
        unimplemented!()
    }
    async fn get_dsee(&self) -> Result<bool, Error> {
        unimplemented!()
    }

    async fn set_speak_to_chat(&mut self, _speek_to_chat: bool) -> Result<(), Error> {
        unimplemented!()
    }
    async fn get_speak_to_chat(&self) -> Result<bool, Error> {
        unimplemented!()
    }

    async fn set_auto_power_off(&mut self, _auto_power_off: bool) -> Result<(), Error> {
        unimplemented!()
    }
    async fn get_auto_power_off(&self) -> Result<bool, Error> {
        unimplemented!()
    }

    async fn set_touch_config(&mut self, _touch_sensor: TouchConfig) -> Result<(), Error> {
        unimplemented!()
    }
    async fn get_touch_config(&self) -> Result<TouchConfig, Error> {
        unimplemented!()
    }

    async fn set_on_device_anc(&mut self, _on_device_control: bool) -> Result<(), Error> {
        unimplemented!()
    }
    async fn get_on_device_anc(&self) -> Result<bool, Error> {
        unimplemented!()
    }

    async fn set_bt_multipoint(&mut self, _bt_multipoint: bool) -> Result<(), Error> {
        unimplemented!()
    }
    async fn get_bt_multipoint(&self) -> Result<bool, Error> {
        unimplemented!()
    }
}

// TODO: Implement
pub struct DeviceInfo {}
pub enum BatteryInfo {
    Headphones(u8),
    // Left, Right, case
    Earbuds(u8, u8, u8),
}

pub struct RegisteredDevices {}

pub enum Anc {
    AmbientSound { level: u8, voice: bool },
    NoiseCanceling { wind: bool },
    Off,
}

#[derive(Debug, Clone, Copy)]
pub enum EqualizerProfile {
    Off,
    Custom1,
    Custom2,
}

#[derive(Debug, Clone, Copy)]
pub enum Bands {
    Zero(),
    FiveBandsAndBass {
        bass: i8,
        b400k: i8,
        b1k: i8,
        b2k5: i8,
        b6k3: i8,
        b16k: i8,
    },
}

impl Bands {
    pub fn validate(&self) -> Result<(), Error> {
        match self {
            Bands::Zero() => Ok(()),
            Bands::FiveBandsAndBass {
                bass,
                b400k,
                b1k,
                b2k5,
                b6k3,
                b16k,
            } => {
                if *bass <= -10 || *bass >= 10 {
                    return Err(Error::new(format!("Invalid bass value: {:?}", bass)));
                }
                if *b400k <= -10 || *b400k >= 10 {
                    return Err(Error::new(format!("Invalid 400k value: {:?}", b400k)));
                }
                if *b1k <= -10 || *b1k >= 10 {
                    return Err(Error::new(format!("Invalid 1k value: {:?}", b1k)));
                }
                if *b2k5 <= -10 || *b2k5 >= 10 {
                    return Err(Error::new(format!("Invalid 2k5 value: {:?}", b2k5)));
                }
                if *b6k3 <= -10 || *b6k3 >= 10 {
                    return Err(Error::new(format!("Invalid 6k3 value: {:?}", b6k3)));
                }
                if *b16k <= -10 || *b16k >= 10 {
                    return Err(Error::new(format!("Invalid 16k value: {:?}", b16k)));
                }
                Ok(())
            }
        }
    }
}

pub struct Equalizer {
    pub profile: EqualizerProfile,
    pub bands: Bands,
}

// TODO: Implement
pub struct TouchConfig {}

pub enum ConnectionQuality {
    Stable,
    Quality,
}
