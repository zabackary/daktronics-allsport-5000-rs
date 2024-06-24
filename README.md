# Daktronics All Sport 5000 for Rust

> A Rust implementation of decoders for the Daktronics All Sport 5000's serial
> output.

## Development goals

- Provide a codec to read packets from the serial output `SerialRTDCodec`
- Provide a convince struct for reading packets from a serial connection into
  `SerialRTDCodec`
- Provide a struct to hold the entire data structure in memory `Daktronics`
  given a size (different sports have different sizes, I believe)
  - Provide a way to get arbitrary fields from the response given a slice
  - Provide a way to decode a packet and update the structure in memory
- Provide a way to obtain a `HashMap` of common fields for common sports via the
  implementation of a trait `DakSportFields`
