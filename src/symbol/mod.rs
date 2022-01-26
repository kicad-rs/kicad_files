//! **Schematic and Symbol Library Common Syntax**
//!
//! This module defines all syntax that is shared across the symbol library and
//! schematic file formats.

use crate::internal::{option_tuple, yes_no};
use serde::{Deserialize, Serialize};

mod fill;
mod pin_names;
mod property;
mod stroke;

pub use fill::{Fill, FillType};
pub use pin_names::PinNames;
pub use property::{Property, PropertyPosition};
pub use stroke::{Stroke, StrokeType};

mod pin_numbers {
	use serde::{Deserialize, Deserializer, Serialize, Serializer};

	#[derive(Deserialize, Serialize)]
	#[serde(deny_unknown_fields, rename = "pin_numbers")]
	struct PinNumbers {
		hide: bool
	}

	pub(super) fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
	where
		D: Deserializer<'de>
	{
		serde_sexpr::Option::deserialize(deserializer).map(|v| {
			v.map(|pin_numbers: PinNumbers| pin_numbers.hide)
				.unwrap_or(false)
		})
	}

	pub(super) fn serialize<S>(this: &bool, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer
	{
		this.then(|| PinNumbers { hide: true })
			.serialize(serializer)
	}
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename = "symbol")]
pub struct Symbol {
	pub id: String,

	#[serde(with = "option_tuple")]
	pub extends: Option<String>,

	#[serde(with = "pin_numbers")]
	pub hide_pin_numbers: bool,

	#[serde(with = "serde_sexpr::Option")]
	pub pin_names: Option<PinNames>,

	#[serde(with = "yes_no")]
	pub in_bom: bool,

	#[serde(with = "yes_no")]
	pub on_board: bool
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::sexpr_test_case;

	sexpr_test_case! {
		name: empty_symbol,
		input: r#"(symbol "empty symbol" (in_bom yes) (on_board yes))"#,
		value: Symbol {
			id: "empty symbol".into(),
			extends: None,
			hide_pin_numbers: false,
			pin_names: None,
			in_bom: true,
			on_board: true
		}
	}
}
