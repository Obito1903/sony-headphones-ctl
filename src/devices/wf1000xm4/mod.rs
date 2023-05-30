use std::time::Duration;

use bluer::{
    rfcomm::{SocketAddr, Stream},
    Address,
};
use derive_try_from_primitive::TryFromPrimitive;

use crate::{DataType, Error, SonyCommand};

use self::anc::{AncCommand, AncMode, AsLevel, WindCode};

use super::{Anc, DeviceCommand, SonyDevice};

pub mod anc;

#[derive(Debug)]
pub struct Wf1000xm4 {
    stream: Stream,
    _mac: Address,
}

impl SonyDevice for Wf1000xm4 {
    async fn new(mac: Address) -> Result<Self, Error> {
        let target_sa = SocketAddr::new(mac, 9);

        // println!("Connecting to [{}]... ", mac);
        let stream = Stream::connect(target_sa).await.map_err(|x| match x {
            _ => Error::new(x.to_string()),
        })?;
        // wait for the connection to be established
        tokio::time::sleep(Duration::from_millis(500)).await;
        // println!("Connected!");
        Ok(Self {
            stream: stream,
            _mac: mac,
        })
    }

    async fn set_anc(&mut self, anc: Anc) -> Result<(), Error> {
        let command: AncCommand = match anc {
            Anc::AmbientSound { level, voice } => AncCommand {
                command: CommandTypes::AncSet,
                continuous: false,
                anc_enable: true,
                anc_mode: AncMode::AmbientSound,
                nc_wind: WindCode::NoWind,
                as_voice: voice,
                as_level: level
                    .try_into()
                    .map_err(|x| Error::new(format!("Invalid ANC level {:?}", x)))?,
            },
            Anc::NoiseCanceling { wind } => AncCommand {
                command: CommandTypes::AncSet,
                continuous: false,
                anc_enable: true,
                anc_mode: AncMode::NoiseCanceling,
                nc_wind: if wind {
                    WindCode::Wind
                } else {
                    WindCode::NoWind
                },
                as_voice: false,
                as_level: AsLevel::Level1,
            },
            Anc::Off => AncCommand {
                command: CommandTypes::AncSet,
                continuous: false,
                anc_enable: false,
                anc_mode: AncMode::NoiseCanceling,
                nc_wind: WindCode::NoWind,
                as_voice: false,
                as_level: AsLevel::Level1,
            },
        };

        Self::send_with_ack(&mut self.stream, command).await?;
        Ok(())
    }

    async fn set_dsee(&mut self, dsee: bool) -> Result<(), Error> {
        let command: DseeCommand = DseeCommand {
            command: CommandTypes::DseeSet,
            enable: dsee,
        };

        Self::send_with_ack(&mut self.stream, command).await?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, TryFromPrimitive)]
#[repr(u16)]
pub enum CommandTypes {
    AncSet = 0x6815,
    AncAck = 0x6915,
    DseeSet = 0xe801,
}

#[derive(Debug, Clone, Copy)]
pub struct DseeCommand {
    pub command: CommandTypes,
    pub enable: bool,
}

impl DeviceCommand for DseeCommand {}

impl TryInto<SonyCommand> for DseeCommand {
    type Error = Error;

    fn try_into(self) -> Result<SonyCommand, Self::Error> {
        let mut bytes = vec![];

        bytes.extend_from_slice(&(self.command as u16).to_be_bytes());
        bytes.push(self.enable as u8);

        Ok(SonyCommand {
            data_type: DataType::DataMdr,
            seq_number: 0,
            payload_size: bytes.len() as u8,
            payload: bytes,
            checksum: 0,
        })
    }
}
