use crate::{
	board::Layer,
	common::{Effects, Position},
	internal::tuple_or_default
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename = "gr_text")]
pub struct Text {
	pub text: String,

	pub position: Position,

	#[serde(with = "serde_sexpr::Option")]
	pub layer: Option<Layer>,

	#[serde(with = "tuple_or_default", skip_serializing_if = "crate::skip_uuid")]
	pub tstamp: Uuid,

	pub effects: Effects
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{common::Font, sexpr_test_case, Unit};

	sexpr_test_case! {
		name: text,
		input: r#"(gr_text "some text" (at 0 0) (tstamp "00000000-0000-0000-0000-000000000000") (effects (font (size 1 1))))"#,
		value: Text {
			text: "some text".to_owned(),
			position: Position::new(0.0.mm(), 0.0.mm()),
			layer: None,
			tstamp: Uuid::nil(),
			effects: Effects::new(Font::new(1.0.mm()))
		}
	}
}
