pub mod mdr;
pub mod mdr2;

use std::fmt::Debug;

use crate::common;

use self::{mdr::Mdr, mdr2::Mdr2};

#[derive(Debug, Clone)]
pub struct Command {
    pub seq_number: u32,
    pub data_type: DataType,
}

impl Command {
    pub fn set_seq_number(&mut self, seq_number: u32) {
        self.seq_number = seq_number;
    }
}

impl TryFrom<&[u8]> for Command {
    type Error = common::Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < 6 {
            return Err(common::Error::new(format!(
                "Invalid Sony Container: {:?}",
                bytes
            )));
        }

        let data_type = bytes[1];
        let seq_number = u32::from_le_bytes(bytes[2..6].try_into().unwrap());
        let payload_size = u8::from_be(bytes[6]) as usize;
        let payload = bytes[7..7 + payload_size].to_vec();
        let checksum = bytes[7 + payload_size];

        let mut sum = data_type + seq_number as u8 + payload_size as u8;
        for b in payload.iter() {
            sum = sum.wrapping_add(*b);
        }

        if sum != checksum {
            return Err(common::Error::new(format!(
                "Invalid Checksum: {:?} != {:?} for {:?} in {:?}",
                sum, checksum, payload, bytes
            )));
        }

        let data = DataType::try_from(bytes[1..(bytes.len() - 1)].to_vec())?;

        Ok(Command {
            seq_number,
            data_type: data,
        })
    }
}

impl TryInto<Vec<u8>> for Command {
    type Error = common::Error;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        let data_code = self.data_type.code();

        let mut payload: Vec<u8> = self.data_type.try_into()?;
        let mut bytes = vec![];

        bytes.push('>' as u8);
        bytes.push(data_code);
        bytes.extend_from_slice(&self.seq_number.to_le_bytes());
        bytes.push(payload.len() as u8);
        bytes.append(&mut payload);

        let checksum = bytes[1..]
            .iter()
            .fold(0, |acc: u8, x: &u8| acc.wrapping_add(*x));

        bytes.push(checksum);
        bytes.push('<' as u8);
        Ok(bytes)
    }
}

// #[derive(Debug, Clone, Copy, PartialEq, TryFromPrimitive)]
// #[repr(u8)]
// enum DataType {
//     Data = 0x00,
//     Ack = 0x01,
//     DataMc1 = 0x02,
//     DataIcd = 0x09,
//     DataEv = 0x0a,
//     DataMdr = 0x0c,
//     DataCommon = 0xd,
//     DataMdr2 = 0x0e,
//     Shot = 0x10,
//     ShotMc1 = 0x12,
//     ShotIcd = 0x19,
//     ShotEv = 0x1a,
//     ShotMdr = 0x1c,
//     ShotCommon = 0x1d,
//     ShotMdr2 = 0x1e,
//     LargeData = 0x2d,
// }
#[derive(Debug, Clone)]
pub enum DataType {
    Data(Vec<u8>),
    Ack(),
    DataMdr(Mdr),
    DataMdr2(Mdr2),
}

impl DataType {
    pub fn code(&self) -> u8 {
        match self {
            DataType::Ack() => 0x01,
            DataType::Data(_) => 0x00,
            DataType::DataMdr(_) => 0x0c,
            DataType::DataMdr2(_) => 0x0e,
        }
    }
}

impl TryFrom<Vec<u8>> for DataType {
    type Error = common::Error;

    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        if bytes.len() < 2 {
            return Err(common::Error::new(format!("Invalid Data: {:?}", bytes)));
        }

        let data_type = bytes[0];
        let payload_size = u8::from_be(bytes[5]) as usize;
        let payload = bytes[6..6 + payload_size].to_vec();

        let data = match data_type {
            0x01 => Ok(DataType::Ack()),
            0x0c => Ok(DataType::DataMdr(Mdr::try_from(payload).map_err(
                |e: common::Error| common::Error::new(format!("Invalid Mdr: {:?}", e.message)),
            )?)),
            0x0e => Ok(DataType::DataMdr2(Mdr2::try_from(payload).map_err(
                |e: common::Error| common::Error::new(format!("Invalid Mdr2: {:?}", e.message)),
            )?)),
            _ => Ok(DataType::Data(payload)),
        }?;

        Ok(data)
    }
}

impl TryInto<Vec<u8>> for DataType {
    type Error = common::Error;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        match self {
            DataType::Data(bytes) => Ok(bytes),
            DataType::Ack() => Ok(vec![]),
            DataType::DataMdr(mdr) => mdr.try_into(),
            DataType::DataMdr2(mdr2) => mdr2.try_into(),
        }
    }
}
