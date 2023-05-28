use derive_try_from_primitive::TryFromPrimitive;

use crate::common;

use super::Mdr;

#[derive(Debug, Clone, Copy, PartialEq, TryFromPrimitive)]
#[repr(u16)]
pub enum AsmCommandType {
    Set = 0x6815,
    Ack = 0x6915,
    // Changed = 0x15,
}

#[derive(Debug, Clone, Copy, TryFromPrimitive)]
#[repr(u8)]
pub enum AscMode {
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

#[derive(Debug, Clone, Copy)]
pub struct AmbientSound {
    pub command_mode: AsmCommandType,
    pub continuous: bool,
    pub asc_enable: bool,
    pub asc_mode: AscMode,
    pub asc_wind: bool,
    pub voice_pass_through: bool,
    pub as_level: AsLevel,
}

impl TryInto<Vec<u8>> for AmbientSound {
    type Error = common::Error;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        let mut bytes = vec![];
        let wind_byte: u8 = if self.asc_wind && self.voice_pass_through {
            0x05
        } else if self.asc_wind {
            0x03
        } else {
            0x02
        };

        bytes.extend_from_slice(&(self.command_mode as u16).to_be_bytes());
        bytes.push(!self.continuous as u8);
        bytes.push(self.asc_enable as u8);
        bytes.push(self.asc_mode as u8);
        bytes.push(wind_byte);
        bytes.push(self.voice_pass_through as u8);
        bytes.push(self.as_level as u8);
        Ok(bytes)
    }
}

impl TryFrom<Vec<u8>> for AmbientSound {
    type Error = common::Error;

    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        if bytes.len() != 8 {
            return Err(common::Error::new(format!(
                "Invalid length for AsmCommand: {}",
                bytes.len()
            )));
        }
        let command_mode =
            AsmCommandType::try_from(u16::from_be_bytes(bytes[0..2].try_into().unwrap()))
                .map_err(|x| common::Error::new(format!("Invalide AsmCommandType {}", x)))?;
        let continuous = bytes[2] == 0;
        let asc_enable = bytes[3] != 0;
        let asc_mode = AscMode::try_from(bytes[4])
            .map_err(|x| common::Error::new(format!("Invalide AscMode {}", x)))?;
        let asc_wind = bytes[5] != 0x02;
        let voice_pass_through = bytes[6] != 0;
        let as_level = AsLevel::try_from(bytes[7])
            .map_err(|x| common::Error::new(format!("Invalide AsLevel {}", x)))?;
        Ok(AmbientSound {
            command_mode,
            continuous,
            asc_enable,
            asc_mode,
            asc_wind,
            voice_pass_through,
            as_level,
        })
    }
}
