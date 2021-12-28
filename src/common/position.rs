use crate::mm;
use serde::{Deserialize, Serialize};

/// This struct does not derive [`PartialEq`] because structurally, -90° and 270°
/// are different, but semantically they are the same.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename = "at")]
pub struct Position {
	pub x: mm,

	pub y: mm,

	#[serde(with = "serde_sexpr::Option")]
	pub angle: Option<i16>
}

impl Position {
	pub fn new(x: mm, y: mm) -> Self {
		Self { x, y, angle: None }
	}

	pub fn new_with_angle(x: mm, y: mm, angle: i16) -> Self {
		Self {
			x,
			y,
			angle: Some(angle)
		}
	}
}
