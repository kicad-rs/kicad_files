#![warn(rust_2018_idioms, unreachable_pub)]
#![forbid(unsafe_code)]

pub use millimeter::{mm, Unit};

#[cfg(test)]
#[macro_use]
mod macros;
mod internal;

pub mod board;
pub mod common;
