use crate::common;

use super::{Mdr2, Mdr2Types};

#[derive(Debug, Clone)]
pub struct Mac {
    pub bytes: [u8; 6],
}

impl ToString for Mac {
    fn to_string(&self) -> String {
        format!(
            "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
            self.bytes[0],
            self.bytes[1],
            self.bytes[2],
            self.bytes[3],
            self.bytes[4],
            self.bytes[5]
        )
    }
}

#[derive(Debug, Clone)]
pub struct Device {
    mac: Mac,
    name: String,
}

#[derive(Debug, Clone)]
pub struct RegisteredDevices {
    pub devices: Vec<Device>,
}

impl TryFrom<Vec<u8>> for RegisteredDevices {
    type Error = common::Error;

    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        let mut devices = vec![];

        // Skip the first 3 bytes, they are the command type and the length of a mac address
        let mut i = 3;
        while i < bytes.len() - 1 {
            let mut mac = [0; 6];
            for mac_byte in 0..6 {
                let chars: Vec<u8> = bytes[i..i + 2].try_into().unwrap();
                println!("{:?}", chars);
                mac[mac_byte] = hex::decode(chars).unwrap()[0];
                i += 3;
            }
            // Skip the 0x00 byte
            // i += 1;
            let name_len = bytes[i] as usize;
            i += 1;
            let name = String::from_utf8(bytes[i..i + name_len].to_vec()).unwrap();
            let device = Device {
                mac: Mac { bytes: mac },
                name,
            };
            println!("{:?}", device);
            devices.push(device);
            i += name_len;
        }
        Ok(RegisteredDevices { devices })
    }
}

impl TryInto<Vec<u8>> for RegisteredDevices {
    type Error = common::Error;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&(Mdr2Types::RegisteredDevicesResp as u16).to_be_bytes());
        for device in self.devices {
            bytes.extend_from_slice(&device.mac.to_string().as_bytes());
            bytes.push(0x00);
            bytes.push(device.name.len() as u8);
            bytes.extend_from_slice(device.name.as_bytes());
        }
        Ok(bytes)
    }
}
