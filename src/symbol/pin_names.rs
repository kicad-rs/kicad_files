use crate::{internal::option_tuple, mm};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename = "pin_names")]
pub struct PinNames {
	#[serde(with = "option_tuple")]
	pub offset: Option<mm>,

	pub hide: bool
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{sexpr_test_case, Unit};

	sexpr_test_case! {
		name: pin_names_no_offset,
		input: r#"(pin_names hide)"#,
		value: PinNames {
			offset: None,
			hide: true
		}
	}

	sexpr_test_case! {
		name: pin_names_with_offset,
		input: r#"(pin_names (offset 0.508) hide)"#,
		value: PinNames {
			offset: Some(0.508.mm()),
			hide: true
		}
	}
}
