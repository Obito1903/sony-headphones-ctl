use derive_try_from_primitive::TryFromPrimitive;

use crate::{devices::DeviceCommand, DataType, Error, SonyCommand};

use super::CommandTypes;

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

#[derive(Debug, Clone, Copy, TryFromPrimitive)]
#[repr(u8)]
pub enum WindCode {
    NoWind = 0x02,
    Wind = 0x03,
    WindVoice = 0x05,
}

#[derive(Debug, Clone, Copy)]
pub struct AncCommand {
    pub command: CommandTypes,
    pub continuous: bool,
    pub anc_enable: bool,
    pub anc_mode: AncMode,
    pub nc_wind: WindCode,
    pub as_voice: bool,
    pub as_level: AsLevel,
}

impl DeviceCommand for AncCommand {}

impl TryInto<SonyCommand> for AncCommand {
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
