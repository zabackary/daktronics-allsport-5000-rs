# Daktronics All Sport 5000 for Rust

> A Rust implementation of decoders for the Daktronics All Sport 5000's serial
> output.

## Development goals

- Provide an implementation for reading from serial `DakSerialRTDReader` that
  implements `std::io::Read` using `tokio-serial`
- Provide a struct to hold the entire data structure in memory `Daktronics`
  given a size (different sports have different sizes, I believe)
  - Provide a way to get arbitrary fields from the response given a slice
  - Provide a way to decode a packet and update the structure in memory
- Provide a way to obtain a `HashMap` of common fields for common sports via the
  implementation of a trait `DakSportFields`
