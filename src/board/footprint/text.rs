use crate::{
	board::Layer,
	common::{Effects, Position},
	internal::tuple
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TextType {
	Reference,
	Value,
	User
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename = "fp_text")]
pub struct Text {
	pub ty: TextType,

	pub text: String,

	pub position: Position,

	/// Indicates if the text orientation can be anything other than the upright
	/// orientation.
	pub unlocked: bool,

	pub layer: Layer,

	pub hide: bool,

	pub effects: Effects,

	#[serde(with = "tuple")]
	pub tstamp: Uuid
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{common::Font, sexpr_test_case, Unit};

	sexpr_test_case! {
		name: text,
		input: r#"(fp_text reference "REF**" (at 0 0) (layer "F.SilkS") (effects (font (size 1 1))) (tstamp "00000000-0000-0000-0000-000000000000"))"#,
		value: Text {
			ty: TextType::Reference,
			text: "REF**".to_owned(),
			position: Position::new(0.0.mm(), 0.0.mm()),
			unlocked: false,
			layer: Layer::new("F.SilkS"),
			hide: false,
			effects: Effects::new(Font::new(1.0.mm())),
			tstamp: Uuid::nil()
		}
	}
}
