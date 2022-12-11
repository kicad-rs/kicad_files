#![warn(rust_2018_idioms, unreachable_pub)]
#![forbid(elided_lifetimes_in_paths, unsafe_code)]
// clippy wants me to write -(1.0.mm()) instead of -1.0.mm().
// I don't think so
#![allow(clippy::precedence)]

//! A library to read KiCAD v6 file formats.
//! 
//! **This crate is not affiliated with KiCAD.**
//! 
//! Currently, the following file formats are supported:
//! 
//!  - KiCAD v5 Footprint file (`*.kicad_mod`), starting with `(module`
//!  - KiCAD v6 Footprint file (`*.kicad_mod`), starting with `(footprint`
//!  - KiCAD v6 Symbol library (`*.kicad_sym`), starting with `(kicad_symbol_lib`

pub use millimeter::{mm, Unit};
use uuid::Uuid;

#[cfg(test)]
#[macro_use]
mod macros;

mod degree;
mod internal;

pub mod board;
pub mod common;
pub mod symbol;
pub mod symbol_lib;

pub use degree::{deg, Deg};

#[cfg(not(feature = "skip_nil_uuids"))]
fn skip_uuid(_: &Uuid) -> bool {
	false
}

#[cfg(feature = "skip_nil_uuids")]
fn skip_uuid(u: &Uuid) -> bool {
	u.is_nil()
}
