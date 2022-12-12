use crate::{
	common::Position,
	internal::{tuple, tuple_or_default, ColorDef},
	mm, Color
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename = "junction")]
pub struct Junction {
	pub pos: Position,

	#[serde(with = "tuple")]
	pub diameter: mm,

	#[serde(with = "ColorDef")]
	pub color: Color,

	#[serde(with = "tuple_or_default", skip_serializing_if = "crate::skip_uuid")]
	pub uuid: Uuid
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{sexpr_test_case, Unit};

	sexpr_test_case! {
		name: junction,
		input: r#"(junction (at 25 31) (diameter 0.9) (color 0 0 0 0) (uuid "00000000-0000-0000-0000-000000000000"))"#,
		value: Junction {
			pos: Position::new(25.0.mm(), 31.0.mm()),
			diameter: 0.9.mm(),
			color: Color::new_alpha(0, 0, 0, 0.0),
			uuid: Uuid::nil()
		}
	}
}
