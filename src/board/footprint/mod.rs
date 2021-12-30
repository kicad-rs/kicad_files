use super::{ConnectPads, Layer, Timestamp};
use crate::{
	common::Position,
	internal::{option_tuple, tuple},
	mm
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod arc;
mod attributes;
mod circle;
mod curve;
mod fill_type;
mod line;
mod poly;
mod rect;
mod text;

pub use arc::Arc;
pub use attributes::{Attributes, FootprintType};
pub use circle::Circle;
pub use curve::Curve;
pub use fill_type::FillType;
pub use line::Line;
pub use poly::Polygon;
pub use rect::Rectangle;
pub use text::Text;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename = "footprint")]
pub struct Footprint {
	/// Defines the link to footprint library of the footprint. This only applies to
	/// footprints defined in the board file format.
	#[serde(with = "serde_sexpr::Option")]
	pub library_link: Option<String>,

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
	#[serde(with = "tuple")]
	pub path: String,

	/// Defines the vertical cost of when using the automatic footprint placement
	/// tool. Valid values are integers 1 through 10. This only applies to
	/// footprints defined in the board file format.
	#[serde(with = "option_tuple")]
	autoplace_cost90: Option<u8>,

	/// Defines the horizontal cost of when using the automatic footprint placement
	/// tool. Valid values are integers 1 through 10. This only applies to
	/// footprints defined in the board file format.
	#[serde(with = "option_tuple")]
	autoplace_cost180: Option<u8>,

	/// Defines the solder mask distance from all pads in the footprint.
	#[serde(with = "option_tuple")]
	solder_mask_margin: Option<mm>,

	/// Defines the solder paste distance from all pads in the footprint.
	#[serde(with = "option_tuple")]
	solder_paste_margin: Option<mm>,

	/// Defines the percentage of the pad size used to define the solder paste for
	/// all pads in the footprint.
	#[serde(with = "option_tuple")]
	solder_paste_ration: Option<f32>,

	/// Defines the clearance to all board copper objects for all pads in the
	/// footprint.
	#[serde(with = "option_tuple")]
	clearance: Option<mm>,

	/// Defines how all pads are connected to filled zone.
	#[serde(with = "option_tuple")]
	zone_connect: Option<ConnectPads>,

	/// Defined the thermal relief spoke width used for zone connections for all
	/// pads in the footprint. This only affects pads connected to zones with
	/// thermal reliefs.
	#[serde(with = "option_tuple")]
	thermal_width: Option<mm>,

	/// Defines the distance from the pad to the zone of thermal relief connections
	/// for all pads in the footprint.
	#[serde(with = "option_tuple")]
	thermal_gap: Option<mm>,

	/// Defines the attributes of the footprint.
	attr: Attributes
}
