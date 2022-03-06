//! **Schematic and Symbol Library Common Syntax**
//!
//! This module defines all syntax that is shared across the symbol library and
//! schematic file formats.

use crate::internal::{option_tuple, yes_no};
use serde::{Deserialize, Serialize};
use serde_sexpr::untagged;

mod arc;
mod circle;
mod curve;
mod fill;
mod pin_names;
mod pin_numbers;
mod polyline;
mod property;
mod rectangle;
mod stroke;

pub use arc::Arc;
pub use circle::Circle;
pub use curve::Curve;
pub use fill::{Fill, FillType};
pub use pin_names::PinNames;
pub use polyline::PolyLine;
pub use property::{Property, PropertyPosition};
pub use rectangle::Rectangle;
pub use stroke::{Stroke, StrokeType};

untagged! {
	#[derive(Clone, Debug, PartialEq)]
	pub enum SymbolContent {
		Property(Property),
		Symbol(Symbol),

		Arc(Arc),
		Circle(Circle),
		Curve(Curve),
		Rectangle(Rectangle)
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
	pub on_board: bool,

	#[serde(default, rename = "")]
	pub content: Vec<SymbolContent>
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{sexpr_test_case, Deg, Unit};

	sexpr_test_case! {
		name: empty_symbol,
		input: r#"(symbol "empty symbol" (in_bom yes) (on_board yes))"#,
		value: Symbol {
			id: "empty symbol".into(),
			extends: None,
			hide_pin_numbers: false,
			pin_names: None,
			in_bom: true,
			on_board: true,
			content: vec![]
		}
	}

	sexpr_test_case! {
		name: empty_symbol_with_property,
		input: r#"(symbol "empty symbol" (in_bom yes) (on_board yes) (property Reference U (id 0) (at -5.08 5.08 0) (effects (font (size 1.27 1.27)))))"#,
		value: Symbol {
			id: "empty symbol".into(),
			extends: None,
			hide_pin_numbers: false,
			pin_names: None,
			in_bom: true,
			on_board: true,
			content: vec![SymbolContent::Property(Property::new(
				Property::REFERENCE_KEY,
				"U",
				Property::REFERENCE_ID,
				PropertyPosition::new(-5.08.mm(), 5.08.mm(), 0.0.deg()),
				1.27.mm(),
				false
			))]
		}
	}
}
