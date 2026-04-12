//! Contains a struct representing one packet sent from the control console.
//!
//! See [`Packet`] for more details.
use std::{num::ParseIntError, str::Utf8Error};

use bytes::{Buf, Bytes};
use snafu::{OptionExt, ResultExt as _, Snafu};

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
        // look ahead into the packet to calculate the expected checksum before we begin
        // the checksum algorithm simply sums up all the bytes in the packet, wrapping
        // around on overflow. the sum is then converted to a 2-digit uppercase hex str
        let expected_checksum_value = value.iter().take_while(|&&byte| byte != 0x04).fold(
            // the checksum is calculated over the data **and** the separator byte,
            // so we start with the value of the separator byte (0x04) to account for that
            0x04u8,
            |sum, byte| sum.wrapping_add(*byte),
        );
        let expected_checksum = format!("{:02X}", expected_checksum_value);

        // extract the first part (not sure what 00000000 is, so forgetting)
        let _ = value.split_to(
            value
                .iter()
                .position(|b| *b == 0x01)
                .context(IllFormedSnafu {
                    reason: "missing header",
                })?,
        );
        value.advance(1); // skip the separator byte

        // extract the data position
        let header = value.split_to(
            value
                .iter()
                .position(|b| *b == 0x02)
                // some packets don't have data, so extract until the checksum
                .or_else(|| value.iter().position(|b| *b == 0x04))
                .context(IllFormedSnafu {
                    reason: "missing checksum without data",
                })?,
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
        .context(BadTextEncodingSnafu)?
        .parse()
        .context(NumberParseFailureSnafu)?;

        // extract the data
        let data = value.split_to(value.iter().position(|b| *b == 0x04).context(
            IllFormedSnafu {
                reason: "missing checksum",
            },
        )?);
        value.advance(1); // skip the separator byte

        // extract the checksum (don't know the hash algorithm, so skipping)
        let checksum = value;
        if checksum != expected_checksum.as_bytes() {
            return Err(PacketParseError::IllFormed {
                reason: "checksum mismatch",
            });
        }

        Ok(Packet { data, start_index })
    }
}

/// An error occurring during the packet parsing stage
#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum PacketParseError {
    /// The packet's type is unsupported
    ///
    /// The header is provided for convenience.
    #[snafu(display("unsupported packet type with header bytes: {:?}", header_bytes))]
    UnsupportedPacket { header_bytes: Bytes },
    /// The packet is ill-formed in some way
    #[snafu(display("packet is ill-formed: {}", reason))]
    IllFormed { reason: &'static str },
    /// The bytes representing the start_index of the `Packet` are malformed
    ///
    /// Those bytes are actually encoded as ASCII for some reason, so there is a
    /// chance that there is a decoding error (however small).
    #[snafu(display("bad text encoding in packet: {}", source))]
    BadTextEncoding { source: Utf8Error },
    /// The bytes representing the start_index of the `Packet` aren't a number
    ///
    /// Those bytes are encoded in ASCII for some reason, so it's possible that
    /// we couldn't parse the int.
    #[snafu(display("couldn't parse number: {}", source))]
    NumberParseFailure { source: ParseIntError },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packet_basic() {
        const PACKET: &[u8] = b"00000000\x010042100000\x0216:0916:09   16:0916:09    s   \x0449";

        let packet = Packet::try_from(Bytes::from_static(PACKET)).unwrap();
        assert_eq!(packet.start_index, 0);
        assert_eq!(packet.data().unwrap(), "16:0916:09   16:0916:09    s   ");
    }

    #[test]
    fn test_packet_offset() {
        const PACKET: &[u8] = b"00000000\x010042100006\x0216:0916:09   16:0916:09    s   \x044F";

        let packet = Packet::try_from(Bytes::from_static(PACKET)).unwrap();
        assert_eq!(packet.start_index, 6);
        assert_eq!(packet.data().unwrap(), "16:0916:09   16:0916:09    s   ");
    }

    #[test]
    #[should_panic]
    fn test_packet_malformed() {
        const PACKET: &[u8] = b"00000000\x010042100006\x0216:0916:09   asfkjkj09    s   \x0449";

        let _ = Packet::try_from(Bytes::from_static(PACKET)).expect("should panic");
    }

    #[test]
    #[should_panic]
    fn test_packet_unknown_header() {
        const PACKET: &[u8] = b"00000000\x010032100006\x0216:0916:09   asfkjkj09    s   \x0449";

        let _ = Packet::try_from(Bytes::from_static(PACKET)).expect("should panic");
    }
}
