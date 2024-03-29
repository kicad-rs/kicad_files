use super::{ConnectPads, Primitives};
use crate::{
	common::{Point, Position},
	internal::{option_tuple, option_unit, rename, tuple, tuple_or_default},
	mm
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PadType {
	#[serde(rename = "thru_hole")]
	ThroughHole,
	Smd,
	Connect,
	#[serde(rename = "np_thru_hole")]
	NonPlatedThroughHole
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PadShape {
	Circle,
	Rect,
	Oval,
	Trapezoid,
	RoundRect,
	Custom
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename = "size")]
pub struct PadSize {
	pub width: mm,
	pub height: mm
}

impl PadSize {
	pub fn new(width: mm, height: mm) -> Self {
		Self { width, height }
	}
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename = "rect_delta")]
pub struct RectDelta(mm, mm);

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename = "drill")]
pub struct PadDrill {
	pub oval: bool,

	#[serde(with = "serde_sexpr::Option")]
	pub diameter: Option<mm>,

	#[serde(with = "serde_sexpr::Option")]
	pub width: Option<mm>,

	#[serde(with = "rename::option::offset")]
	pub offset: Option<Point>
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PadEdge {
	TopLeft,
	TopRight,
	BottomLeft,
	BottomRight
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PadClearanceType {
	Outline,
	ConvexHull
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PadAnchor {
	Rect,
	Circle
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename = "options")]
pub struct PadOptions {
	#[serde(with = "tuple")]
	pub clearance: PadClearanceType,

	#[serde(with = "tuple")]
	pub anchor: PadAnchor
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename = "pad")]
pub struct Pad {
	pub number: String,

	pub ty: PadType,

	pub shape: PadShape,

	pub position: Position,

	#[serde(with = "option_unit")]
	pub locked: bool,

	pub size: PadSize,

	#[serde(with = "serde_sexpr::Option")]
	pub rect_delta: Option<RectDelta>,

	#[serde(with = "serde_sexpr::Option")]
	pub drill: Option<PadDrill>,

	pub layers: Vec<String>,

	#[serde(with = "option_unit")]
	pub remove_unused_layer: bool,

	#[serde(with = "option_unit")]
	pub keep_end_layers: bool,

	#[serde(with = "option_tuple")]
	pub roundrect_rratio: Option<f32>,

	#[serde(with = "option_tuple")]
	pub chamfer_ratio: Option<f32>,

	#[serde(with = "serde_sexpr::Option")]
	pub chamfer: Option<Vec<PadEdge>>,

	#[serde(with = "serde_sexpr::Option")]
	pub net: Option<(u32, String)>,

	#[serde(with = "tuple_or_default", skip_serializing_if = "crate::skip_uuid")]
	pub tstamp: Uuid,

	#[serde(with = "option_tuple")]
	pub pinfunction: Option<String>,

	#[serde(with = "option_tuple")]
	pub pintype: Option<String>,

	#[serde(with = "option_tuple")]
	pub die_length: Option<mm>,

	#[serde(with = "option_tuple")]
	pub solder_mask_margin: Option<mm>,

	#[serde(with = "option_tuple")]
	pub solder_paste_margin: Option<mm>,

	#[serde(with = "option_tuple")]
	pub solder_paste_margin_ratio: Option<f32>,

	#[serde(with = "option_tuple")]
	pub clearance: Option<mm>,

	#[serde(with = "option_tuple")]
	pub zone_connect: Option<ConnectPads>,

	#[serde(with = "option_tuple")]
	pub thermal_width: Option<mm>,

	#[serde(with = "option_tuple")]
	pub thermal_gap: Option<mm>,

	#[serde(with = "serde_sexpr::Option")]
	pub custom_pad_options: Option<PadOptions>,

	#[serde(with = "serde_sexpr::Option")]
	pub custom_pad_primitives: Option<Primitives>
}

impl Pad {
	pub fn new<N>(
		number: N,
		ty: PadType,
		shape: PadShape,
		pos: Position,
		size: PadSize,
		layers: Vec<String>,
		tstamp: Uuid
	) -> Self
	where
		N: Into<String>
	{
		Self {
			number: number.into(),
			ty,
			shape,
			position: pos,
			locked: false,
			size,
			rect_delta: None,
			drill: None,
			layers,
			remove_unused_layer: false,
			keep_end_layers: false,
			roundrect_rratio: None,
			chamfer_ratio: None,
			chamfer: None,
			net: None,
			tstamp,
			pinfunction: None,
			pintype: None,
			die_length: None,
			solder_mask_margin: None,
			solder_paste_margin: None,
			solder_paste_margin_ratio: None,
			clearance: None,
			zone_connect: None,
			thermal_width: None,
			thermal_gap: None,
			custom_pad_options: None,
			custom_pad_primitives: None
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{sexpr_test_case, Unit};

	sexpr_test_case! {
		name: pad_simple,
		input: r#"(pad "1" smd rect (at 0 0) (size 2 2) (layers "F.Cu" "F.Paste" "F.Mask") (tstamp "00000000-0000-0000-0000-000000000000"))"#,
		value: Pad::new(
			"1",
			PadType::Smd,
			PadShape::Rect,
			Position::new(0.0.mm(), 0.0.mm()),
			PadSize::new(2.0.mm(), 2.0.mm()),
			vec!["F.Cu".to_owned(), "F.Paste".to_owned(), "F.Mask".to_owned()],
			Uuid::nil()
		)
	}
}
