//! Traits for implementing a data source, and one built-in implementor
//!
//! This module contains a trait to be passed to `RTDState`. It provides one
//! method specifying a way to read a packet from a data source, whether that be
//! UDP, serial, or something else.
//!
//! The serial implementation is built-in, but the maintainer doesn't have any
//! hardware to test the UDP support that seems to be available for getting the
//! real-time data (RTD) to a computer. See [`serial_stream_data_source`].

use std::fmt::{Debug, Display};

use crate::packet::Packet;

/// The trait to implement for data sources
///
/// It must be `Send`able because of future considerations. It should implement
/// either read_packet or read_packet_async (but read_packet can panic or return
/// `None` for non-async data sources).
pub trait RTDStateDataSource: Send + Debug {
    type Error: Debug + Display;

    fn read_packet(&mut self) -> Result<Option<Packet>, Self::Error>;

    #[cfg(feature = "async")]
    fn read_packet_async(
        &mut self,
    ) -> impl std::future::Future<Output = Result<Option<Packet>, Self::Error>> + Send;
}

#[cfg(feature = "async")]
pub mod serial_stream_data_source;
