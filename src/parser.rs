use std::num::ParseIntError;

#[derive(Debug, Clone)]
pub struct Packet<'a> {
    pub data: &'a str,
    pub start_index: u32,
}

const HEADER_PREFIX: &[u8] = b"004210";

impl<'a> TryFrom<&'a [u8]> for Packet<'a> {
    type Error = PacketParseError;

    /// Parse the packet from some data.
    /// Note that I have no idea what hash function is used, so the hash isn't
    /// checked.
    fn try_from(value: &'a [u8]) -> Result<Packet<'a>, PacketParseError> {
        let mut parts = value.split(|b| *b == 0x01 || *b == 0x02 || *b == 0x04);

        // extract the first part (not sure what 00000000 is, so forgetting)
        let _ = parts.next().ok_or(PacketParseError::IllFormed)?;

        // extract the data position
        let header = parts.next().ok_or(PacketParseError::IllFormed)?;
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
        let data = std::str::from_utf8(parts.next().ok_or(PacketParseError::IllFormed)?)
            .map_err(|_| PacketParseError::BadTextEncoding)?;

        // extract the checksum (don't know the hash algorithm, so skipping)
        let _checksum = parts.next().ok_or(PacketParseError::IllFormed)?;

        // if there's any data left, something's fishy
        if parts.next() != None {
            Err(PacketParseError::IllFormed)
        } else {
            Ok(Packet { data, start_index })
        }
    }
}

#[derive(Debug)]
pub enum PacketParseError {
    UnsupportedPacket,
    IllFormed,
    BadTextEncoding,
    NumberParseFailure(ParseIntError),
}
