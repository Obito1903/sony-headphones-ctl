use derive_try_from_primitive::TryFromPrimitive;

use crate::{
    devices::{Anc, DeviceCommand},
    DataType, Error, SonyCommand,
};

use super::MessageCode;

#[derive(Debug, Clone, Copy, TryFromPrimitive)]
#[repr(u8)]
pub enum AncMode {
    NoiseCanceling = 0x00,
    AmbientSound = 0x01,
}

#[derive(Debug, Clone, Copy, TryFromPrimitive)]
#[repr(u8)]
pub enum AsLevel {
    Level1 = 0x01,
    Level2 = 0x02,
    Level3 = 0x03,
    Level4 = 0x04,
    Level5 = 0x05,
    Level6 = 0x06,
    Level7 = 0x07,
    Level8 = 0x08,
    Level9 = 0x09,
    Level10 = 0x0a,
    Level11 = 0x0b,
    Level12 = 0x0c,
    Level13 = 0x0d,
    Level14 = 0x0e,
    Level15 = 0x0f,
    Level16 = 0x10,
    Level17 = 0x11,
    Level18 = 0x12,
    Level19 = 0x13,
    Level20 = 0x14,
}

#[derive(Debug, Clone, Copy, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum WindCode {
    NoWind = 0x02,
    Wind = 0x03,
    WindVoice = 0x05,
}

#[derive(Debug, Clone, Copy)]
pub struct AncWF1000XM4 {
    pub command: MessageCode,
    pub continuous: bool,
    pub anc_enable: bool,
    pub anc_mode: AncMode,
    pub nc_wind: WindCode,
    pub as_voice: bool,
    pub as_level: AsLevel,
}

impl DeviceCommand for AncWF1000XM4 {}

impl TryInto<Anc> for AncWF1000XM4 {
    type Error = Error;

    fn try_into(self) -> Result<Anc, Self::Error> {
        match self.anc_enable {
            true => match self.anc_mode {
                AncMode::NoiseCanceling => Ok(Anc::NoiseCanceling {
                    wind: if self.nc_wind == WindCode::Wind || self.nc_wind == WindCode::WindVoice {
                        true
                    } else {
                        false
                    },
                }),
                AncMode::AmbientSound => Ok(Anc::AmbientSound {
                    level: self.as_level as u8,
                    voice: self.as_voice,
                }),
            },
            false => Ok(Anc::Off),
        }
    }
}

impl TryFrom<Anc> for AncWF1000XM4 {
    type Error = Error;

    fn try_from(value: Anc) -> Result<Self, Self::Error> {
        match value {
            Anc::Off => Ok(AncWF1000XM4 {
                command: MessageCode::AncSet,
                continuous: false,
                anc_enable: false,
                anc_mode: AncMode::NoiseCanceling,
                nc_wind: WindCode::NoWind,
                as_voice: false,
                as_level: AsLevel::Level1,
            }),
            Anc::NoiseCanceling { wind } => Ok(AncWF1000XM4 {
                command: MessageCode::AncSet,
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
            }),
            Anc::AmbientSound { level, voice } => Ok(AncWF1000XM4 {
                command: MessageCode::AncSet,
                continuous: false,
                anc_enable: true,
                anc_mode: AncMode::AmbientSound,
                nc_wind: WindCode::NoWind,
                as_voice: voice,
                as_level: AsLevel::try_from(level)
                    .map_err(|x| Error::new(format!("Invalid ANC level: {:?} ({})", x, level)))?,
            }),
        }
    }
}

impl TryFrom<Vec<u8>> for AncWF1000XM4 {
    type Error = Error;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.len() != 8 {
            return Err(Error::new(format!(
                "Invalid ANC command length: {}",
                value.len()
            )));
        }

        Ok(AncWF1000XM4 {
            command: MessageCode::try_from(u16::from_be_bytes([value[0], value[1]]))
                .map_err(|x| Error::new(format!("Invalid command type: {:?} ({:?})", x, value)))?,
            continuous: !value[2] == 0x00,
            anc_enable: value[3] == 0x01,
            anc_mode: AncMode::try_from(value[4])
                .map_err(|x| Error::new(format!("Invalid ANC mode: {:?} ({})", x, value[3])))?,
            nc_wind: WindCode::try_from(value[5])
                .map_err(|x| Error::new(format!("Invalid NC wind code: {:?} ({})", x, value[4])))?,
            as_voice: value[6] == 0x01,
            as_level: AsLevel::try_from(value[7])
                .map_err(|x| Error::new(format!("Invalid AS level: {:?} ({})", x, value[6])))?,
        })
    }
}

impl TryInto<SonyCommand> for AncWF1000XM4 {
    type Error = Error;

    fn try_into(self) -> Result<SonyCommand, Self::Error> {
        let mut bytes = vec![];

        bytes.extend_from_slice(&(self.command as u16).to_be_bytes());
        bytes.push(!self.continuous as u8);
        bytes.push(self.anc_enable as u8);
        bytes.push(self.anc_mode as u8);
        bytes.push(self.nc_wind as u8);
        bytes.push(self.as_voice as u8);
        bytes.push(self.as_level as u8);

        Ok(SonyCommand {
            data_type: DataType::DataMdr,
            seq_number: 0,
            // Recalculate at conversion time
            payload_size: bytes.len() as u8,
            payload: bytes,
            // Recalculate at conversion time
            checksum: 0,
        })
    }
}
