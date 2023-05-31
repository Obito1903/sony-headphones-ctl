use crate::{
    devices::{DeviceCommand, DeviceMessage, Equalizer, EqualizerProfile},
    DataType, Error, SonyCommand,
};

use super::MessageCode;

#[derive(Debug, Clone)]
pub struct EqualizerCommand {
    pub command: MessageCode,
    pub preset: u8,
    pub nb_bands: u8,
    pub bands: Vec<u8>,
}

impl TryFrom<Equalizer> for EqualizerCommand {
    type Error = Error;

    fn try_from(equalizer: Equalizer) -> Result<Self, Self::Error> {
        let mut bands = vec![];

        match equalizer.bands {
            crate::devices::Bands::Zero() => {}
            crate::devices::Bands::FiveBandsAndBass {
                bass,
                b400k,
                b1k,
                b2k5,
                b6k3,
                b16k,
            } => {
                bands.push((bass + 10) as u8);
                bands.push((b400k + 10) as u8);
                bands.push((b1k + 10) as u8);
                bands.push((b2k5 + 10) as u8);
                bands.push((b6k3 + 10) as u8);
                bands.push((b16k + 10) as u8);
            }
        }

        Ok(Self {
            command: MessageCode::EqSet,
            preset: match equalizer.profile {
                EqualizerProfile::Off => 0x00,
                EqualizerProfile::Custom1 => 0xa1,
                EqualizerProfile::Custom2 => 0xa2,
            },
            nb_bands: bands.len() as u8,
            bands,
        })
    }
}

impl DeviceCommand for EqualizerCommand {}

impl TryFrom<Vec<u8>> for EqualizerCommand {
    type Error = Error;

    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        let command =
            MessageCode::try_from(u16::from_be_bytes([bytes[0], bytes[1]])).map_err(|x| {
                Error::new(format!(
                    "EqualizerCommand::try_from: Unsupported command: {:?}",
                    x
                ))
            })?;
        let preset = bytes[2];
        let nb_bands = bytes[3];
        let bands = bytes[4..].to_vec();

        Ok(Self {
            command: command,
            preset,
            nb_bands,
            bands,
        })
    }
}

impl TryInto<DeviceMessage> for EqualizerCommand {
    type Error = Error;

    fn try_into(self) -> Result<DeviceMessage, Self::Error> {
        let profile = self.preset.try_into().map_err(|_| {
            Error::new(format!(
                "EqualizerCommand::try_into: Unsupported preset: {:?}",
                self.preset
            ))
        })?;
        match self.nb_bands {
            6 => Ok(DeviceMessage::Equalizer {
                profile: profile,
                bands: crate::devices::Bands::FiveBandsAndBass {
                    bass: self.bands[0] as i8 - 10,
                    b400k: self.bands[1] as i8 - 10,
                    b1k: self.bands[2] as i8 - 10,
                    b2k5: self.bands[3] as i8 - 10,
                    b6k3: self.bands[4] as i8 - 10,
                    b16k: self.bands[5] as i8 - 10,
                },
            }),
            0 => Ok(DeviceMessage::Equalizer {
                profile: profile,
                bands: crate::devices::Bands::Zero(),
            }),
            _ => Err(Error::new(format!(
                "EqualizerCommand::try_into: Unsupported number of bands: {:?}",
                self.nb_bands
            ))),
        }
    }
}

impl TryInto<SonyCommand> for EqualizerCommand {
    type Error = Error;

    fn try_into(self) -> Result<SonyCommand, Self::Error> {
        let mut bytes = vec![];

        bytes.extend_from_slice(&(self.command as u16).to_be_bytes());
        bytes.push(self.preset as u8);
        bytes.push(self.nb_bands);
        bytes.extend_from_slice(&self.bands);
        // println!("EqualizerCommand: {:?}", bytes);

        Ok(SonyCommand {
            data_type: DataType::DataMdr,
            seq_number: 0,
            payload_size: bytes.len() as u8,
            payload: bytes,
            checksum: 0,
        })
    }
}
