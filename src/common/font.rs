use super::Size;
use crate::{internal::option_tuple, mm};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename = "font")]
pub struct Font {
	pub size: Size,

	#[serde(with = "option_tuple")]
	pub thickness: Option<mm>,

	pub bold: bool,

	pub italic: bool
}

impl Font {
	pub fn new(size: mm) -> Self {
		Self {
			size: Size::new(size, size),
			thickness: None,
			bold: false,
			italic: false
		}
	}

	pub fn new_bold(size: mm) -> Self {
		Self {
			size: Size::new(size, size),
			thickness: None,
			bold: true,
			italic: false
		}
	}

	pub fn new_italic(size: mm) -> Self {
		Self {
			size: Size::new(size, size),
			thickness: None,
			bold: false,
			italic: true
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{sexpr_test_case, Unit};

	sexpr_test_case! {
		name: font_simple,
		input: "(font (size 1.27 1.27))",
		value: Font::new(1.27.mm())
	}

	sexpr_test_case! {
		name: font_with_thickness,
		input: "(font (size 1.27 1.27) (thickness 0.508))",
		value: Font {
			size: Size::new(1.27.mm(), 1.27.mm()),
			thickness: Some(0.508.mm()),
			bold: false,
			italic: false
		}
	}
}
