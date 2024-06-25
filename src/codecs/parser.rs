use std::{fmt, num::ParseIntError};

use bytes::Bytes;

#[derive(Debug, Clone)]
pub struct Packet {
    data: Bytes,
    pub start_index: u32,
}

impl<'a> Packet {
    pub fn data(&'a self) -> Option<&'a str> {
        std::str::from_utf8(&self.data).ok()
    }

    pub fn raw_data(&self) -> Bytes {
        self.data.clone()
    }
}

const HEADER_PREFIX: &[u8] = b"004210";

impl TryFrom<&[u8]> for Packet {
    type Error = PacketParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Self::try_from(Bytes::copy_from_slice(value))
    }
}

impl TryFrom<Bytes> for Packet {
    type Error = PacketParseError;

    /// Parse the packet from some data.
    /// Note that I have no idea what hash function is used, so the hash isn't
    /// checked.
    fn try_from(mut value: Bytes) -> Result<Packet, PacketParseError> {
        // extract the first part (not sure what 00000000 is, so forgetting)
        let _ = value.split_to(
            value
                .iter()
                .position(|b| *b == 0x01)
                .ok_or(PacketParseError::IllFormed)?,
        );

        // extract the data position
        let header = value.split_to(
            value
                .iter()
                .position(|b| *b == 0x02)
                .ok_or(PacketParseError::IllFormed)?,
        );
        let start_index: u32 = std::str::from_utf8(
            header
                // take off the packet prefix
                .strip_prefix(HEADER_PREFIX)
                // if the prefix doesn't exist, we're handling an unsupported packet type
                .ok_or(PacketParseError::UnsupportedPacket)?,
        )
        .map_err(|_| PacketParseError::BadTextEncoding)?
        .parse()
        .map_err(|err| PacketParseError::NumberParseFailure(err))?;

        // extract the data
        let data = value.split_to(
            value
                .iter()
                .position(|b| *b == 0x04)
                .ok_or(PacketParseError::IllFormed)?,
        );

        // extract the checksum (don't know the hash algorithm, so skipping)
        let _ = value;

        Ok(Packet { data, start_index })
    }
}

#[derive(Debug)]
pub enum PacketParseError {
    UnsupportedPacket,
    IllFormed,
    BadTextEncoding,
    NumberParseFailure(ParseIntError),
}

impl fmt::Display for PacketParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PacketParseError::UnsupportedPacket => write!(f, "unsupported packet type"),
            PacketParseError::IllFormed => write!(f, "packet is ill-formed"),
            PacketParseError::BadTextEncoding => write!(f, "bad text encoding in packet"),
            PacketParseError::NumberParseFailure(e) => write!(f, "couldn't parse number: {}", e),
        }
    }
}

impl std::error::Error for PacketParseError {}
