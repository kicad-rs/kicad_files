use crate::internal::tuple;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FillType {
	None,
	Outline,
	Background
}

impl Default for FillType {
	fn default() -> Self {
		Self::None
	}
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename = "fill")]
pub struct Fill {
	#[serde(rename = "type", with = "tuple")]
	pub ty: FillType
}

impl Fill {
	pub const fn new(ty: FillType) -> Self {
		Self { ty }
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::sexpr_test_case;

	sexpr_test_case! {
		name: fill,
		input: r#"(fill (type none))"#,
		value: Fill::new(FillType::None)
	}
}
