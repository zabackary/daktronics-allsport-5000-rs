#![doc = include_str!("../README.md")]

#[cfg(feature = "tokio")]
pub mod codecs;
pub mod packet;
pub mod rtd_state;
pub mod sports;

pub use rtd_state::{RTDFieldJustification, RTDState, RTDStateError, RTDStateFieldError};
