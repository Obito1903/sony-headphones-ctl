use crate::{
    devices::{DeviceCommand, Equalizer, EqualizerProfile},
    DataType, Error, SonyCommand,
};

use super::CommandTypes;

#[derive(Debug, Clone)]
pub struct EqualizerCommand {
    pub command: CommandTypes,
    pub preset: u8,
    pub nb_bands: u8,
    pub bands: Vec<u8>,
}

impl TryFrom<Equalizer> for EqualizerCommand {
    type Error = Error;

    fn try_from(equalizer: Equalizer) -> Result<Self, Self::Error> {
        let mut bands = vec![];

        match equalizer.bands {
            crate::devices::Bands::Zero() => {
                bands.push(0x00);
            }
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
            command: CommandTypes::EqSet,
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

impl TryInto<SonyCommand> for EqualizerCommand {
    type Error = Error;

    fn try_into(self) -> Result<SonyCommand, Self::Error> {
        let mut bytes = vec![];

        bytes.extend_from_slice(&(self.command as u16).to_be_bytes());
        bytes.push(self.preset as u8);
        bytes.push(self.nb_bands);
        bytes.extend_from_slice(&self.bands);
        println!("EqualizerCommand: {:?}", bytes);

        Ok(SonyCommand {
            data_type: DataType::DataMdr,
            seq_number: 0,
            payload_size: bytes.len() as u8,
            payload: bytes,
            checksum: 0,
        })
    }
}
