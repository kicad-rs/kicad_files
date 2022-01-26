use crate::{internal::option_tuple, mm};
use monostate::MustBe;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename = "pin_names")]
pub struct PinNames {
	#[serde(with = "option_tuple")]
	pub offset: Option<mm>,

	hide: MustBe!(true)
}

impl PinNames {
	pub const fn new(offset: Option<mm>) -> Self {
		Self {
			offset,
			hide: MustBe!(true)
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{sexpr_test_case, Unit};

	sexpr_test_case! {
		name: pin_names_no_offset,
		input: r#"(pin_names hide)"#,
		value: PinNames::new(None)
	}

	sexpr_test_case! {
		name: pin_names_with_offset,
		input: r#"(pin_names (offset 0.508) hide)"#,
		value: PinNames::new(Some(0.508.mm()))
	}
}
