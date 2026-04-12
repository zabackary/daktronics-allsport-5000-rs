//! The main module for this crate, a struct storing the state of the real-time
//! data
//!
//! The control console spits out a bunch of packets with data about events, but
//! you need a buffer to store the data in between transmissions. That's where
//! this module comes in; you can use it to hold a buffer with the current
//! state, then access it with the sports getters available in their own module.

pub mod data_source;

use std::{num::ParseIntError, str::Utf8Error};

use bytes::BytesMut;
use data_source::RTDStateDataSource;
use snafu::{ResultExt as _, Snafu};

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
    /// If you're using the `tokio` feature, you should use [`update_async`]
    /// instead. Doing otherwise may result in a panic.
    pub fn update(&mut self) -> Result<bool, RTDStateError<DS>> {
        let packet = match self
            .data_source
            .read_packet()
            .map_err(|source| RTDStateError::DataSource { source })?
        {
            None => return Ok(false),
            Some(x) => x,
        };
        self.update_from_packet(packet).map(|_| true)
    }

    /// Updates the state asynchronously with the next packet that can be read
    /// from the data source.
    ///
    /// Returns a boolean indicating whether there's any new data in the state
    /// from the packet.
    #[cfg(feature = "async")]
    pub async fn update_async(&mut self) -> Result<bool, RTDStateError<DS>> {
        let packet = match self
            .data_source
            .read_packet_async()
            .await
            .map_err(|source| RTDStateError::DataSource { source })?
        {
            None => return Ok(false),
            Some(x) => x,
        };
        self.update_from_packet(packet).map(|_| true)
    }

    /// Updates the internal state based on the contents of a packet.
    ///
    /// Usually, you'll want to read a packet from a [`RTDStateDataSource`] using
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
    ///
    /// The `justify` parameter controls how whitespace is handled in the field.
    /// Passing `Left` will trim the right side of the value for whitespace, `Right`
    /// will trim the left, and `None` will avoid whitespace processing and will
    /// return the raw value. See [`RTDFieldJustification`] for more details.
    ///
    /// > [!WARNING]
    /// > If the item is empty or out-of-bounds, this method will return
    /// > an error. If you need to get the raw field content, regardless of whether
    /// > it is empty or not, avoid passing a justification.
    pub fn field_str(
        &self,
        item: usize,
        length: usize,
        justify: RTDFieldJustification,
    ) -> Result<&str, RTDStateFieldError> {
        let real_index = item - 1;
        if (real_index + length) > self.data.len() {
            // the field is out-of-bounds
            return Err(RTDStateFieldError::NoData);
        }
        let field_bytes = &self.data[real_index..real_index + length];
        let mut field_str = std::str::from_utf8(field_bytes).context(Utf8Snafu)?;
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
        self.field_str(item, length, justify)
            .and_then(|field| field.parse::<i32>().context(ParseIntSnafu))
    }

    /// Gets a boolean field from the state. Internally, Daktronics uses a space
    /// character/one letter to show a boolean, but this method assumes that ` `
    /// means `false` and anything else is `true`.
    ///
    /// Note that **`item` is one-based**, following Daktronics' documentation
    /// format.
    pub fn field_bool(&self, item: usize) -> Result<bool, RTDStateFieldError> {
        self.field_str(item, 1, RTDFieldJustification::None)
            .map(|char| !char.trim().is_empty())
    }
}

/// An error returned from an [`RTDState`] operation
#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum RTDStateError<DS: data_source::RTDStateDataSource> {
    /// The backing data source returned an error
    #[snafu(display("data source error: {}", source))]
    DataSource {
        // not a real source since we don't require the data source to implement `std::error::Error`
        #[snafu(source(false))]
        source: DS::Error,
    },
}

/// An error occurring while reading a field from an [`RTDState`].
///
/// As the sport implementations use the `RTDState::field_*` methods under the
/// hood, they also return this in the event of an error.
#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum RTDStateFieldError {
    /// The field is empty or out-of-bounds, so no data can be read.
    #[snafu(display("no data can be read from the field"))]
    NoData,
    /// The field's contents couldn't be parsed as an int.
    ///
    /// This can happen if the sport is configured incorrectly, or if the data
    /// is corrupted in some way.
    #[snafu(display("failed to parse int from field: {}", source))]
    ParseIntError { source: ParseIntError },
    /// The field's contents couldn't be parsed as a string.
    ///
    /// This is unlikely to occur, as the [`Packet`] constructor already
    /// validates RTD state.
    #[snafu(display("failed to parse string from field: {}", source))]
    Utf8Error { source: Utf8Error },
}

/// The justification of the field in the RTDState
///
/// Passing `Left` will trim the right side of the value for whitespace, `Right`
/// will trim the left, and `None` will avoid whitespace processing and will
/// return the raw value.
#[derive(Debug)]
pub enum RTDFieldJustification {
    Left,
    Right,
    None,
}

#[cfg(test)]
mod tests {
    use super::*;

    impl RTDStateDataSource for () {
        type Error = String;

        fn read_packet(&mut self) -> Result<Option<Packet>, Self::Error> {
            Ok(None)
        }

        #[cfg(feature = "async")]
        async fn read_packet_async(&mut self) -> Result<Option<Packet>, Self::Error> {
            Ok(None)
        }
    }

    #[test]
    fn test_field_str() {
        let mut state = RTDState::with_capacity(10, ());
        state.data[0..10].copy_from_slice(b"  hello   ");
        assert_eq!(
            state.field_str(1, 10, RTDFieldJustification::Left).unwrap(),
            "  hello"
        );
        assert_eq!(
            state
                .field_str(1, 10, RTDFieldJustification::Right)
                .unwrap(),
            "hello   "
        );
        assert_eq!(
            state.field_str(1, 10, RTDFieldJustification::None).unwrap(),
            "  hello   "
        );
    }

    #[test]
    fn test_field_i32() {
        let mut state = RTDState::with_capacity(10, ());
        state.data[0..10].copy_from_slice(b"  12345   ");
        assert_eq!(
            state.field_i32(3, 7, RTDFieldJustification::Left).unwrap(),
            12345
        );
    }

    #[test]
    fn test_field_bool_true() {
        let mut state = RTDState::with_capacity(1, ());
        state.data[0..1].copy_from_slice(b"h");
        assert!(state.field_bool(1).unwrap());
    }

    #[test]
    fn test_field_bool_false() {
        let mut state = RTDState::with_capacity(1, ());
        state.data[0..1].copy_from_slice(b" ");
        assert!(!state.field_bool(1).unwrap());
    }

    #[test]
    fn test_update_from_packet() {
        let mut state = RTDState::with_capacity(10, ());
        let packet = Packet::new(bytes::Bytes::copy_from_slice(b"test"), 0);
        state.update_from_packet(packet).unwrap();
        assert_eq!(&state.data[0..4], b"test");
    }

    #[test]
    #[should_panic]
    fn test_field_str_empty() {
        let mut state = RTDState::with_capacity(10, ());
        state.data[0..10].copy_from_slice(b"          ");
        state.field_str(1, 10, RTDFieldJustification::Left).unwrap();
    }
}
