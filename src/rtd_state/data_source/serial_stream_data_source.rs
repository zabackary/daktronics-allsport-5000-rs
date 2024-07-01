use super::RTDStateDataSource;
use futures_util::StreamExt;
use tokio_serial::{SerialPort, SerialStream};
use tokio_util::codec::{Decoder, Framed};

use crate::codecs::{SerialRTDCodec, SerialRTDCodecError};
use crate::packet::{Packet, PacketParseError};
use crate::RTDState;

#[derive(Debug)]
pub struct SerialStreamDataSource {
    reader: Framed<SerialStream, SerialRTDCodec>,
    ignore_unsupported_packets: bool,
}

impl<S: crate::rtd_state::sport::Sport> RTDState<SerialStreamDataSource, S> {
    /// Create a new [`RTDState`] from a serial stream representing a connection
    /// with the Daktronics All-Sport 5000's serial output. By passing `true` to
    /// `ignore_unsupported_packets`, it will ignore unsupported packets. As the
    /// author of the crate doesn't understand the protocol well enough to know
    /// what every packet means (and thus there are packets that the decoder
    /// doesn't understand), you should pass `true` as of right now.
    ///
    /// The underlying implementation creates an
    /// [`data_source::SerialStreamDataSource`] which will configure the serial
    /// stream for you. Thus, don't worry too much about the baud rate and
    /// parity bit config.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use daktronics_allsport_5000::RTDState;
    /// # let tty_path = "/dev/ttyUSB0";
    /// # let baud_rate = 19200;
    /// # let parity = tokio_serial::Parity::None;
    /// let serial_stream = tokio_serial::new(tty_path, baud_rate)
    ///     .parity(parity)
    ///     .open_native_async()?;
    /// let rtd_state = RTDState::from_serial_stream(serial_stream, true);
    /// ```
    #[cfg(feature = "tokio-serial")]
    pub fn from_serial_stream(
        serial_stream: tokio_serial::SerialStream,
        ignore_unsupported_packets: bool,
        sport: S,
    ) -> Result<Self, tokio_serial::Error> {
        Ok(Self::new(
            SerialStreamDataSource::new(serial_stream, ignore_unsupported_packets)?,
            sport,
        ))
    }
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
