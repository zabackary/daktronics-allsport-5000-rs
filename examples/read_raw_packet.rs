use std::env;

use daktronics_allsport_5000::codecs;
use futures_util::StreamExt;
use tokio_serial::SerialPortBuilderExt;
use tokio_util::codec::Decoder;

#[cfg(unix)]
const DEFAULT_TTY: &str = "/dev/ttyUSB0";
#[cfg(windows)]
const DEFAULT_TTY: &str = "COM1";

/// Prints packets of information from the console to the terminal, separated
/// by newlines. Doesn't attempt to do any parsing.
#[tokio::main]
async fn main() -> tokio_serial::Result<()> {
    // Get the command line arguments
    let mut args = env::args();
    // Use the first argument as the TTY path or default to the predefined path
    let tty_path = args.nth(1).unwrap_or_else(|| DEFAULT_TTY.into());

    // Create the serial port. Note the baud rate and parity.
    // allow because cargo gets suspicious on Windows
    #[allow(unused_mut)]
    let mut port = tokio_serial::new(tty_path, 19200)
        .parity(tokio_serial::Parity::None)
        .open_native_async()?;

    #[cfg(unix)]
    // Set the serial port to non-exclusive mode on Unix systems
    port.set_exclusive(false)
        .expect("Unable to set serial port exclusive to false");

    // Create the codec reader
    let mut reader = codecs::SerialRTDCodec::default().framed(port);

    // Read packets from the serial port and print them
    while let Some(packet_bytes_result) = reader.next().await {
        let packet = packet_bytes_result.expect("failed to read packet");
        println!("{:?}", packet);
    }
    Ok(())
}
