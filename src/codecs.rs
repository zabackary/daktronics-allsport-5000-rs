//! Contains a `tokio_util` codec to parse packets sent from the serial line.

use super::packet::{Packet, PacketParseError};
use bytes::{Buf, Bytes, BytesMut};

use std::{fmt, io};

use tokio_util::codec::Decoder;

/// The codec, implementing [`Decoder`]
///
/// The codec returns a stream of [`Packet`]s containing an offset and data
/// contents. For more information, see the `packet` module docs.
///
/// # Examples
///
/// ```no_run
/// use daktronics_allsport_5000::codecs;
/// use tokio_serial::SerialPortBuilderExt; // for open_native_async
/// use tokio_util::codec::Decoder; // for framed
/// use futures_util::StreamExt; // for next
///
/// let serial_stream = tokio_serial::new("/dev/ttyUSB0", 19200)
///     .parity(tokio_serial::Parity::None)
///     .open_native_async()
///     .unwrap();
/// let mut reader = codecs::SerialRTDCodec::default().framed(serial_stream);
/// reader.next(); // returns a future
/// ```
#[derive(Debug)]
pub struct SerialRTDCodec {
    state: SerialRTDCodecState,
}

impl SerialRTDCodec {
    /// Create a new instance
    ///
    /// This assumes that the transmission stream is in an idle state.
    fn new() -> Self {
        Self {
            state: SerialRTDCodecState::ReadingSyncIdle,
        }
    }
}

impl Default for SerialRTDCodec {
    /// Create a new instance.
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
enum SerialRTDCodecState {
    /// The SYNC IDLE transmission is ongoing
    ReadingSyncIdle,
    /// Data is being read; last checked index for the end byte is stored in
    /// `next_index`
    ReadingData { next_index: usize },
}

impl Decoder for SerialRTDCodec {
    type Item = Packet;
    type Error = SerialRTDCodecError;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        match &mut self.state {
            SerialRTDCodecState::ReadingSyncIdle => {
                // Look for the SYNC IDLE end byte
                match buf.iter().position(|b| *b == 0x16) {
                    Some(position) => {
                        // We found the byte, advance the cursor past it
                        buf.advance(position + 1);
                        // Prepare to read the data next `decode` pass
                        self.state = SerialRTDCodecState::ReadingData { next_index: 0 }
                    }
                    // Skip more
                    None => buf.advance(buf.len()),
                }
                Ok(None)
            }
            SerialRTDCodecState::ReadingData { next_index } => {
                // Look for the body end byte
                match buf[*next_index..].iter().position(|b| *b == 0x17) {
                    Some(position_offset) => {
                        let position = *next_index + position_offset;
                        // We found the byte, terminate the frame
                        let mut data = buf.split_to(position + 1);
                        // Cut off the termination byte
                        data.truncate(data.len() - 1);
                        // Prepare to read the next frame next `decode` pass
                        self.state = SerialRTDCodecState::ReadingSyncIdle;
                        // Return the frame
                        Ok(Some(
                            Bytes::from(data)
                                .try_into()
                                .map_err(SerialRTDCodecError::PacketParseError)?,
                        ))
                    }
                    // We haven't found the byte yet, so resume the search next
                    // time
                    None => {
                        *next_index = buf.len();
                        Ok(None)
                    }
                }
            }
        }
    }
}

/// An error occurring while fetching the next packet
#[derive(Debug)]
pub enum SerialRTDCodecError {
    /// Couldn't parse a packet.
    PacketParseError(PacketParseError),
    /// An IO error occurred.
    Io(io::Error),
}

impl fmt::Display for SerialRTDCodecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SerialRTDCodecError::Io(e) => write!(f, "io error while reading serial: {}", e),
            SerialRTDCodecError::PacketParseError(e) => write!(f, "packet parse error: {}", e),
        }
    }
}

impl From<io::Error> for SerialRTDCodecError {
    fn from(e: io::Error) -> SerialRTDCodecError {
        SerialRTDCodecError::Io(e)
    }
}

impl std::error::Error for SerialRTDCodecError {}
