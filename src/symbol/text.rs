use crate::common::{Effects, Position};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename = "text")]
pub struct Text {
	pub text: String,
	pub pos: Position,
	pub effects: Effects
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{common::Font, sexpr_test_case, Deg, Unit};

	sexpr_test_case! {
		name: text,
		input: r#"(text FOOBAR (at 1.27 0 0) (effects (font (size 0.635 0.635))))"#,
		value: Text {
			text: "FOOBAR".to_owned(),
			pos: Position { x: 1.27.mm(), y: 0.0.mm(), angle: Some(0.0.deg()) },
			effects: Effects::new(Font::new(0.635.mm()))
		}
	}
}
