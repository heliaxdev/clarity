//! Address type to interop with Ethereum.

#![warn(clippy::all)]
#![allow(clippy::pedantic)]

pub mod address;
pub mod error;
pub mod utils;

pub use address::Address;
pub use error::Error;
