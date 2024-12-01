use std::env;

use daktronics_allsport_5000::{
    rtd_state::data_source::RTDStateDataSource,
    sports::{basketball::BasketballSport, Sport},
    RTDState,
};
use tokio_serial::SerialPortBuilderExt;

#[cfg(unix)]
const DEFAULT_TTY: &str = "/dev/ttyUSB0";
#[cfg(windows)]
const DEFAULT_TTY: &str = "COM1";

/// Main function to dump basketball data as JSON.
///
/// This function initializes a serial port connection to the AllSport 5000
/// controller, reads the basketball game data, and prints it as JSON to the console.
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

    // Initialize the BasketballSport with the RTDState from the serial stream.
    let mut sport = BasketballSport::new(RTDState::from_serial_stream(port, true).unwrap());

    // Continuously update and print the basketball game data as JSON.
    let mut update_result = Ok(false);
    while let Ok(_) = update_result {
        println!(
            "{}",
            serde_json::to_string(&sport).expect("couldn't read data")
        );
        update_result = sport.rtd_state().update_async().await;
    }
    if let Err(e) = update_result {
        eprintln!("{:?}", e);
    }
    Ok(())
}
