use std::time::Duration;

use bluer::{
    rfcomm::{SocketAddr, Stream},
    Address,
};
use derive_try_from_primitive::TryFromPrimitive;

use crate::{DataType, Error, SonyCommand};

use self::{anc::AncWF1000XM4, equalizer::EqualizerCommand};

use super::{DeviceCommand, DeviceMessage, Equalizer, SonyDevice};

pub mod anc;
pub mod equalizer;

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

    fn get_stream(&mut self) -> &mut Stream {
        &mut self.stream
    }

    fn decode(command: SonyCommand) -> Result<super::DeviceMessage, Error> {
        match command.data_type {
            DataType::DataMdr => {
                match MessageCode::try_from(u16::from_be_bytes([
                    command.payload[0],
                    command.payload[1],
                ]))
                .map_err(|x| Error::new(format!("Unknown command type {:?}", x)))?
                {
                    MessageCode::AncAck | MessageCode::AncSet => Ok(super::DeviceMessage::Anc(
                        AncWF1000XM4::try_from(command.payload)?.try_into()?,
                    )),
                    MessageCode::EqAck | MessageCode::EqSet => {
                        Ok(EqualizerCommand::try_from(command.payload)?.try_into()?)
                    }
                    MessageCode::DseeAck | MessageCode::DseeSet => {
                        Ok(super::DeviceMessage::Dsee(command.payload[2] == 0x01))
                    }
                    MessageCode::StcAck | MessageCode::StcSet => Ok(
                        super::DeviceMessage::SpeakToChat(command.payload[2] == 0x00),
                    ),
                    MessageCode::AutoPowerAck | MessageCode::AutoPowerSet => Ok(
                        super::DeviceMessage::AutoPowerOff(command.payload[2] == 0x10),
                    ),
                    MessageCode::WearDetectionAck | MessageCode::WearDetectionSet => Ok(
                        super::DeviceMessage::WearDetection(command.payload[2] == 0x00),
                    ),
                    ct => unimplemented!("Unimplemented command type {:?}", ct),
                }
            }
            DataType::Ack => Ok(DeviceMessage::Ack()),
            _ => Err(Error::new(format!(
                "Unknown data type {:?}",
                command.data_type
            ))),
        }
    }

    async fn encode(option: DeviceMessage) -> Result<SonyCommand, Error> {
        match option {
            DeviceMessage::Anc(anc) => Ok(AncWF1000XM4::try_from(anc)?.try_into()?),
            DeviceMessage::Equalizer { profile, bands } => {
                Ok(EqualizerCommand::try_from(Equalizer { profile, bands })?.try_into()?)
            }
            DeviceMessage::Dsee(dsee) => Ok(DseeCommand {
                command: MessageCode::DseeSet,
                enable: dsee,
            }
            .try_into()?),
            DeviceMessage::SpeakToChat(stc) => Ok(StcCommand {
                command: MessageCode::StcSet,
                enable: stc,
                _unknown: 0x01,
            }
            .try_into()?),
            DeviceMessage::AutoPowerOff(auto_power_off) => Ok(AutoPowerOffCommand {
                command: MessageCode::AutoPowerSet,
                enable: match auto_power_off {
                    true => ApoEnable::On,
                    false => ApoEnable::Off,
                },
                _unknown: 0x00,
            }
            .try_into()?),
            DeviceMessage::WearDetection(wear_detection) => Ok(PauseRemovedCommand {
                command: MessageCode::WearDetectionSet,
                enable: wear_detection,
            }
            .try_into()?),
            _ => unimplemented!(),
        }
    }

    async fn set(&mut self, option: DeviceMessage) -> Result<(), Error> {
        match option {
            _ => Self::set_and_confirm(&mut self.stream, option).await?,
            // _ => {}
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, TryFromPrimitive)]
#[repr(u16)]
pub enum MessageCode {
    AncSet = 0x6815,
    AncAck = 0x6915,
    DseeSet = 0xe801,
    DseeAck = 0xe901,
    StcSet = 0xf802,
    StcAck = 0xf902,
    EqSet = 0x5800,
    EqAck = 0x5900,
    AutoPowerSet = 0x2805,
    AutoPowerAck = 0x2905,
    WearDetectionSet = 0xf801,
    WearDetectionAck = 0xf901,
    AudioQualitySet = 0xe800,
    AudioQualityAck = 0xe900,
    NotificationSet = 0x4801,
    NotificationAck = 0x4901,
    TouchControlSet = 0xf803,
    TouchControlAck = 0xf903,
}

#[derive(Debug, Clone, Copy)]
struct DseeCommand {
    command: MessageCode,
    enable: bool,
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

#[derive(Debug, Clone, Copy)]
struct StcCommand {
    command: MessageCode,
    enable: bool,
    // Always 0x01
    _unknown: u8,
}

impl DeviceCommand for StcCommand {}

impl TryInto<SonyCommand> for StcCommand {
    type Error = Error;

    fn try_into(self) -> Result<SonyCommand, Self::Error> {
        let mut bytes = vec![];

        bytes.extend_from_slice(&(self.command as u16).to_be_bytes());
        bytes.push(!self.enable as u8);
        bytes.push(self._unknown);

        Ok(SonyCommand {
            data_type: DataType::DataMdr,
            seq_number: 0,
            payload_size: bytes.len() as u8,
            payload: bytes,
            checksum: 0,
        })
    }
}

#[derive(Debug, Clone, Copy, TryFromPrimitive)]
#[repr(u8)]
pub enum ApoEnable {
    Off = 0x11,
    On = 0x10,
}

#[derive(Debug, Clone, Copy)]
pub struct AutoPowerOffCommand {
    command: MessageCode,
    enable: ApoEnable,
    // Always 0x00
    _unknown: u8,
}

impl DeviceCommand for AutoPowerOffCommand {}

impl TryInto<SonyCommand> for AutoPowerOffCommand {
    type Error = Error;

    fn try_into(self) -> Result<SonyCommand, Self::Error> {
        let mut bytes = vec![];

        bytes.extend_from_slice(&(self.command as u16).to_be_bytes());
        bytes.push(self.enable as u8);
        bytes.push(self._unknown);

        Ok(SonyCommand {
            data_type: DataType::DataMdr,
            seq_number: 0,
            payload_size: bytes.len() as u8,
            payload: bytes,
            checksum: 0,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PauseRemovedCommand {
    command: MessageCode,
    enable: bool,
}

impl DeviceCommand for PauseRemovedCommand {}

impl TryInto<SonyCommand> for PauseRemovedCommand {
    type Error = Error;

    fn try_into(self) -> Result<SonyCommand, Self::Error> {
        let mut bytes = vec![];

        bytes.extend_from_slice(&(self.command as u16).to_be_bytes());
        bytes.push(!self.enable as u8);

        Ok(SonyCommand {
            data_type: DataType::DataMdr,
            seq_number: 0,
            payload_size: bytes.len() as u8,
            payload: bytes,
            checksum: 0,
        })
    }
}
