# Daktronics All Sport 5000 for Rust

> A Rust implementation of decoders for the Daktronics All Sport 5000's serial
> output.

## High-level access

Create a new `RTDState` instance with `RTDState::from_serial_stream`.

### Example

```rust
# use tokio;
use daktronics_allsport_5000::RTDState;

# #[tokio::main]
# fn main() {
let serial_stream = tokio_serial::new(tty_path, 19200)
    .parity(tokio_serial::Parity::None)
    .open_native_async()?;
let rtd_state = RTDState::from_serial_stream(serial_stream, true);

while true {
    let update_result = rtd_state.update_async().await;
    if let Err(e) = update_result {
        eprintln!("{:?}", e);
        break;
    }

    // do something with `rtd_state`
}
# }
```

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
giving that to `RTDState::new`.
