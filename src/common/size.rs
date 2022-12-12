use crate::mm;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename = "size")]
pub struct Size {
	pub height: mm,
	pub width: mm
}

impl Size {
	pub fn new(height: mm, width: mm) -> Self {
		Self { height, width }
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{sexpr_test_case, Unit};

	sexpr_test_case! {
		name: size,
		input: "(size 1.27 1.27)",
		value: Size::new(1.27.mm(), 1.27.mm())
	}
}
