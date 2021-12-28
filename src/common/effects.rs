use super::{Font, Justify};
use serde::{Deserialize, Deserializer, Serialize};

fn justify_deserialize<'de, D>(deserializer: D) -> Result<Justify, D::Error>
where
	D: Deserializer<'de>
{
	serde_sexpr::Option::deserialize(deserializer).map(Option::unwrap_or_default)
}

fn justify_is_default(justify: &Justify) -> bool {
	*justify == Justify::default()
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename = "effects")]
pub struct Effects {
	pub font: Font,

	#[serde(
		deserialize_with = "justify_deserialize",
		skip_serializing_if = "justify_is_default"
	)]
	pub justify: Justify,

	pub hide: bool
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{common::JustifyVert, sexpr_test_case, Unit};

	sexpr_test_case! {
		name: effects_simple,
		input: "(effects (font (size 1.27 1.27)))",
		value: Effects {
			font: Font::new(1.27.mm()),
			justify: Justify::default(),
			hide: false
		}
	}

	sexpr_test_case! {
		name: effects_hide,
		input: "(effects (font (size 1.27 1.27)) hide)",
		value: Effects {
			font: Font::new(1.27.mm()),
			justify: Justify::default(),
			hide: true
		}
	}

	sexpr_test_case! {
		name: effects_justify,
		input: "(effects (font (size 1.27 1.27)) (justify bottom))",
		value: Effects {
			font: Font::new(1.27.mm()),
			justify: Justify::new(Default::default(), JustifyVert::Bottom, false),
			hide: false
		}
	}

	sexpr_test_case! {
		name: effects_justify_hide,
		input: "(effects (font (size 1.27 1.27)) (justify bottom) hide)",
		value: Effects {
			font: Font::new(1.27.mm()),
			justify: Justify::new(Default::default(), JustifyVert::Bottom, false),
			hide: true
		}
	}
}
