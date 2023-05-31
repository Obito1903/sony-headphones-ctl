use std::{collections::HashMap, fmt::Debug, time::Duration};

use bluer::{rfcomm::Stream, Address};
use derive_try_from_primitive::TryFromPrimitive;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    time::timeout,
};

use crate::{DataType, Error, SonyCommand};

pub mod wf1000xm4;

pub const SONY_DEVICES: &[&str] = &["WF-1000XM4"];

pub trait DeviceCommand
where
    Self: Debug + TryInto<SonyCommand, Error = Error>,
{
}

pub trait SonyDevice
where
    Self: Sized + Debug,
{
    async fn new(mac: Address) -> Result<Self, Error>;

    fn get_stream(&mut self) -> &mut Stream;

    async fn send(stream: &mut Stream, command: SonyCommand) -> Result<(), Error> {
        // println!("Sending {:?}", command);
        let raw_command: Vec<u8> = command.try_into()?;
        // println!("Sending raw {:?}", hex::encode(&raw_command));
        stream
            .write_all(raw_command.as_slice())
            .await
            .map_err(|x| Error::new(x.to_string()))?;
        Ok(())
    }

    async fn send_hex(stream: &mut Stream, data_type: DataType, hex: Vec<u8>) -> Result<(), Error> {
        let command = SonyCommand {
            data_type: data_type,
            seq_number: 0,
            payload_size: hex.len() as u8,
            payload: hex,
            checksum: 0,
        };
        let raw_command: Vec<u8> = command.try_into()?;
        // println!("Sending raw {:?}", hex::encode(&raw_command));
        stream
            .write_all(&raw_command.as_slice())
            .await
            .map_err(|x| Error::new(x.to_string()))?;
        Ok(())
    }

    async fn listen(stream: &mut Stream) {
        loop {
            let command = Self::read(stream).await.unwrap();
            // println!("Received {:?}", command);
        }
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
                    DataType::Ack => {
                        // println!("Received Ack");
                        return Ok(());
                    }
                    _ => {
                        return Err(Error::new(format!("Invalid Ack received: {:?}", cmd)));
                    }
                }
            }
            Err(_) => Err(Error::new("No Ack received".to_string())),
        }
    }

    async fn send_wait_ack(stream: &mut Stream, command: SonyCommand) -> Result<(), Error> {
        for _ in 0..3 {
            Self::send(stream, command.clone()).await?;
            match Self::wait_ack(stream).await {
                Ok(_) => return Ok(()),
                Err(_) => {}
            }
        }
        Ok(())
    }

    fn decode(_command: SonyCommand) -> Result<DeviceMessage, Error> {
        unimplemented!()
    }

    async fn encode(_option: DeviceMessage) -> Result<SonyCommand, Error> {
        unimplemented!()
    }

    async fn set_and_ack(stream: &mut Stream, option: DeviceMessage) -> Result<(), Error> {
        let command = Self::encode(option).await?;
        Self::send_wait_ack(stream, command).await
    }

    async fn set_and_confirm(stream: &mut Stream, option: DeviceMessage) -> Result<(), Error> {
        let command = Self::encode(option).await?;
        Self::send_wait_ack(stream, command).await?;
        let current_state = Self::decode(Self::read(stream).await?)?;

        println!("Current state: {:?}", current_state);

        Ok(())
    }

    async fn set(&mut self, option: DeviceMessage) -> Result<(), Error>;
}

// TODO: Implement

#[derive(Debug)]
pub enum DeviceMessage {
    RegisteredDevices(Vec<RegisteredDevice>),
    BatteryInfo(BatteryInfo),
    DeviceInfo(HashMap<String, String>),
    Anc(Anc),
    Equalizer {
        profile: EqualizerProfile,
        bands: Bands,
    },
    ConnectionQuality(bool),
    Dsee(bool),
    SpeakToChat(bool),
    AutoPowerOff(bool),
    WearDetection(bool),
    TouchConfig(TouchConfig),
    OnDeviceAnc {
        asm: bool,
        nc: bool,
        off: bool,
    },
    BtMultipoint(bool),
    Ack(),
    Unknown(Vec<u8>),
}

#[derive(Debug)]
pub struct RegisteredDevice {
    pub name: String,
    pub mac: Address,
}

#[derive(Debug)]
pub enum BatteryInfo {
    Headphones(u8),
    // Left, Right, case
    Earbuds(u8, u8, u8),
}

#[derive(Debug)]
pub enum Anc {
    AmbientSound { level: u8, voice: bool },
    NoiseCanceling { wind: bool },
    Off,
}

#[derive(Debug, Clone, Copy, TryFromPrimitive)]
#[repr(u8)]
pub enum EqualizerProfile {
    Off = 0x00,
    Custom1 = 0xa1,
    Custom2 = 0xa2,
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

#[derive(Debug)]
// TODO: Implement
pub struct TouchConfig {}

pub enum ConnectionQuality {
    Stable,
    Quality,
}
