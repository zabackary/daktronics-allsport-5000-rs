//! Contains a struct representing one packet sent from the control console.
//!
//! See [`Packet`] for more details.
use std::{fmt, num::ParseIntError, str::Utf8Error};

use bytes::{Buf, Bytes};

/// Represents a packet sent from the control console
///
/// Each packet contains two pieces of data: the data being sent, and the index
/// in the data buffer that the data belongs at. Thus, the entire buffer of data
/// the console holds in memory is not sent every transmission, but rather,
/// incrementally, whenever a piece of data changes.
///
/// Tip: the console will not dump the initial state when your program initially
/// reads from the serial. You can press the `STOP` button for this to happen.
#[derive(Debug, Clone)]
pub struct Packet {
    data: Bytes,
    start_index: u32,
}

impl<'a> Packet {
    /// Creates a new packet from the given data and start index
    ///
    /// `data` is the data sent from the control console, and `start_index` is
    /// the index in the data buffer that the data belongs at.
    ///
    /// This function is mostly used for testing purposes, as you should be
    /// using the `TryFrom` implementations to parse packets from bytes.
    pub fn new(data: Bytes, start_index: u32) -> Self {
        Packet { data, start_index }
    }

    /// Gets the decoded data sent from the control console
    ///
    /// This data is actually a string, so this function does the parsing for
    /// you and returns [`Some`] if the UTF-8 conversion succeeds and [`None`]
    /// if it fails. If this behavior is unwanted, you can get the raw bytes by
    /// calling [`Packet::raw_data`] instead.
    pub fn data(&'a self) -> Option<&'a str> {
        std::str::from_utf8(&self.data).ok()
    }

    /// Gets the starting index in the data buffer this packet's data is at
    ///
    /// This index is zero-based, unlike the documentation for the console's
    /// fields, which is one-based (as far as I can tell).
    pub fn start_index(&'a self) -> u32 {
        self.start_index
    }

    /// Gets the raw data this packet represents
    ///
    /// Prefer using [`Packet::data`] instead, as you usually want the actual
    /// data and not the raw buffer.
    pub fn raw_data(&self) -> Bytes {
        self.data.clone()
    }
}

/// The prefix at the beginning of all data packets.
///
/// I can't really work out the other packets, so the below functions filter out
/// any packets that don't match this signature.
const HEADER_PREFIX: &[u8] = b"004210";

impl TryFrom<&[u8]> for Packet {
    type Error = PacketParseError;

    /// Tries to convert from a bytes slice
    ///
    /// If you're using [`bytes::Bytes`], it should be preferred to use the
    /// `TryFrom<Bytes>` implementation to avoid an allocation.
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Self::try_from(Bytes::copy_from_slice(value))
    }
}

impl TryFrom<Bytes> for Packet {
    type Error = PacketParseError;

    /// Parse the packet from some data.
    ///
    /// Note that I have no idea what hash function is used (it's probably a
    /// simple one I can't figure out), so the hash isn't checked.
    fn try_from(mut value: Bytes) -> Result<Packet, PacketParseError> {
        // extract the first part (not sure what 00000000 is, so forgetting)
        let _ = value.split_to(
            value
                .iter()
                .position(|b| *b == 0x01)
                .ok_or(PacketParseError::IllFormed)?,
        );
        value.advance(1); // skip the separator byte

        // extract the data position
        let header = value.split_to(
            value
                .iter()
                .position(|b| *b == 0x02)
                // some packets don't have data, so extract until the checksum
                .or_else(|| value.iter().position(|b| *b == 0x04))
                .ok_or(PacketParseError::IllFormed)?,
        );
        value.advance(1); // skip the separator byte
        let start_index: u32 = std::str::from_utf8(
            header
                // take off the packet prefix
                .strip_prefix(HEADER_PREFIX)
                // if the prefix doesn't exist, we're handling an unsupported packet type
                .ok_or_else(|| PacketParseError::UnsupportedPacket {
                    header_bytes: header.clone(),
                })?,
        )
        .map_err(PacketParseError::BadTextEncoding)?
        .parse()
        .map_err(PacketParseError::NumberParseFailure)?;

        // extract the data
        let data = value.split_to(
            value
                .iter()
                .position(|b| *b == 0x04)
                .ok_or(PacketParseError::IllFormed)?,
        );
        value.advance(1); // skip the separator byte

        // extract the checksum (don't know the hash algorithm, so skipping)
        let _ = value;

        Ok(Packet { data, start_index })
    }
}

/// An error occurring during the packet parsing stage
#[derive(Debug)]
pub enum PacketParseError {
    /// The packet's type is unsupported
    ///
    /// The header is provided for convenience.
    UnsupportedPacket { header_bytes: Bytes },
    /// The packet is ill-formed, i.e., it appears to be half-finished
    IllFormed,
    /// The bytes representing the start_index of the `Packet` are malformed
    ///
    /// Those bytes are actually encoded as ASCII for some reason, so there is a
    /// chance that there is a decoding error (however small).
    BadTextEncoding(Utf8Error),
    /// The bytes representing the start_index of the `Packet` aren't a number
    ///
    /// Those bytes are encoded in ASCII for some reason, so it's possible that
    /// we couldn't parse the int.
    NumberParseFailure(ParseIntError),
}

impl fmt::Display for PacketParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PacketParseError::UnsupportedPacket { header_bytes: _ } => {
                write!(f, "unsupported packet type")
            }
            PacketParseError::IllFormed => write!(f, "packet is ill-formed"),
            PacketParseError::BadTextEncoding(err) => {
                write!(f, "bad text encoding in packet: {}", err)
            }
            PacketParseError::NumberParseFailure(err) => {
                write!(f, "couldn't parse number: {}", err)
            }
        }
    }
}

impl std::error::Error for PacketParseError {}
