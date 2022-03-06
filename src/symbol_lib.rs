//! **Symbol Library**
//!
//! This module defines syntax that is used by the symbol library.

use crate::{internal::tuple, symbol::Symbol};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename = "version")]
pub struct Version(u32);

impl Default for Version {
	fn default() -> Self {
		Self(20211014)
	}
}

impl Version {
	pub fn new() -> Self {
		Self::default()
	}
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename = "kicad_symbol_lib")]
pub struct SymbolLib {
	pub version: Version,

	#[serde(with = "tuple")]
	pub generator: String,

	#[serde(default, rename = "")]
	pub symbols: Vec<Symbol>
}

impl SymbolLib {
	pub fn from_str(s: &str) -> Result<Self, serde_sexpr::de::Error> {
		serde_sexpr::from_str(s)
	}
}
