use std::{fmt, io};

use tokio_util::{
    bytes::{Buf, Bytes},
    codec::Decoder,
};

pub struct SerialRTDCodec {
    state: SerialRTDCodecState,
}

impl SerialRTDCodec {
    fn new() -> Self {
        Self {
            state: SerialRTDCodecState::ReadingSyncIdle,
        }
    }
}

impl Default for SerialRTDCodec {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
enum SerialRTDCodecState {
    ReadingSyncIdle,
    ReadingData { next_index: usize },
}

impl Decoder for SerialRTDCodec {
    type Item = Bytes;
    type Error = SerialRTDCodecError;

    fn decode(
        &mut self,
        buf: &mut tokio_util::bytes::BytesMut,
    ) -> Result<Option<Self::Item>, Self::Error> {
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
                        Ok(Some(data.into()))
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

#[derive(Debug)]
pub enum SerialRTDCodecError {
    /// An IO error occurred.
    Io(io::Error),
}

impl fmt::Display for SerialRTDCodecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SerialRTDCodecError::Io(e) => write!(f, "{}", e),
        }
    }
}

impl From<io::Error> for SerialRTDCodecError {
    fn from(e: io::Error) -> SerialRTDCodecError {
        SerialRTDCodecError::Io(e)
    }
}

impl std::error::Error for SerialRTDCodecError {}
