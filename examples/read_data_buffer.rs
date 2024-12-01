/// This example demonstrates how to read data from a Daktronics AllSport 5000
/// device using a serial port. It continuously reads and prints the data buffer.
use std::env;

use daktronics_allsport_5000::RTDState;
use tokio_serial::SerialPortBuilderExt;

#[cfg(unix)]
const DEFAULT_TTY: &str = "/dev/ttyUSB0";
#[cfg(windows)]
const DEFAULT_TTY: &str = "COM1";

/// The main function initializes the serial port and continuously reads data
/// from the Daktronics AllSport 5000 device.
#[tokio::main]
async fn main() -> tokio_serial::Result<()> {
    // Get the command line arguments and determine the TTY path.
    let mut args = env::args();
    let tty_path = args.nth(1).unwrap_or_else(|| DEFAULT_TTY.into());

    // Create the serial port. Note the baud rate and parity.
    // allow because cargo gets suspicious on Windows
    #[allow(unused_mut)]
    let mut port = tokio_serial::new(tty_path, 19200)
        .parity(tokio_serial::Parity::None)
        .open_native_async()?;

    #[cfg(unix)]
    port.set_exclusive(false)
        .expect("Unable to set serial port exclusive to false");

    // Initialize the RTDState from the serial stream.
    let mut rtd_state = RTDState::from_serial_stream(port, true).unwrap();

    // Continuously update and print the data buffer.
    let mut update_result = Ok(false);
    while let Ok(_) = update_result {
        println!("{:?}", rtd_state.data().expect("couldn't read data buffer"));
        update_result = rtd_state.update_async().await;
    }

    // Print any errors that occur during the update process.
    if let Err(e) = update_result {
        eprintln!("{:?}", e);
    }
    Ok(())
}
