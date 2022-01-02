use super::{
	footprint::{
		Attributes, FillType, Footprint, FootprintContent, FootprintType, Version
	},
	Layer, Timestamp
};
use crate::{common::Position, internal::option_tuple, mm};
use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
enum ModuleAttributes {
	#[serde(rename = "thru_hole")]
	ThroughHole,
	Smd,
	Virtual
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields, rename = "attr")]
struct Attr(ModuleAttributes);

impl Default for Attr {
	fn default() -> Self {
		Self(ModuleAttributes::ThroughHole)
	}
}

impl From<Attr> for Attributes {
	fn from(attr: Attr) -> Self {
		match attr.0 {
			ModuleAttributes::ThroughHole => Self::new(FootprintType::ThroughHole),
			ModuleAttributes::Smd => Self::new(FootprintType::Smd),
			ModuleAttributes::Virtual => Self::new_virtual()
		}
	}
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields, rename = "module")]
pub(super) struct FootprintModule {
	library_link: String,

	layer: Layer,

	#[serde(with = "serde_sexpr::Option")]
	position: Option<Position>,

	#[serde(with = "serde_sexpr::Option")]
	tedit: Option<Timestamp>,

	#[serde(with = "serde_sexpr::Option")]
	attributes_top: Option<Attr>,

	#[serde(rename = "descr", with = "option_tuple")]
	description: Option<String>,

	#[serde(with = "serde_sexpr::Option")]
	attributes_semitop: Option<Attr>,

	#[serde(with = "option_tuple")]
	tags: Option<String>,

	#[serde(with = "serde_sexpr::Option")]
	attributes_mid: Option<Attr>,

	#[serde(with = "option_tuple")]
	solder_mask_margin: Option<mm>,

	#[serde(with = "option_tuple")]
	solder_paste_margin: Option<mm>,

	#[serde(with = "option_tuple")]
	solder_paste_ratio: Option<f32>,

	#[serde(with = "option_tuple")]
	clearance: Option<mm>,

	#[serde(with = "serde_sexpr::Option")]
	attributes_bottom: Option<Attr>,

	#[serde(default, rename = "")]
	content: Vec<FootprintContent>
}

/// **DO NOT USE**
impl Serialize for FootprintModule {
	fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer
	{
		// we only need this to satisfy the serde_sexpr::untagged macro
		// this method will never get called, so we won't bother implementing
		// anything here
		unimplemented!()
	}
}

impl From<FootprintModule> for Footprint {
	fn from(module: FootprintModule) -> Self {
		let mut content = module.content;
		for c in &mut content {
			match c {
				FootprintContent::Text(text) if text.text == "%R" => {
					text.text = "${REFERENCE}".to_owned();
				},
				FootprintContent::Circle(circle) if circle.fill.is_none() => {
					circle.fill = Some(FillType::None);
				},
				_ => {}
			}
		}

		Self {
			library_link: Some(module.library_link),
			version: Some(Version::default()),
			generator: Some("kicad-rs".to_owned()),
			locked: false,
			placed: false,
			layer: module.layer,
			tedit: module.tedit.unwrap_or(Timestamp(0)),
			tstamp: None,
			position: module.position,
			description: module.description,
			tags: module.tags,
			path: None,
			autoplace_cost90: None,
			autoplace_cost180: None,
			solder_mask_margin: module.solder_mask_margin,
			solder_paste_margin: module.solder_paste_margin,
			solder_paste_ratio: module.solder_paste_ratio,
			clearance: module.clearance,
			zone_connect: None,
			thermal_width: None,
			thermal_gap: None,
			attributes: module
				.attributes_mid
				.or(module.attributes_top)
				.or(module.attributes_semitop)
				.or(module.attributes_bottom)
				.unwrap_or_default()
				.into(),
			content
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use pretty_assertions::assert_eq;

	#[test]
	fn empty_lib_footprint() {
		let input = r#"
			(module MountingHole
				(layer F.Cu)
				(tedit DEADBEEF)
				(descr "A mounting hole")
				(tags "mounting hole"))
		"#;

		let expected = FootprintModule {
			library_link: "MountingHole".to_owned(),
			layer: Layer::new("F.Cu"),
			tedit: Some(Timestamp(0xDEADBEEF)),
			position: None,
			attributes_top: None,
			description: Some("A mounting hole".to_owned()),
			attributes_semitop: None,
			tags: Some("mounting hole".to_owned()),
			attributes_mid: None,
			solder_mask_margin: None,
			solder_paste_margin: None,
			solder_paste_ratio: None,
			clearance: None,
			attributes_bottom: None,
			content: Vec::new()
		};

		let parsed: FootprintModule =
			serde_sexpr::from_str(input).expect("Failed to parse input");
		assert_eq!(parsed, expected);
	}

	#[test]
	fn test_upgrade_footprint_module() {
		let input = include_str!("example-old.txt");
		let expected = include_str!("example-new.txt");

		let old: FootprintModule =
			serde_sexpr::from_str(input).expect("Failed to parse input");
		let new: Footprint = old.into();
		let actual =
			serde_sexpr::to_string_pretty(&new).expect("Failed to write output");

		assert_eq!(actual.trim_end(), expected.trim_end());
	}
}
