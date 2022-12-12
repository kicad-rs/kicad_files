use crate::{
	internal::{tuple, ColorDef},
	mm, Color
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StrokeType {
	Dash,
	DashDot,
	Dot,
	Default,
	Solid
}

impl Default for StrokeType {
	fn default() -> Self {
		Self::Default
	}
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename = "stroke")]
pub struct Stroke {
	#[serde(with = "tuple")]
	pub width: mm,

	#[serde(rename = "type", with = "tuple")]
	pub ty: StrokeType,

	#[serde(with = "ColorDef")]
	pub color: Color
}

impl Stroke {
	pub const fn new(width: mm, ty: StrokeType) -> Self {
		Self {
			width,
			ty,
			color: Color::new_alpha(0, 0, 0, 0.0)
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{sexpr_test_case, Unit};

	sexpr_test_case! {
		name: stroke,
		input: r#"(stroke (width 0.15) (type default) (color 0 0 0 0))"#,
		value: Stroke::new(0.15.mm(), StrokeType::Default)
	}
}
