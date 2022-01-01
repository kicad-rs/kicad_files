use crate::{deg, mm};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename = "at")]
pub struct Position {
	pub x: mm,

	pub y: mm,

	#[serde(with = "serde_sexpr::Option")]
	pub angle: Option<deg>
}

impl Position {
	pub fn new(x: mm, y: mm) -> Self {
		Self { x, y, angle: None }
	}

	pub fn new_with_angle(x: mm, y: mm, angle: deg) -> Self {
		Self {
			x,
			y,
			angle: Some(angle)
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{sexpr_test_case, Deg, Unit};

	sexpr_test_case! {
		name: no_rotation,
		input: "(at 1.27 -2.54)",
		value: Position::new(1.27.mm(), -2.54.mm())
	}

	sexpr_test_case! {
		name: with_rotation,
		input: "(at 1.27 -2.54 90)",
		value: Position::new_with_angle(1.27.mm(), -2.54.mm(), 90.0.deg())
	}

	sexpr_test_case! {
		name: with_neg_rotation,
		input: "(at 1.27 -2.54 -90)",
		value: Position::new_with_angle(1.27.mm(), -2.54.mm(), -90.0.deg())
	}
}
