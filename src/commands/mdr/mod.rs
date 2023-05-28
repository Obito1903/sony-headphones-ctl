use crate::common;

use self::ambient_sound::AmbientSound;

pub mod ambient_sound;

#[derive(Debug, Clone)]
pub enum Mdr {
    Asm(AmbientSound),
}

impl TryFrom<Vec<u8>> for Mdr {
    type Error = common::Error;

    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        if bytes.len() < 2 {
            return Err(common::Error::new(format!(
                "Invalid Mdr Command: {:?}",
                bytes
            )));
        }

        let command_type = u16::from_be_bytes(bytes[0..2].try_into().unwrap());

        let command = match command_type {
            0x6815 | 0x6915 => Ok(Mdr::Asm(bytes.try_into()?)),
            _ => Err(common::Error::new(format!(
                "Invalid Mdr Command type: {:?}",
                command_type
            ))),
        }?;

        Ok(command)
    }
}

impl TryInto<Vec<u8>> for Mdr {
    type Error = common::Error;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        match self {
            Mdr::Asm(asm_command) => asm_command.try_into(),
        }
    }
}
