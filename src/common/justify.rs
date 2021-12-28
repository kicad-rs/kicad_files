use serde::{
	de::{Deserialize, Deserializer, Error as _, SeqAccess, Visitor},
	ser::{Serialize, SerializeTupleStruct, Serializer}
};
use std::fmt::{self, Formatter};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum JustifyHoriz {
	Left,
	Center,
	Right
}

impl Default for JustifyHoriz {
	fn default() -> Self {
		Self::Center
	}
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum JustifyVert {
	Top,
	Center,
	Bottom
}

impl Default for JustifyVert {
	fn default() -> Self {
		Self::Center
	}
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Justify {
	pub horiz: JustifyHoriz,
	pub vert: JustifyVert,
	pub mirror: bool
}

impl Justify {
	pub fn new(horiz: JustifyHoriz, vert: JustifyVert, mirror: bool) -> Self {
		Self {
			horiz,
			vert,
			mirror
		}
	}
}

impl Serialize for Justify {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer
	{
		let mut sexpr = serializer.serialize_tuple_struct("justify", 3)?;

		let horiz = match self.horiz {
			JustifyHoriz::Left => Some("left"),
			JustifyHoriz::Center => None,
			JustifyHoriz::Right => Some("right")
		};
		sexpr.serialize_field(&horiz)?;

		let vert = match self.vert {
			JustifyVert::Top => Some("top"),
			JustifyVert::Center => None,
			JustifyVert::Bottom => Some("bottom")
		};
		sexpr.serialize_field(&vert)?;

		let mirror = match self.mirror {
			true => Some("mirror"),
			false => None
		};
		sexpr.serialize_field(&mirror)?;

		sexpr.end()
	}
}

impl<'de> Deserialize<'de> for Justify {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>
	{
		deserializer.deserialize_tuple_struct("justify", 3, JustifyVisitor)
	}
}

struct JustifyVisitor;

impl<'de> Visitor<'de> for JustifyVisitor {
	type Value = Justify;

	fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str("a (justify [left|right] [top|bottom] [mirror]) s-expression")
	}

	fn visit_seq<A>(self, mut seq: A) -> Result<Justify, A::Error>
	where
		A: SeqAccess<'de>
	{
		let mut pos = 0;
		let mut horiz = JustifyHoriz::default();
		let mut vert = JustifyVert::default();
		let mut mirror = false;

		while let Some(token) = seq.next_element::<&str>()? {
			match token {
				"left" if pos == 0 => {
					horiz = JustifyHoriz::Left;
					pos = 1;
				},
				"right" if pos == 0 => {
					horiz = JustifyHoriz::Right;
					pos = 1;
				},

				"top" if pos <= 1 => {
					vert = JustifyVert::Top;
					pos = 2;
				},
				"bottom" if pos <= 1 => {
					vert = JustifyVert::Bottom;
					pos = 2;
				},

				"mirror" if pos <= 2 => {
					mirror = true;
					pos = 3;
				},

				_ => {
					return Err(A::Error::custom(format_args!(
						"unexpected token {:?}",
						token
					)));
				}
			}
		}

		Ok(Justify {
			horiz,
			vert,
			mirror
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::sexpr_test_case;

	sexpr_test_case! {
		name: justify_center_center,
		input: "(justify)",
		value: Justify::default()
	}

	sexpr_test_case! {
		name: justify_center_center_mirror,
		input: "(justify mirror)",
		value: Justify::new(JustifyHoriz::default(), JustifyVert::default(), true)
	}

	sexpr_test_case! {
		name: justify_left_center,
		input: "(justify left)",
		value: Justify::new(JustifyHoriz::Left, JustifyVert::default(), false)
	}

	sexpr_test_case! {
		name: justify_left_center_mirror,
		input: "(justify left mirror)",
		value: Justify::new(JustifyHoriz::Left, JustifyVert::default(), true)
	}

	sexpr_test_case! {
		name: justify_right_center,
		input: "(justify right)",
		value: Justify::new(JustifyHoriz::Right, JustifyVert::default(), false)
	}

	sexpr_test_case! {
		name: justify_right_center_mirror,
		input: "(justify right mirror)",
		value: Justify::new(JustifyHoriz::Right, JustifyVert::default(), true)
	}

	sexpr_test_case! {
		name: justify_center_top,
		input: "(justify top)",
		value: Justify::new(JustifyHoriz::default(), JustifyVert::Top, false)
	}

	sexpr_test_case! {
		name: justify_center_top_mirror,
		input: "(justify top mirror)",
		value: Justify::new(JustifyHoriz::default(), JustifyVert::Top, true)
	}

	sexpr_test_case! {
		name: justify_center_bottom,
		input: "(justify bottom)",
		value: Justify::new(JustifyHoriz::default(), JustifyVert::Bottom, false)
	}

	sexpr_test_case! {
		name: justify_center_bottom_mirror,
		input: "(justify bottom mirror)",
		value: Justify::new(JustifyHoriz::default(), JustifyVert::Bottom, true)
	}

	sexpr_test_case! {
		name: justify_left_top,
		input: "(justify left top)",
		value: Justify::new(JustifyHoriz::Left, JustifyVert::Top, false)
	}

	sexpr_test_case! {
		name: justify_left_top_mirror,
		input: "(justify left top mirror)",
		value: Justify::new(JustifyHoriz::Left, JustifyVert::Top, true)
	}

	sexpr_test_case! {
		name: justify_right_top,
		input: "(justify right top)",
		value: Justify::new(JustifyHoriz::Right, JustifyVert::Top, false)
	}

	sexpr_test_case! {
		name: justify_right_top_mirror,
		input: "(justify right top mirror)",
		value: Justify::new(JustifyHoriz::Right, JustifyVert::Top, true)
	}

	sexpr_test_case! {
		name: justify_left_bottom,
		input: "(justify left bottom)",
		value: Justify::new(JustifyHoriz::Left, JustifyVert::Bottom, false)
	}

	sexpr_test_case! {
		name: justify_left_bottom_mirror,
		input: "(justify left bottom mirror)",
		value: Justify::new(JustifyHoriz::Left, JustifyVert::Bottom, true)
	}

	sexpr_test_case! {
		name: justify_right_bottom,
		input: "(justify right bottom)",
		value: Justify::new(JustifyHoriz::Right, JustifyVert::Bottom, false)
	}

	sexpr_test_case! {
		name: justify_right_bottom_mirror,
		input: "(justify right bottom mirror)",
		value: Justify::new(JustifyHoriz::Right, JustifyVert::Bottom, true)
	}
}
