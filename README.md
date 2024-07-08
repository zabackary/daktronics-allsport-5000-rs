# Daktronics All Sport 5000 for Rust

[![Documentation](https://docs.rs/daktronics-allsport-5000/badge.svg)](https://docs.rs/daktronics-allsport-5000)
[![Crates.io](https://img.shields.io/crates/v/daktronics-allsport-5000.svg)](https://crates.io/crates/daktronics-allsport-5000)
[![License](https://img.shields.io/crates/l/daktronics-allsport-5000.svg)](https://github.com/daktronics-allsport-5000-rs/iced/blob/master/LICENSE)
[![Downloads](https://img.shields.io/crates/d/daktronics-allsport-5000.svg)](https://crates.io/crates/daktronics-allsport-5000)

A Rust implementation of decoders for the Daktronics All Sport 5000's serial
output.

Please see the [documentation](https://docs.rs/daktronics-allsport-5000).

**If you use this crate: let me know about your use case by
[creating a GitHub discussion](https://github.com/zabackary/daktronics-allsport-5000-rs/discussions)!
It also lets me know other people find this crate interesting and useful.**

## High-level access

Create a new `RTDState` instance with `RTDState::from_serial_stream` (available
with default features enabled).

If you need any help, there should be more extensive documentation for each item
at [docs.rs](https://docs.rs/daktronics-allsport-5000). Don't hesitate to
[create a GitHub issue](https://github.com/zabackary/daktronics-allsport-5000-rs/issues)
if something is unclear, either.

### Sports

This crate supports all sports the control console supports. For a list of them,
see the
[`sports`](https://docs.rs/daktronics-allsport-5000/latest/daktronics_allsport_5000/sports/index.html)
module documentation.

### Examples

#### Getting a sport-specific field

```no_run
use daktronics_allsport_5000::{
    RTDState,
    // there are lots of other sports available in their respective modules
    sports::basketball::BasketballSport
};
use tokio_serial::SerialPortBuilderExt; // for open_native_async
use crate::daktronics_allsport_5000::sports::Sport; // for rtd_state

#[tokio::main]
async fn main() {
    let serial_stream = tokio_serial::new("/dev/ttyUSB0", 19200)
        .parity(tokio_serial::Parity::None)
        .open_native_async()
        .unwrap();
    let rtd_state = RTDState::from_serial_stream(serial_stream, true).unwrap();
    let mut basketball = BasketballSport::new(rtd_state);

    loop {
        // get the underlying rtd_state to update it
        let update_result = basketball.rtd_state().update_async().await.unwrap();

        basketball.main_clock_time(); // -> Result<&str, ...>
    }
}
```

#### With Serde

Enable the `serde` feature to enable serialization for sports.

```ignore
use tokio;
use daktronics_allsport_5000::{
    RTDState,
    sports::basketball::BasketballSport
};
use tokio_serial::SerialPortBuilderExt; // for open_native_async
use crate::daktronics_allsport_5000::sports::Sport; // for rtd_state

#[tokio::main]
async fn main() {
    let serial_stream = tokio_serial::new("/dev/ttyUSB0", 19200)
        .parity(tokio_serial::Parity::None)
        .open_native_async()
        .unwrap();
    let rtd_state = RTDState::from_serial_stream(serial_stream, true).unwrap();
    let basketball = BasketballSport::new(rtd_state);

    loop {
        // get the underlying rtd_state to update it
        let update_result = basketball
            .rtd_state()
            .update_async()
            .await
            .unwrap();

        serde_json::to_string(&basketball); // -> Result<String, ...>
    }
}
```

#### Getting the raw data buffer

```no_run
use tokio;
use daktronics_allsport_5000::{RTDState, RTDFieldJustification};
use tokio_serial::SerialPortBuilderExt; // for open_native_async

#[tokio::main]
async fn main() {
    let serial_stream = tokio_serial::new("/dev/ttyUSB0", 19200)
        .parity(tokio_serial::Parity::None)
        .open_native_async()
        .unwrap();
    let mut rtd_state = RTDState::from_serial_stream(serial_stream, true).unwrap();

    loop {
        let update_result = rtd_state.update_async().await.unwrap();

        // do something with `rtd_state`
        rtd_state.field_str(1, 5, RTDFieldJustification::Left); // -> Result<&str, ...>
    }
}
```

## In other languages...

The same concept as this crate is also implemented in
[Python by @FlantasticDan](https://github.com/FlantasticDan/scorebox-consoles),
[C# by @JimThatcher](https://github.com/JimThatcher/sport-streamer), and
[Python again by @fimion](https://github.com/fimion/pydak). In fact, the data in
this crate is extracted from a PDF provided by @fimion, so thank you!

If you're interested in porting this crate to another language, check out the
Excel spreadsheet I compiled with the data that underpins this crate in
[`./sports_data`](./sports_data/).

## Low-level access

When used with the `tokio` feature, this package provides a `tokio-util` codec
implementing `Decoder` to decode packets from a serial stream from the control
console. That is used internally in `SerialStreamDataSource` to route data into
`RTDState`.

If you're not using `tokio` or you're not using a serial stream to deliver the
data (e.g. using UDP), you must get the packets somehow yourself, but after
that, you can use `daktronics_allsport_5000::packet::Packet`'s
`TryFrom<bytes::Bytes>` implementation to parse the packet into a readable
format. Then, you can provide that to the main `RTDState` struct by implementing
`daktronics_allsport_5000::rtd_state::data_source::RTDStateDataSource` then
giving that to `RTDState::new`. After that, everything works as normal.
