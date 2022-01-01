use super::{
	footprint::{
		Attributes, FillType, Footprint, FootprintContent, FootprintType, Version
	},
	Layer, Timestamp
};
use crate::{common::Position, internal::option_tuple, mm};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields, rename = "module")]
pub(super) struct FootprintModule {
	library_link: String,

	layer: Layer,

	#[serde(with = "serde_sexpr::Option")]
	position: Option<Position>,

	tedit: Timestamp,

	#[serde(rename = "descr", with = "option_tuple")]
	description: Option<String>,

	#[serde(with = "option_tuple")]
	tags: Option<String>,

	#[serde(with = "serde_sexpr::Option")]
	attributes: Option<Attributes>,

	#[serde(with = "option_tuple")]
	solder_mask_margin: Option<mm>,

	#[serde(with = "option_tuple")]
	solder_paste_margin: Option<mm>,

	#[serde(with = "option_tuple")]
	solder_paste_ratio: Option<f32>,

	#[serde(with = "option_tuple")]
	clearance: Option<mm>,

	#[serde(default, rename = "")]
	content: Vec<FootprintContent>
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
			tedit: module.tedit,
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
				.attributes
				.unwrap_or_else(|| Attributes::new(FootprintType::ThroughHole)),
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
			tedit: Timestamp(0xDEADBEEF),
			position: None,
			description: Some("A mounting hole".to_owned()),
			tags: Some("mounting hole".to_owned()),
			solder_mask_margin: None,
			solder_paste_margin: None,
			solder_paste_ratio: None,
			clearance: None,
			attributes: None,
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
