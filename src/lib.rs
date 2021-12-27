#![feature(const_float_classify)]
#![warn(rust_2018_idioms, unreachable_pub)]
#![forbid(unsafe_code)]

#[cfg(test)]
#[macro_use]
mod macros;

pub mod common;
pub mod unit;

pub use unit::{inch, mm, Unit};
