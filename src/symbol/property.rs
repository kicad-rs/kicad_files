use crate::{
	common::{Effects, Font},
	deg,
	internal::tuple,
	mm, Deg, Unit
};
use serde::{Deserialize, Serialize};

/// This is basically [`Position`] but for some reason, KiCAD requires the angle
/// to be always present, whereas usually it is optional.
///
///  [`Position`]: crate::common::Position
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename = "at")]
pub struct PropertyPosition {
	pub x: mm,

	pub y: mm,

	pub angle: deg
}

impl PropertyPosition {
	pub fn origin() -> Self {
		Self {
			x: 0.0.mm(),
			y: 0.0.mm(),
			angle: 0.0.deg()
		}
	}

	pub const fn new(x: mm, y: mm, angle: deg) -> Self {
		Self { x, y, angle }
	}
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename = "property")]
pub struct Property {
	pub key: String,

	pub value: String,

	#[serde(with = "tuple")]
	pub id: i32,

	pub position: PropertyPosition,

	pub effects: Effects
}

impl Property {
	pub fn new<K, V>(
		key: K,
		value: V,
		id: i32,
		pos: PropertyPosition,
		font_size: mm,
		hide: bool
	) -> Self
	where
		K: Into<String>,
		V: Into<String>
	{
		let mut effects = Effects::new(Font::new(font_size));
		effects.hide = hide;
		Self {
			key: key.into(),
			value: value.into(),
			id,
			position: pos,
			effects
		}
	}
}

pub struct Key(&'static str);

impl From<Key> for String {
	fn from(key: Key) -> Self {
		key.0.into()
	}
}

impl Property {
	pub const REFERENCE_KEY: Key = Key("Reference");
	pub const REFERENCE_ID: i32 = 0;

	pub const VALUE_KEY: Key = Key("Value");
	pub const VALUE_ID: i32 = 1;

	pub const FOOTPRINT_KEY: Key = Key("Footprint");
	pub const FOOTPRINT_ID: i32 = 2;

	pub const DATASHEET_KEY: Key = Key("Datasheet");
	pub const DATASHEET_ID: i32 = 3;
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{sexpr_test_case, Unit};

	sexpr_test_case! {
		name: property,
		input: r#"(property Reference U (id 0) (at 0 0 0) (effects (font (size 1.27 1.27))))"#,
		value: Property::new(
			Property::REFERENCE_KEY,
			"U",
			Property::REFERENCE_ID,
			PropertyPosition::origin(),
			1.27.mm(),
			false
		)
	}
}
