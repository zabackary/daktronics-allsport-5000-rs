use std::error::Error;
use std::fmt::Display;

use super::RTDStateDataSource;
use futures_util::StreamExt;
use tokio_serial::{SerialPort, SerialStream};
use tokio_util::codec::{Decoder, Framed};

use crate::codecs::{SerialRTDCodec, SerialRTDCodecError};
use crate::packet::{Packet, PacketParseError};
use crate::RTDState;

/// A data source reading from a serial connection
#[derive(Debug)]
pub struct SerialStreamDataSource {
    /// The framed serial reader
    reader: Framed<SerialStream, SerialRTDCodec>,
    /// Whether the data source should skip unsupported packets
    ignore_unsupported_packets: bool,
}

// An extension to RTDState providing a helper builder
impl RTDState<SerialStreamDataSource> {
    /// Create a new [`RTDState`] from a serial stream representing a connection
    /// with the Daktronics All-Sport 5000's serial output. By passing `true` to
    /// `ignore_unsupported_packets`, it will ignore unsupported packets. As the
    /// author of the crate doesn't understand the protocol well enough to know
    /// what every packet means (and thus there are packets that the decoder
    /// doesn't understand), you should **pass `true` as of right now**.
    ///
    /// The underlying implementation creates an
    /// [`SerialStreamDataSource`] which will configure the serial stream for
    /// you. Thus, don't worry too much about the baud rate and parity bit
    /// config, since it will be set automatically.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use daktronics_allsport_5000::RTDState;
    /// # use tokio_serial::SerialPortBuilderExt; // for open_native_async
    /// # let tty_path = "/dev/ttyUSB0";
    /// # let baud_rate = 19200;
    /// # let parity = tokio_serial::Parity::None;
    /// let serial_stream = tokio_serial::new(tty_path, baud_rate)
    ///     .parity(parity)
    ///     .open_native_async()
    ///     .unwrap();
    /// let rtd_state = RTDState::from_serial_stream(serial_stream, true);
    /// ```
    #[cfg(feature = "tokio-serial")]
    pub fn from_serial_stream(
        serial_stream: tokio_serial::SerialStream,
        ignore_unsupported_packets: bool,
    ) -> Result<Self, tokio_serial::Error> {
        Ok(Self::new(SerialStreamDataSource::new(
            serial_stream,
            ignore_unsupported_packets,
        )?))
    }
}

impl RTDStateDataSource for SerialStreamDataSource {
    type Error = SerialStreamDataSourceError;

    fn read_packet(&mut self) -> Result<Option<Packet>, SerialStreamDataSourceError> {
        eprintln!("can't read synchronous packet from async SerialStreamDataSource");
        Err(SerialStreamDataSourceError::Unsupported)
    }

    async fn read_packet_async(&mut self) -> Result<Option<Packet>, SerialStreamDataSourceError> {
        let res = self
            .reader
            .next()
            .await
            .ok_or(SerialStreamDataSourceError::StreamExhausted)?;
        if self.ignore_unsupported_packets
            && matches!(
                res,
                Err(SerialRTDCodecError::PacketParseError(
                    PacketParseError::UnsupportedPacket { .. }
                ))
            )
        {
            Ok(None)
        } else {
            res.map(Some).map_err(SerialStreamDataSourceError::Codec)
        }
    }
}

impl SerialStreamDataSource {
    /// Create a new [`SerialStreamDataSource`] from a serial_stream
    ///
    /// The serial stream passed in will be automatically configured to the
    /// right parity and baud rate, so don't worry too much about it.
    ///
    /// This constructor has the parameter `ignore_unsupported_packets`, which,
    /// when `true`, will skip over packets unsupported by this crate and return
    /// `None` instead of an error when asked for a packet by the `RTDState`.
    /// **For now, please set it to `true`**, since the documentation is not
    /// public and thus not all packet types are fully understood by the crate
    /// author.
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

#[derive(Debug)]
pub enum SerialStreamDataSourceError {
    Codec(SerialRTDCodecError),
    Unsupported,
    StreamExhausted,
}

impl Display for SerialStreamDataSourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SerialStreamDataSourceError::Codec(err) => write!(f, "codec error: {err}"),
            SerialStreamDataSourceError::Unsupported => write!(f, "the operation is unsupported"),
            SerialStreamDataSourceError::StreamExhausted => {
                write!(f, "the internal serial stream has been exhausted")
            }
        }
    }
}

impl Error for SerialStreamDataSourceError {}
