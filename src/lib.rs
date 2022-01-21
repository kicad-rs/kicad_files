#![warn(rust_2018_idioms, unreachable_pub)]
#![forbid(unsafe_code)]

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
