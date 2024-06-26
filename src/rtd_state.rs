use bytes::BytesMut;
use data_source::RtdStateDataSource;

use crate::codecs::Packet;

pub struct RtdState<DS: RtdStateDataSource> {
    data_source: DS,
    data: BytesMut,
}

pub mod data_source {

    use crate::codecs::Packet;

    pub trait RtdStateDataSource {
        type Error;

        fn read_packet(&mut self) -> Option<Result<Packet, Self::Error>>;

        #[cfg(feature = "async")]
        fn read_packet_async(
            &mut self,
        ) -> impl std::future::Future<Output = Option<Result<Packet, Self::Error>>> + Send;
    }

    pub use serial_stream_data_source::SerialStreamDataSource;

    #[cfg(feature = "async")]
    mod serial_stream_data_source {
        use super::RtdStateDataSource;
        use futures_util::StreamExt;
        use tokio_serial::{SerialPort, SerialStream};
        use tokio_util::codec::{Decoder, Framed};

        use crate::codecs::{Packet, PacketParseError, SerialRTDCodec, SerialRTDCodecError};

        #[derive(Debug)]
        pub struct SerialStreamDataSource {
            reader: Framed<SerialStream, SerialRTDCodec>,
            ignore_unsupported_packets: bool,
        }

        impl RtdStateDataSource for SerialStreamDataSource {
            type Error = SerialRTDCodecError;

            fn read_packet(&mut self) -> Option<Result<Packet, SerialRTDCodecError>> {
                panic!("can't read synonymous packet from async SerialStreamDataSource")
            }

            async fn read_packet_async(&mut self) -> Option<Result<Packet, SerialRTDCodecError>> {
                let res = self.reader.next().await;
                if self.ignore_unsupported_packets {
                    match res {
                        Some(Err(SerialRTDCodecError::PacketParseError(
                            PacketParseError::UnsupportedPacket { header_bytes: _ },
                        ))) => None,
                        x => x,
                    }
                } else {
                    res
                }
            }
        }

        impl SerialStreamDataSource {
            pub fn new(
                mut serial_stream: tokio_serial::SerialStream,
                ignore_unsupported_packets: bool,
            ) -> Result<Self, tokio_serial::Error> {
                // set up the serial port for use
                serial_stream.set_parity(tokio_serial::Parity::None)?;
                serial_stream.set_baud_rate(19200)?;

                Ok(Self {
                    reader: SerialRTDCodec::default().framed(serial_stream),
                    ignore_unsupported_packets,
                })
            }
        }
    }
}

impl RtdState<data_source::SerialStreamDataSource> {
    #[cfg(feature = "tokio-serial")]
    pub fn from_serial_stream(
        serial_stream: tokio_serial::SerialStream,
        ignore_unsupported_packets: bool,
    ) -> Result<Self, tokio_serial::Error> {
        Ok(Self::new(data_source::SerialStreamDataSource::new(
            serial_stream,
            ignore_unsupported_packets,
        )?))
    }
}

impl<DS: data_source::RtdStateDataSource> RtdState<DS> {
    pub fn new(data_source: DS) -> Self {
        // as far as I can tell from the docs, the largest sport only uses
        // ~1000 bytes of space
        let mut data = BytesMut::with_capacity(1000);
        // the starting data for daktronics is blank spaces
        // safety: the buffer is immediately filled with empty data after the
        // size is increased
        unsafe {
            data.set_len(1000);
        }
        data.fill(b' ');
        Self { data, data_source }
    }

    /// Updates the state synchronously with the next packet that can be read
    /// from the data source. Returns a boolean indicating whether there's any
    /// new data.
    ///
    /// DO NOT USE IF YOU'RE USING ASYNC
    pub fn update(&mut self) -> Result<bool, RtdStateError<DS>> {
        let packet = match self.data_source.read_packet() {
            None => return Ok(false),
            Some(x) => x,
        }
        .map_err(RtdStateError::DataSource)?;
        self.update_from_packet(packet).map(|_| true)
    }

    /// Updates the state asynchronously with the next packet that can be read
    /// from the data source. Returns a boolean indicating whether there's any
    /// new data in the state from the packet.
    #[cfg(feature = "async")]
    pub async fn update_async(&mut self) -> Result<bool, RtdStateError<DS>> {
        let packet = match self.data_source.read_packet_async().await {
            None => return Ok(false),
            Some(x) => x,
        }
        .map_err(RtdStateError::DataSource)?;
        self.update_from_packet(packet).map(|_| true)
    }

    /// Updates the internal state based on the contents of a packet. Usually,
    /// you'll want to read a packet from a [RtdStateDataSource] using `update`
    /// or `update_async` (if that's what you're doing)
    pub fn update_from_packet(&mut self, packet: Packet) -> Result<(), RtdStateError<DS>> {
        let packet_data = packet.raw_data();
        self.data[packet.start_index as usize..packet.start_index as usize + packet_data.len()]
            .copy_from_slice(&packet_data);
        Ok(())
    }

    pub fn data(&self) -> Option<&str> {
        std::str::from_utf8(&self.data).ok()
    }
}

#[derive(Debug)]
pub enum RtdStateError<DS: data_source::RtdStateDataSource> {
    DataSource(DS::Error),
    InvalidData,
    NoPacketData,
}
