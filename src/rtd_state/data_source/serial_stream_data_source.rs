use super::RTDStateDataSource;
use futures_util::StreamExt;
use snafu::{ResultExt, Snafu};
use tokio_serial::{SerialPort, SerialStream};
use tokio_util::codec::{Decoder, Framed};

use crate::RTDState;
use crate::codecs::{SerialRTDCodec, SerialRTDCodecError};
use crate::packet::Packet;

/// A data source reading from a serial connection
#[derive(Debug)]
pub struct SerialStreamDataSource {
    /// The framed serial reader
    reader: Framed<SerialStream, SerialRTDCodec>,
    /// Whether the data source should skip unsupported packets
    ignore_malformed_packets: bool,
}

// An extension to RTDState providing a helper builder
impl RTDState<SerialStreamDataSource> {
    /// Create a new [`RTDState`] from a serial stream representing a connection
    /// with the Daktronics All Sport 5000's serial output.
    ///
    /// By passing `true` for `ignore_unsupported_packets`, the data source will
    /// skip over malformed and unsupported packets. (This is a breaking change
    /// from 0.4.0, where only unsupported packets were skipped.)
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
        ignore_malformed_packets: bool,
    ) -> Result<Self, tokio_serial::Error> {
        Ok(Self::new(SerialStreamDataSource::new(
            serial_stream,
            ignore_malformed_packets,
        )?))
    }
}

impl RTDStateDataSource for SerialStreamDataSource {
    type Error = SerialStreamDataSourceError;

    fn read_packet(&mut self) -> Result<Option<Packet>, SerialStreamDataSourceError> {
        panic!("can't read synchronous packet from async SerialStreamDataSource");
    }

    async fn read_packet_async(&mut self) -> Result<Option<Packet>, SerialStreamDataSourceError> {
        let res = self.reader.next().await;
        if let Some(res) = res {
            if self.ignore_malformed_packets
                && matches!(res, Err(SerialRTDCodecError::PacketParseError { .. }))
            {
                Ok(None)
            } else {
                res.map(Some).context(CodecSnafu)
            }
        } else {
            Ok(None)
        }
    }
}

impl SerialStreamDataSource {
    /// Create a new [`SerialStreamDataSource`] from a serial_stream
    ///
    /// The serial stream passed in will be automatically configured to the
    /// right parity and baud rate, so don't worry too much about it.
    ///
    /// This constructor has the parameter `ignore_malformed_packets`, which,
    /// when `true`, will skip over malformed packets and return
    /// `None` instead of an error when asked for a packet by the `RTDState`.
    /// **For now, please set it to `true`**, since the documentation is not
    /// public and thus not all packet types are fully understood by the crate
    /// author.
    pub fn new(
        mut serial_stream: tokio_serial::SerialStream,
        ignore_malformed_packets: bool,
    ) -> Result<Self, tokio_serial::Error> {
        // set up the serial port for use
        serial_stream.set_parity(tokio_serial::Parity::None)?;
        serial_stream.set_baud_rate(19200)?;

        let mut reader = SerialRTDCodec::default().framed(serial_stream);
        // no idea what this does but hopefully it helps
        reader.set_backpressure_boundary(32);

        Ok(Self {
            reader,
            ignore_malformed_packets,
        })
    }
}

#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum SerialStreamDataSourceError {
    /// An error from the underlying codec
    #[snafu(display("codec error: {}", source))]
    Codec { source: SerialRTDCodecError },
}
