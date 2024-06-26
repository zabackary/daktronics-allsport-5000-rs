use super::RTDStateDataSource;
use futures_util::StreamExt;
use tokio_serial::{SerialPort, SerialStream};
use tokio_util::codec::{Decoder, Framed};

use crate::codecs::{Packet, PacketParseError, SerialRTDCodec, SerialRTDCodecError};

#[derive(Debug)]
pub struct SerialStreamDataSource {
    reader: Framed<SerialStream, SerialRTDCodec>,
    ignore_unsupported_packets: bool,
}

impl RTDStateDataSource for SerialStreamDataSource {
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
