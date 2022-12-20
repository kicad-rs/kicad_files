use crate::{
	common::{Effects, Position},
	internal::tuple_or_default
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename = "label")]
pub struct Label {
	pub text: String,

	pub pos: Position,

	pub effects: Effects,

	#[serde(with = "tuple_or_default", skip_serializing_if = "crate::skip_uuid")]
	pub uuid: Uuid
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{
		common::{Font, Justify, JustifyHoriz, JustifyVert},
		sexpr_test_case, Deg as _, Unit
	};

	sexpr_test_case! {
		name: label,
		input: r#"(label "D+" (at 25 31 180) (effects (font (size 1.6 1.6)) (justify right bottom)) (uuid "00000000-0000-0000-0000-000000000000"))"#,
		value: Label {
			text: "D+".into(),
			pos: Position::new_with_angle(25.0.mm(), 31.0.mm(), 180.0.deg()),
			effects: Effects {
				font: Font::new(1.6.mm()),
				justify: Justify::new(JustifyHoriz::Right, JustifyVert::Bottom, false),
				hide: false
			},
			uuid: Uuid::nil()
		}
	}
}
