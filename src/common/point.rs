use crate::mm;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename = "xy")]
pub struct Point {
	pub x: mm,

	pub y: mm
}

impl Point {
	pub fn new(x: mm, y: mm) -> Self {
		Self { x, y }
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{sexpr_test_case, Unit};

	sexpr_test_case! {
		name: point,
		input: "(xy 1.27 -2.54)",
		value: Point::new(1.27.mm(), -2.54.mm())
	}
}
