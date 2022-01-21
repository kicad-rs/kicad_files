use super::{footprint_module::FootprintModule, ConnectPads, Layer, Timestamp};
use crate::{common::Position, internal::option_tuple, mm};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub(super) mod arc;
mod attributes;
mod circle;
mod curve;
mod fill_type;
mod line;
mod model;
mod pad;
mod poly;
mod primitives;
mod rect;
mod text;

pub use arc::Arc;
pub use attributes::{Attributes, FootprintType};
pub use circle::Circle;
pub use curve::Curve;
pub use fill_type::FillType;
pub use line::Line;
pub use model::{Model, Xyz};
pub use pad::{
	Pad, PadAnchor, PadClearanceType, PadDrill, PadOptions, PadShape, PadSize,
	PadType
};
pub use poly::Polygon;
pub use primitives::Primitives;
pub use rect::Rectangle;
pub use text::Text;

serde_sexpr::untagged! {
	#[derive(Clone, Debug, PartialEq)]
	pub enum FootprintContent {
		Text(Text),
		Line(Line),
		Rect(Rectangle),
		Circle(Circle),
		Arc(Arc),
		Poly(Polygon),
		Curve(Curve),
		Pad(Pad),
		Model(Model)
	}
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename = "version")]
pub struct Version(u32);

impl Default for Version {
	fn default() -> Self {
		Self(20211014)
	}
}

impl Version {
	pub fn new() -> Self {
		Self::default()
	}
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename = "footprint")]
pub struct Footprint {
	/// Defines the link to footprint library of the footprint. This only applies to
	/// footprints defined in the board file format.
	#[serde(with = "serde_sexpr::Option")]
	pub library_link: Option<String>,

	#[serde(with = "serde_sexpr::Option")]
	pub version: Option<Version>,

	#[serde(with = "option_tuple")]
	pub generator: Option<String>,

	/// Defines a flag to indicate the footprint cannot be edited.
	pub locked: bool,

	/// Defines a flag to indicate that the footprint has not been placed.
	pub placed: bool,

	/// Defines the canonical layer the footprint is placed.
	pub layer: Layer,

	/// Defines a the last time the footprint was edited.
	pub tedit: Timestamp,

	/// Defines the unique identifier for the footprint. This only applies to
	/// footprints defined in the board file format.
	#[serde(with = "option_tuple")]
	pub tstamp: Option<Uuid>,

	/// The X and Y coordinates and rotational angle of the footprint. This only
	/// applies to footprints defined in the board file format.
	#[serde(with = "serde_sexpr::Option")]
	pub position: Option<Position>,

	/// Defines a string containing the description of the footprint.
	#[serde(rename = "descr", with = "option_tuple")]
	pub description: Option<String>,

	/// Defines a string of search tags for the footprint.
	#[serde(with = "option_tuple")]
	pub tags: Option<String>,

	/// Defines the hierarchical path of the schematic symbol linked to the
	/// footprint. This only applies to footprints defined in the board file format.
	#[serde(with = "option_tuple")]
	pub path: Option<String>,

	/// Defines the vertical cost of when using the automatic footprint placement
	/// tool. Valid values are integers 1 through 10. This only applies to
	/// footprints defined in the board file format.
	#[serde(with = "option_tuple")]
	pub autoplace_cost90: Option<u8>,

	/// Defines the horizontal cost of when using the automatic footprint placement
	/// tool. Valid values are integers 1 through 10. This only applies to
	/// footprints defined in the board file format.
	#[serde(with = "option_tuple")]
	pub autoplace_cost180: Option<u8>,

	/// Defines the solder mask distance from all pads in the footprint.
	#[serde(with = "option_tuple")]
	pub solder_mask_margin: Option<mm>,

	/// Defines the solder paste distance from all pads in the footprint.
	#[serde(with = "option_tuple")]
	pub solder_paste_margin: Option<mm>,

	/// Defines the percentage of the pad size used to define the solder paste for
	/// all pads in the footprint.
	#[serde(with = "option_tuple")]
	pub solder_paste_ratio: Option<f32>,

	/// Defines the clearance to all board copper objects for all pads in the
	/// footprint.
	#[serde(with = "option_tuple")]
	pub clearance: Option<mm>,

	/// Defines how all pads are connected to filled zone.
	#[serde(with = "option_tuple")]
	pub zone_connect: Option<ConnectPads>,

	/// Defined the thermal relief spoke width used for zone connections for all
	/// pads in the footprint. This only affects pads connected to zones with
	/// thermal reliefs.
	#[serde(with = "option_tuple")]
	pub thermal_width: Option<mm>,

	/// Defines the distance from the pad to the zone of thermal relief connections
	/// for all pads in the footprint.
	#[serde(with = "option_tuple")]
	pub thermal_gap: Option<mm>,

	/// Defines the attributes of the footprint.
	pub attributes: Attributes,

	#[serde(default, rename = "")]
	pub content: Vec<FootprintContent>
}

serde_sexpr::untagged! {
	enum FootprintOrModule {
		Footprint(Footprint),
		Module(FootprintModule)
	}
}

impl Footprint {
	pub fn from_str(s: &str) -> Result<Self, serde_sexpr::de::Error> {
		let fp: FootprintOrModule = serde_sexpr::from_str(s)?;
		Ok(match fp {
			FootprintOrModule::Footprint(fp) => fp,
			FootprintOrModule::Module(m) => m.into()
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::Unit;

	#[test]
	fn empty_lib_footprint() {
		let input = r#"
			(footprint "MountingHole"
				(version 20211014)
				(generator foobar)
				(layer "F.Cu")
				(tedit DEADBEEF)
				(descr "A mounting hole")
				(tags "mounting hole")
				(attr exclude_from_pos_files exclude_from_bom))
		"#;

		let expected = Footprint {
			library_link: Some("MountingHole".to_owned()),
			version: Some(Version(20211014)),
			generator: Some("foobar".to_owned()),
			locked: false,
			placed: false,
			layer: Layer::new("F.Cu"),
			tedit: Timestamp(0xDEADBEEF),
			tstamp: None,
			position: None,
			description: Some("A mounting hole".to_owned()),
			tags: Some("mounting hole".to_owned()),
			path: None,
			autoplace_cost90: None,
			autoplace_cost180: None,
			solder_mask_margin: None,
			solder_paste_margin: None,
			solder_paste_ratio: None,
			clearance: None,
			zone_connect: None,
			thermal_width: None,
			thermal_gap: None,
			attributes: Attributes::new_virtual(),
			content: Vec::new()
		};

		let parsed: Footprint =
			serde_sexpr::from_str(input).expect("Failed to parse input");
		assert_eq!(parsed, expected);
	}

	#[test]
	fn empty_pcb_footprint() {
		let input = r#"
			(footprint "MountingHole:MountingHole"
				(layer "F.Cu")
				(tedit DEADBEEF)
				(tstamp 931fb3d7-f50a-4517-80c8-bbc40990b0af)
				(at 42 42)
				(descr "A mounting hole")
				(tags "mounting hole")
				(attr exclude_from_pos_files exclude_from_bom))
		"#;

		let expected = Footprint {
			library_link: Some("MountingHole:MountingHole".to_owned()),
			version: None,
			generator: None,
			locked: false,
			placed: false,
			layer: Layer::new("F.Cu"),
			tedit: Timestamp(0xDEADBEEF),
			tstamp: Some("931fb3d7-f50a-4517-80c8-bbc40990b0af".parse().unwrap()),
			position: Some(Position::new(42.0.mm(), 42.0.mm())),
			description: Some("A mounting hole".to_owned()),
			tags: Some("mounting hole".to_owned()),
			path: None,
			autoplace_cost90: None,
			autoplace_cost180: None,
			solder_mask_margin: None,
			solder_paste_margin: None,
			solder_paste_ratio: None,
			clearance: None,
			zone_connect: None,
			thermal_width: None,
			thermal_gap: None,
			attributes: Attributes::new_virtual(),
			content: Vec::new()
		};

		let parsed: Footprint =
			serde_sexpr::from_str(input).expect("Failed to parse input");
		assert_eq!(parsed, expected);
	}
}
