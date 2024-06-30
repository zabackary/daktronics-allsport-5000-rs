pub mod data_source;

use std::{num::ParseIntError, str::Utf8Error};

use bytes::BytesMut;
use data_source::RTDStateDataSource;

use crate::packet::Packet;

pub struct RTDState<DS: RTDStateDataSource> {
    data_source: DS,
    data: BytesMut,
}

impl<DS: data_source::RTDStateDataSource> RTDState<DS> {
    /// Create a new [`RTDState`], with the provided [`RTDStateDataSource`]
    /// implementation.
    ///
    /// Prefer using [`RTDState::from_serial_stream`] if you're using the
    /// `tokio-serial` feature (on by default).
    pub fn new(data_source: DS) -> Self {
        // as far as I can tell from the docs, the largest sport only uses
        // ~1000 bytes of space
        let mut data = BytesMut::with_capacity(1024);
        // the starting data for daktronics is blank spaces
        // safety: the buffer is immediately filled with empty data after the
        // size is increased
        unsafe {
            data.set_len(data.capacity());
            data.fill(b' ');
        }
    }

    /// Updates the state synchronously with the next packet that can be read
    /// from the data source. Returns a boolean indicating whether there's any
    /// new data.
    ///
    /// DO NOT USE IF YOU'RE USING ASYNC
    pub fn update(&mut self) -> Result<bool, RTDStateError<DS>> {
        let packet = match self.data_source.read_packet() {
            None => return Ok(false),
            Some(x) => x,
        }
        .map_err(RTDStateError::DataSource)?;
        self.update_from_packet(packet).map(|_| true)
    }

    /// Updates the state asynchronously with the next packet that can be read
    /// from the data source. Returns a boolean indicating whether there's any
    /// new data in the state from the packet.
    #[cfg(feature = "async")]
    pub async fn update_async(&mut self) -> Result<bool, RTDStateError<DS>> {
        let packet = match self.data_source.read_packet_async().await {
            None => return Ok(false),
            Some(x) => x,
        }
        .map_err(RTDStateError::DataSource)?;
        self.update_from_packet(packet).map(|_| true)
    }

    /// Updates the internal state based on the contents of a packet. Usually,
    /// you'll want to read a packet from a [RtdStateDataSource] using `update`
    /// or `update_async` (if that's what you're doing)
    pub fn update_from_packet(&mut self, packet: Packet) -> Result<(), RTDStateError<DS>> {
        let packet_data = packet.raw_data();
        self.data[packet.start_index as usize..packet.start_index as usize + packet_data.len()]
            .copy_from_slice(&packet_data);
        Ok(())
    }

    /// Gets the entire raw data buffer backing this [RTDState] as a &str.
    pub fn data(&self) -> Option<&str> {
        std::str::from_utf8(&self.data).ok()
    }

    /// Gets a string field from the state. This includes things like team
    /// names, etc. It also includes the clock time, since that includes the ':'
    /// and '.' characters as separators.
    ///
    /// Note that **`item` is one-based**, following Daktronics' documentation
    /// format.
    pub fn field_str(
        &self,
        item: usize,
        length: usize,
        justify: RTDFieldJustification,
    ) -> Result<&str, RTDStateFieldError> {
        let real_index = item - 1;
        let field_bytes = &self.data[real_index..real_index + length];
        let mut field_str =
            std::str::from_utf8(field_bytes).map_err(RTDStateFieldError::Utf8Error)?;
        field_str = match justify {
            RTDFieldJustification::Left => field_str.trim_end(),
            RTDFieldJustification::Right => field_str.trim_start(),
            RTDFieldJustification::None => field_str,
        };
        if field_str.is_empty() {
            Err(RTDStateFieldError::NoData)
        } else {
            Ok(field_str)
        }
    }

    /// Gets a number field from the state as a [i32]. This includes things like
    /// the score, outs, etc.
    ///
    /// Note that **`item` is one-based**, following Daktronics' documentation
    /// format.
    pub fn field_i32(
        &self,
        item: usize,
        length: usize,
        justify: RTDFieldJustification,
    ) -> Result<i32, RTDStateFieldError> {
        self.field_str(item, length, justify).and_then(|field| {
            field
                .parse::<i32>()
                .map_err(RTDStateFieldError::ParseIntError)
        })
    }

    /// Gets a boolean field from the state. Internally, Daktronics uses a space
    /// character/one letter to show a boolean, but this method assumes that ` `
    /// means `false` and anything else is `true`.
    ///
    /// Note that **`item` is one-based**, following Daktronics' documentation
    /// format.
    pub fn field_bool(&self, item: usize) -> Result<bool, RTDStateFieldError> {
        self.field_str(item, 1, RTDFieldJustification::None)
            .map(|char| !char.is_empty())
    }
}

#[derive(Debug)]
pub enum RTDStateError<DS: data_source::RTDStateDataSource> {
    DataSource(DS::Error),
    InvalidData,
    NoPacketData,
}

#[derive(Debug)]
pub enum RTDStateFieldError {
    NoData,
    ParseIntError(ParseIntError),
    Utf8Error(Utf8Error),
}

#[derive(Debug)]
pub enum RTDFieldJustification {
    Left,
    Right,
    None,
}
