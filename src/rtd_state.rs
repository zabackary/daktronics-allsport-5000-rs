pub mod data_source;

use std::{num::ParseIntError, str::Utf8Error};

use bytes::BytesMut;
use data_source::RTDStateDataSource;

use crate::packet::Packet;

pub mod sport {
    //! A sport represents a way of looking at an RTDState.
    //!
    //! The underlying
    //! format is the same, but the data to be extracted from it varies based
    //! on what sport you're talking about. Use this module's [`Sport`] trait
    //! to implement your own sport, or use this crate's `sports` module for a
    //! collection of ones already implemented.
    //!
    //! ## Please read this before implementing!
    //!
    //! This may be bad practice, but [`super::RTDState`] assumes that all
    //! [`Sport`] implementers are ZSTs! That means that **all [`Sport`]
    //! implementers _must_ not have any fields**, since otherwise, some methods
    //! of [`super::RTDState`] will panic!
    //!
    //! ## Rationale and explanation
    //!
    //! Under the hood, Daktronics simply hands a large buffer of data to the
    //! serial output. Also, different sports have different fields that can be
    //! accessed. Thus, there must be some way of setting a specific way of
    //! viewing that data when `RTDState` is constructed, since otherwise, it is
    //! hard to make sense of the data. The question is, should the sports be
    //! selected at compile-time or run-time? Compile-time allows for better
    //! developer ergonomics, so that is what this create uses. It will in the
    //! future allow for dumping the data interpreted as a specific sport into
    //! a `HashMap` or something similar for run-time decisions on what sport
    //! is being used.
    //!
    //! This design for sports allows for each `RTDState` to have a different
    //! `Sport` (a viewing on the raw data itself).

    /// The trait to implement for different sports, which can then be passed
    /// to `RTDState`'s constructor
    ///
    /// For more information, please see the module-level documentation.
    ///
    ///
    /// ## Please read this before implementing!
    ///
    /// This may be bad practice, but [`super::RTDState`] assumes that all
    /// [`Sport`] implementers are ZSTs! That means that **all [`Sport`]
    /// implementers _must_ not have any fields**, since otherwise, some methods
    /// of [`super::RTDState`] will panic!
    pub trait Sport {
        fn name() -> &'static str;
        fn new() -> Self;
    }

    /// An empty sport. Useful if you don't want to use the `Sport` concept of
    /// creating views on a RTDState and would prefer to just do it yourself.
    pub struct EmptySport {}

    impl Sport for EmptySport {
        fn name() -> &'static str {
            "Empty sport"
        }
        fn new() -> Self {
            EmptySport {}
        }
    }
}

/// A struct holding the state for the real-time data transmission.
///
/// Of course, the entire state of the sports controller is not dumped to serial
/// every time an event happens, so you must hold the state in memory and update
/// it based on the incoming packets (you can do that with [`RTDState::update`]
/// or [`RTDState::update_async`]). You must also supply a [`sport::Sport`] for
/// the data to be interpreted as in the constructor, so make sure you do so.
///
/// # Examples
///
/// ## Read the raw data buffer
/// ```
/// let mut rtd_state = RTDState::from_serial_stream(serial_port, true, MySport::new()).unwrap();
///
/// while let Ok(_) = rtd_state.update_async().await {
///     println!("{:?}", rtd_state.data().expect("couldn't read data buffer"));
/// }
/// ```
pub struct RTDState<DS: RTDStateDataSource, S: sport::Sport> {
    data_source: DS,
    data: BytesMut,
    // It's quite bad for binary size, but we're kind of using each Sport as a
    // way to tell each interpretation of the underlying data source apart.
    // Thus, it is not actually used to do anything, so we don't actually use it
    // I'm not sure whether it would be good to use `PhantomData` or something
    // to solve this, so let me know by creating an issue if something seems
    // fishy.
    #[allow(unused)]
    sport: S,
}

impl<DS: data_source::RTDStateDataSource, S: sport::Sport> RTDState<DS, S> {
    /// Create a new [`RTDState`], with the provided [`RTDStateDataSource`]
    /// implementation.
    ///
    /// Prefer using [`RTDState::from_serial_stream`] if you're using the
    /// `tokio-serial` feature (on by default).
    pub fn new(data_source: DS, sport: S) -> Self {
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
        Self {
            data,
            data_source,
            sport,
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

    /// Returns the current sport this data source is being viewed as's name
    pub fn sport_name(&self) -> &'static str {
        S::name()
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

impl<DS: data_source::RTDStateDataSource + Sized, S: sport::Sport + Sized> RTDState<DS, S> {
    /// As the type layout of all sports should be the same, you can view an
    /// `RTDState` in different ways using `transmute`. You can use this
    /// function to temporarily access another sport's fields.
    pub fn as_sport<OS: sport::Sport, F, R>(&self, mut closure: F) -> R
    where
        F: FnMut(&RTDState<DS, OS>) -> R,
    {
        // Check that we can comfortably transmute between `S`` and `OS`, since
        // if either can have any value, then unsoundness and undefined
        // behavior will result.
        assert!(std::mem::size_of::<S>() == 0);
        assert!(std::mem::size_of::<OS>() == 0);

        let self_reference_as_other_sport =
            unsafe { std::mem::transmute::<&RTDState<DS, S>, &RTDState<DS, OS>>(self) };
        closure(self_reference_as_other_sport)
    }

    /// As the type layout of all sports should be the same,you can view an
    /// `RTDState` as a different sport. This methods converts an `RTDState`
    /// into one with another sport.
    pub fn into_sport<OS: sport::Sport>(self) -> RTDState<DS, OS> {
        // Check that we can comfortably transmute between `S`` and `OS`, since
        // if either can have any value, then unsoundness and undefined
        // behavior will result.
        assert_eq!(std::mem::size_of::<S>(), 0);
        assert_eq!(std::mem::size_of::<OS>(), 0);

        // Since transmute_copy will let us copy different sized types, we
        // should assert the type sizes to be sure.
        // Panics are better than undefined behavior.
        assert_eq!(
            std::mem::size_of::<RTDState<DS, S>>(),
            std::mem::size_of::<RTDState<DS, OS>>()
        );

        unsafe { std::mem::transmute_copy::<RTDState<DS, S>, RTDState<DS, OS>>(&self) }
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
