use crate::{
	deg,
	internal::{option_tuple, tuple},
	mm
};
use serde::{Deserialize, Serialize};
use std::ops::Add;

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename = "xyz")]
pub struct Xyz<T> {
	pub x: T,
	pub y: T,
	pub z: T
}

impl<T> Xyz<T> {
	pub fn new(x: T, y: T, z: T) -> Self {
		Self { x, y, z }
	}
}

impl<T> Add for Xyz<T>
where
	T: Add
{
	type Output = Xyz<T::Output>;

	fn add(self, rhs: Self) -> Self::Output {
		Xyz {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
			z: self.z + rhs.z
		}
	}
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields, rename = "model")]
struct ModelDef {
	file: String,

	#[serde(with = "option_tuple")]
	at: Option<Xyz<mm>>,

	#[serde(with = "option_tuple")]
	offset: Option<Xyz<mm>>,

	#[serde(with = "tuple")]
	scale: Xyz<f32>,

	#[serde(with = "tuple")]
	rotate: Xyz<deg>
}

impl From<ModelDef> for Model {
	fn from(def: ModelDef) -> Self {
		Self {
			file: def.file,
			offset: def.at.unwrap_or_default() + def.offset.unwrap_or_default(),
			scale: def.scale,
			rotate: def.rotate
		}
	}
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(from = "ModelDef", rename = "model")]
pub struct Model {
	pub file: String,

	#[serde(with = "tuple")]
	pub offset: Xyz<mm>,

	#[serde(with = "tuple")]
	pub scale: Xyz<f32>,

	#[serde(with = "tuple")]
	pub rotate: Xyz<deg>
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{sexpr_test_case, Deg, Unit};

	sexpr_test_case! {
		name: model_with_offset,
		input: r#"(model "model.wrl" (offset (xyz 5 -1 3)) (scale (xyz 1 1 1)) (rotate (xyz 0 0 0)))"#,
		value: Model {
			file: "model.wrl".to_owned(),
			offset: Xyz::new(5.0.mm(), -1.0.mm(), 3.0.mm()),
			scale: Xyz::new(1.0, 1.0, 1.0),
			rotate: Xyz::new(0.0.deg(), 0.0.deg(), 0.0.deg())
		}
	}

	#[test]
	fn test_deserialize_model_with_at() {
		let input = r#"(model "model.wrl" (at (xyz 5 -1 3)) (scale (xyz 1 1 1)) (rotate (xyz 0 0 0)))"#;
		let value = Model {
			file: "model.wrl".to_owned(),
			offset: Xyz::new(5.0.mm(), -1.0.mm(), 3.0.mm()),
			scale: Xyz::new(1.0, 1.0, 1.0),
			rotate: Xyz::new(0.0.deg(), 0.0.deg(), 0.0.deg())
		};

		let parsed: Model =
			serde_sexpr::from_str(input).expect("Failed to parse input");
		pretty_assertions::assert_eq!(parsed, value);
	}
}
