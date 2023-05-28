use derive_try_from_primitive::TryFromPrimitive;

use crate::common;

use self::registered_devices::RegisteredDevices;

pub mod registered_devices;

#[derive(Debug, Clone, TryFromPrimitive)]
#[repr(u16)]
pub enum Mdr2Types {
    RegisteredDevicesAsk,
    RegisteredDevicesResp,
}

#[derive(Debug, Clone)]
pub enum Mdr2 {
    RegisteredDevices(RegisteredDevices),
}

impl TryFrom<Vec<u8>> for Mdr2 {
    type Error = common::Error;

    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        if bytes.len() < 2 {
            return Err(common::Error::new(format!(
                "Invalid Mdr2 Command: {:?}",
                bytes
            )));
        }

        let command_type = Mdr2Types::try_from(u16::from_be_bytes(bytes[0..2].try_into().unwrap()))
            .map_err(|x| common::Error::new(format!("Invalid Mdr2 Command type: {}", x)))?;

        let command = match command_type {
            Mdr2Types::RegisteredDevicesAsk | Mdr2Types::RegisteredDevicesResp => {
                Ok(Mdr2::RegisteredDevices(bytes.try_into()?))
            }
        }?;

        Ok(command)
    }
}

impl TryInto<Vec<u8>> for Mdr2 {
    type Error = common::Error;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        match self {
            Mdr2::RegisteredDevices(registered_devices) => registered_devices.try_into(),
        }
    }
}
