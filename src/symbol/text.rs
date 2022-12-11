use crate::common::{Effects, Position};
use serde::{Deserialize, Serialize};

// for some weird reason, the angle is stored as one tenth of the angle, and only for
// this one type.
mod one_tenth_angle {
	use crate::{common::Position, deg, mm};
	use serde::{Deserialize, Deserializer, Serialize, Serializer};

	#[derive(Deserialize, Serialize)]
	#[serde(deny_unknown_fields, rename = "at")]
	struct PositionDef {
		x: mm,
		y: mm,

		#[serde(with = "serde_sexpr::Option")]
		angle: Option<f32>
	}

	impl From<Position> for PositionDef {
		fn from(pos: Position) -> Self {
			Self {
				x: pos.x,
				y: pos.y,
				angle: pos.angle.map(|angle| angle.raw_value() * 10.0)
			}
		}
	}

	impl From<PositionDef> for Position {
		fn from(def: PositionDef) -> Self {
			Self {
				x: def.x,
				y: def.y,
				angle: def.angle.map(|angle| deg::new(angle / 10.0))
			}
		}
	}

	pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Position, D::Error>
	where
		D: Deserializer<'de>
	{
		PositionDef::deserialize(deserializer).map(Into::into)
	}

	pub(crate) fn serialize<S>(
		this: &Position,
		serializer: S
	) -> Result<S::Ok, S::Error>
	where
		S: Serializer
	{
		PositionDef::serialize(&(*this).into(), serializer)
	}
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename = "text")]
pub struct Text {
	pub text: String,

	#[serde(with = "one_tenth_angle")]
	pub pos: Position,

	pub effects: Effects
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{common::Font, sexpr_test_case, Deg, Unit};

	sexpr_test_case! {
		name: text,
		input: r#"(text FOOBAR (at 1.27 0 900) (effects (font (size 0.635 0.635))))"#,
		value: Text {
			text: "FOOBAR".to_owned(),
			pos: Position { x: 1.27.mm(), y: 0.0.mm(), angle: Some(90.0.deg()) },
			effects: Effects::new(Font::new(0.635.mm()))
		}
	}
}
