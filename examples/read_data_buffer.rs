use std::env;

use daktronics_allsport_5000::RtdState;
use tokio_serial::SerialPortBuilderExt;

#[cfg(unix)]
const DEFAULT_TTY: &str = "/dev/ttyUSB0";
#[cfg(windows)]
const DEFAULT_TTY: &str = "COM1";

#[tokio::main]
async fn main() -> tokio_serial::Result<()> {
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

    let mut rtd_state = RtdState::from_serial_stream(port, true).unwrap();

    let mut update_result = Ok(false);
    while let Ok(_) = update_result {
        println!("{:?}", rtd_state.data().expect("couldn't read data buffer"));
        update_result = rtd_state.update_async().await;
    }
    if let Err(e) = update_result {
        eprintln!("{:?}", e);
    }
    Ok(())
}
