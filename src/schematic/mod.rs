//! **Schematic File Format**
//!
//! This module defines all the syntax used in the schematic file format that is not
//! shared with symbol libraries.

use crate::internal::{tuple, tuple_or_default};
use serde::{Deserialize, Serialize};
use serde_sexpr::untagged;
use uuid::Uuid;

mod bus_entry;
mod junction;
mod lib_symbols;
mod no_connect;

pub use bus_entry::BusEntry;
pub use junction::Junction;
pub use lib_symbols::LibSymbols;
pub use no_connect::NoConnect;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename = "version")]
pub struct Version(u32);

impl Default for Version {
	fn default() -> Self {
		Self(20211123)
	}
}

impl Version {
	pub fn new() -> Self {
		Self::default()
	}
}

untagged! {
	#[derive(Clone, Debug, PartialEq)]
	pub enum SchematicContent {
		Junction(Junction)
	}
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename = "kicad_sch")]
pub struct Schematic {
	pub version: Version,

	#[serde(with = "tuple")]
	pub generator: String,

	#[serde(with = "tuple_or_default", skip_serializing_if = "crate::skip_uuid")]
	pub uuid: Uuid,

	pub lib_symbols: LibSymbols
}
