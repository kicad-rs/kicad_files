use crate::{internal::option_tuple, mm};
use serde::{Deserialize, Serialize};

fn is_default(hide: &bool) -> bool {
	!(*hide)
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename = "pin_names")]
pub struct PinNames {
	#[serde(with = "option_tuple")]
	pub offset: Option<mm>,

	#[serde(default, skip_serializing_if = "is_default")]
	pub hide: bool
}

impl PinNames {
	pub const fn new_with_offset(offset: mm) -> Self {
		Self {
			offset: Some(offset),
			hide: false
		}
	}

	pub const fn new_hidden() -> Self {
		Self {
			offset: None,
			hide: true
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{sexpr_test_case, Unit};

	sexpr_test_case! {
		name: pin_names_hidden,
		input: r#"(pin_names hide)"#,
		value: PinNames::new_hidden()
	}

	sexpr_test_case! {
		name: pin_names_with_offset,
		input: r#"(pin_names (offset 0.508))"#,
		value: PinNames::new_with_offset(0.508.mm())
	}
}
