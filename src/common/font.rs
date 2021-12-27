use crate::mm;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename = "size")]
pub struct FontSize {
	pub height: mm,
	pub width: mm
}

impl FontSize {
	pub fn new(height: mm, width: mm) -> Self {
		Self { height, width }
	}
}

mod thickness {
	use serde::{Deserialize, Deserializer, Serialize, Serializer};

	#[derive(Debug, Deserialize, Serialize)]
	#[serde(deny_unknown_fields, rename = "thickness")]
	struct FontThickness<T>(T);

	pub(super) fn deserialize<'de, D, T>(
		deserializer: D
	) -> Result<Option<T>, D::Error>
	where
		D: Deserializer<'de>,
		T: Deserialize<'de>
	{
		serde_sexpr::Option::deserialize(deserializer)
			.map(|thn: Option<FontThickness<T>>| thn.map(|thn| thn.0))
	}

	pub(super) fn serialize<S, T>(
		this: &Option<T>,
		serializer: S
	) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
		T: Serialize + Copy
	{
		serde_sexpr::Option::serialize(&this.map(FontThickness), serializer)
	}
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename = "font")]
pub struct Font {
	pub size: FontSize,

	#[serde(with = "thickness")]
	pub thickness: Option<mm>,

	pub bold: bool,

	pub italic: bool
}

impl Font {
	pub fn new(size: mm) -> Self {
		Self {
			size: FontSize::new(size, size),
			thickness: None,
			bold: false,
			italic: false
		}
	}

	pub fn new_bold(size: mm) -> Self {
		Self {
			size: FontSize::new(size, size),
			thickness: None,
			bold: true,
			italic: false
		}
	}

	pub fn new_italic(size: mm) -> Self {
		Self {
			size: FontSize::new(size, size),
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
		name: size,
		input: "(size 1.27 1.27)",
		value: FontSize::new(1.27.mm(), 1.27.mm())
	}

	sexpr_test_case! {
		name: font_simple,
		input: "(font (size 1.27 1.27))",
		value: Font::new(1.27.mm())
	}

	sexpr_test_case! {
		name: font_with_thickness,
		input: "(font (size 1.27 1.27) (thickness 0.508))",
		value: Font {
			size: FontSize::new(1.27.mm(), 1.27.mm()),
			thickness: Some(0.508.mm()),
			bold: false,
			italic: false
		}
	}
}
