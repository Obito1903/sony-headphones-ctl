#![feature(async_fn_in_trait)]

pub mod devices;

use std::fmt::Debug;

use derive_try_from_primitive::TryFromPrimitive;

#[derive(Debug)]
pub struct Error {
    pub message: String,
}

impl Error {
    pub fn new(message: String) -> Self {
        Error { message }
    }
}

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

#[derive(Debug, Clone, Copy, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum DataType {
    Data = 0x00,
    Ack = 0x01,
    DataMc1 = 0x02,
    DataIcd = 0x09,
    DataEv = 0x0a,
    DataMdr = 0x0c,
    DataCommon = 0xd,
    DataMdr2 = 0x0e,
    Shot = 0x10,
    ShotMc1 = 0x12,
    ShotIcd = 0x19,
    ShotEv = 0x1a,
    ShotMdr = 0x1c,
    ShotCommon = 0x1d,
    ShotMdr2 = 0x1e,
    LargeData = 0x2d,
}

#[derive(Debug, Clone)]
pub struct SonyCommand {
    pub data_type: DataType,
    pub seq_number: u32,
    pub payload_size: u8,
    pub payload: Vec<u8>,
    pub checksum: u8,
}

impl SonyCommand {
    pub fn set_seq_number(&mut self, seq_number: u32) {
        self.seq_number = seq_number;
    }
}

impl TryFrom<&[u8]> for SonyCommand {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < 6 {
            return Err(Error::new(format!("Invalid Sony Container: {:?}", bytes)));
        }

        let data_type = bytes[1]
            .try_into()
            .map_err(|e: u8| Error::new(format!("Invalid Data Type: {:?}", e)))?;
        let seq_number = u32::from_le_bytes(bytes[2..6].try_into().unwrap());
        let payload_size = u8::from_be(bytes[6]) as usize;
        let payload = bytes[7..7 + payload_size].to_vec();
        let checksum = bytes[7 + payload_size];

        let mut sum: u8 = data_type as u8 + seq_number as u8 + payload_size as u8;
        for b in payload.iter() {
            sum = sum.wrapping_add(*b);
        }

        if sum != checksum {
            return Err(Error::new(format!(
                "Invalid Checksum: {:?} != {:?} for {:?} in {:?}",
                sum, checksum, payload, bytes
            )));
        }

        Ok(SonyCommand {
            seq_number,
            data_type: data_type,
            payload_size: payload_size as u8,
            payload,
            checksum,
        })
    }
}

impl TryInto<Vec<u8>> for SonyCommand {
    type Error = Error;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        let mut bytes = vec![];

        bytes.push('>' as u8);
        bytes.push(self.data_type as u8);
        bytes.extend_from_slice(&self.seq_number.to_le_bytes());
        bytes.push(self.payload.len() as u8);
        bytes.append(&mut self.payload.clone());

        let checksum = bytes[1..]
            .iter()
            .fold(0, |acc: u8, x: &u8| acc.wrapping_add(*x));
        bytes.push(checksum);
        bytes.push('<' as u8);
        Ok(bytes)
    }
}
