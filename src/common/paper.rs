use crate::mm;
use serde::{
	de::Error as _,
	ser::{Serialize, SerializeStruct, Serializer},
	Deserialize, Deserializer
};
use std::str::FromStr;
use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PaperSize {
	A0,
	A1,
	A2,
	A3,
	A4,
	A5,
	A,
	B,
	C,
	D,
	E,
	Custom { width: mm, height: mm }
}

impl PaperSize {
	pub const fn landscape(self) -> Paper {
		Paper {
			size: self,
			portrait: false
		}
	}

	pub const fn portrait(self) -> Paper {
		Paper {
			size: self,
			portrait: true
		}
	}
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("Unknown paper size {0}")]
pub struct UnknownPaperSize(String);

impl FromStr for PaperSize {
	type Err = UnknownPaperSize;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(match s {
			"A0" => Self::A0,
			"A1" => Self::A1,
			"A2" => Self::A2,
			"A3" => Self::A3,
			"A4" => Self::A4,
			"A5" => Self::A5,
			"A" => Self::A,
			"B" => Self::B,
			"C" => Self::C,
			"D" => Self::D,
			"E" => Self::E,
			_ => return Err(UnknownPaperSize(s.to_owned()))
		})
	}
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Paper {
	pub size: PaperSize,
	pub portrait: bool
}

#[derive(Deserialize)]
#[serde(rename = "paper")]
struct PaperDef<'a> {
	#[serde(with = "serde_sexpr::Option")]
	width: Option<mm>,

	#[serde(with = "serde_sexpr::Option")]
	height: Option<mm>,

	#[serde(with = "serde_sexpr::Option")]
	size: Option<&'a str>,

	portrait: bool
}

impl<'de> Deserialize<'de> for Paper {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>
	{
		let def = <PaperDef<'de>>::deserialize(deserializer)?;
		let size = match (def.size, def.width, def.height) {
			(Some(size), None, None) => {
				size.parse().map_err(|err| D::Error::custom(err))?
			},
			(None, Some(width), Some(height)) => PaperSize::Custom { width, height },
			_ => return Err(D::Error::custom("invalid paper size"))
		};
		Ok(Paper {
			size,
			portrait: def.portrait
		})
	}
}

struct PaperSizeDef(u32, &'static str);

impl Serialize for PaperSizeDef {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer
	{
		serializer.serialize_unit_variant("PaperSize", self.0, self.1)
	}
}

impl Serialize for Paper {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer
	{
		let size = match self.size {
			PaperSize::A0 => (0, "A0"),
			PaperSize::A1 => (1, "A1"),
			PaperSize::A2 => (2, "A2"),
			PaperSize::A3 => (3, "A3"),
			PaperSize::A4 => (4, "A4"),
			PaperSize::A5 => (5, "A5"),
			PaperSize::A => (6, "A"),
			PaperSize::B => (7, "B"),
			PaperSize::C => (8, "C"),
			PaperSize::D => (9, "D"),
			PaperSize::E => (10, "E"),

			PaperSize::Custom { width, height } => {
				let mut sexpr = serializer.serialize_struct("paper", 3)?;
				sexpr.serialize_field("width", &width)?;
				sexpr.serialize_field("height", &height)?;
				sexpr.serialize_field("portrait", &self.portrait)?;
				return sexpr.end();
			}
		};

		let mut sexpr = serializer.serialize_struct("paper", 2)?;
		sexpr.serialize_field("size", &PaperSizeDef(size.0, size.1))?;
		sexpr.serialize_field("portrait", &self.portrait)?;
		sexpr.end()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{sexpr_test_case, Unit};
	use paste::paste;

	macro_rules! test_cases_predefined {
		($($name:ident)+) => {
			$(
				paste! {
					sexpr_test_case! {
						name: [<paper_ $name:lower>],
						input: concat!("(paper ", stringify!($name), ")"),
						value: PaperSize::$name.landscape()
					}

					sexpr_test_case! {
						name: [<paper_ $name:lower _portrait>],
						input: concat!("(paper ", stringify!($name), " portrait)"),
						value: PaperSize::$name.portrait()
					}
				}
			)+
		}
	}

	test_cases_predefined! {
		A0 A1 A2 A3 A4 A5 A B C D E
	}

	sexpr_test_case! {
		name: paper_custom_size,
		input: "(paper 210 297)",
		value: PaperSize::Custom {
			width: 210.0.mm(),
			height: 297.0.mm()
		}.landscape()
	}

	sexpr_test_case! {
		name: paper_custom_size_portrait,
		input: "(paper 210 297 portrait)",
		value: PaperSize::Custom {
			width: 210.0.mm(),
			height: 297.0.mm()
		}.portrait()
	}
}
