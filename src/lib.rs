//! # Daktronics All Sport 5000 for Rust
//!
//! This crate provides a Rust implementation of decoders for the Daktronics All Sport 5000's serial output.
//!
//! ## Features
//!
//! - Decode the serial output of a Daktronics All Sport 5000 console.
//! - Optionally serialize the decoded data.
//!
//! ## Modules
//!
//! - `codecs`: Contains codecs for decoding the serial output (requires the `tokio` feature).
//! - `packet`: Contains definitions and utilities for handling packets.
//! - `rtd_state`: Contains definitions and utilities for handling RTD state.
//! - `sports`: Contains definitions and utilities for handling different sports.
//!

#[cfg(feature = "tokio")]
pub mod codecs;
pub mod packet;
pub mod rtd_state;
pub mod sports;

pub use rtd_state::{RTDFieldJustification, RTDState, RTDStateError, RTDStateFieldError};
