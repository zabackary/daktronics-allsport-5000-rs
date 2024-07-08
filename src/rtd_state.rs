//! The main module for this crate, a struct storing the state of the real-time
//! data
//!
//! The control console spits out a bunch of packets with data about events, but
//! you need a buffer to store the data in between transmissions. That's where
//! this module comes in; you can use it to hold a buffer with the current
//! state, then access it with the sports getters available in their own module.

pub mod data_source;

use std::{fmt::Display, num::ParseIntError, str::Utf8Error};

use bytes::BytesMut;
use data_source::RTDStateDataSource;

use crate::packet::Packet;

/// A struct holding the state for the real-time data transmission.
///
/// Of course, the entire state of the sports controller is not dumped to serial
/// every time an event happens, so you must hold the state in memory and update
/// it based on the incoming packets (you can do that with [`RTDState::update`]
/// or [`RTDState::update_async`]). To read the fields of a sport and not the
/// raw data buffer, you must pass your [`RTDState`] to a `Sport` constructor
/// (various implementations are available in the `sports` module).
///
/// # Examples
///
/// ## Read the raw data buffer
///
/// ```no_run
/// # use daktronics_allsport_5000::RTDState;
/// #
/// # tokio_test::block_on(async {
/// let mut rtd_state = RTDState::from_serial_stream(
///     todo!(), // get the serial port from somewhere
///     true     // we want to ignore unparsable packets (recommended)
/// ).unwrap();
///
/// while let Ok(_) = rtd_state.update_async().await {
///     println!("{:?}", rtd_state.data().expect("couldn't read data buffer"));
/// }
/// # });
/// ```
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
        // ~1000 bytes of space, so we use a nice 'round' number
        Self::with_capacity(1024, data_source)
    }

    /// Create a new [`RTDState`] with the provided buffer size and data source
    ///
    /// This is intended for advanced use cases.
    ///
    /// Prefer using the plain [`RTDState::new`] if possible, but if you're in
    /// a memory-constrained situation, you might want to allocate the smallest
    /// possible buffer for the sport you're using instead of the default 1KiB.
    ///
    /// To figure out what the buffer size required for various sports is, check
    /// out the source for the `Sport` you're using, scroll down to the bottom
    /// and add the last field index with the last field length get the buffer
    /// size. Note that errors may result if a packet's start index is
    /// out-of-bounds for the buffer you allocated.
    pub fn with_capacity(capacity: usize, data_source: DS) -> Self {
        // allocate the `BytesMut` up-front then immediately fill it
        let mut data = BytesMut::with_capacity(capacity);
        // the starting data for daktronics is blank spaces
        // safety: the buffer is immediately filled with empty data after the
        // size is increased
        unsafe {
            data.set_len(data.capacity());
            // immediately fill the buffer after setting its length
            data.fill(b' ');
        }
        Self { data, data_source }
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
    /// you'll want to read a packet from a [`RTDStateDataSource`] using
    /// [`RTDState::update`] or [`RTDState::update_async`] (if that's what
    /// you're doing)
    pub fn update_from_packet(&mut self, packet: Packet) -> Result<(), RTDStateError<DS>> {
        let packet_data = packet.raw_data();
        // TODO: prevent panics by returning a soft Err when the start index
        // is out-of-bounds
        self.data[packet.start_index() as usize..packet.start_index() as usize + packet_data.len()]
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

/// An error returned from an [`RTDState`] operation
#[derive(Debug)]
pub enum RTDStateError<DS: data_source::RTDStateDataSource> {
    /// The backing data source returned an error
    DataSource(DS::Error),
}

/// An error occurring while reading a field from an [`RTDState`].
///
/// As the sport implementations use the `RTDState::field_*` methods under the
/// hood, they also return this in the event of an error.
#[derive(Debug)]
pub enum RTDStateFieldError {
    NoData,
    ParseIntError(ParseIntError),
    Utf8Error(Utf8Error),
}

impl Display for RTDStateFieldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RTDStateFieldError::NoData => write!(f, "no data can be read"),
            RTDStateFieldError::ParseIntError(e) => write!(f, "failed to parse int: {}", e),
            RTDStateFieldError::Utf8Error(e) => write!(f, "failed to parse string: {}", e),
        }
    }
}

impl std::error::Error for RTDStateFieldError {}

/// The justification of the field in the RTDState
///
/// Passing `Left` will trip the right side of the value for whitespace, `Right`
/// will trim the left, and `None` will avoid whitespace processing and will
/// return the raw value.
#[derive(Debug)]
pub enum RTDFieldJustification {
    Left,
    Right,
    None,
}
