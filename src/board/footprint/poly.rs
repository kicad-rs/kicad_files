use super::FillType;
use crate::{
	board::Layer,
	common::PointList,
	internal::{option_tuple, option_unit, tuple, tuple_or_default},
	mm
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename = "fp_poly")]
pub struct Polygon {
	pub pts: PointList,

	pub layer: Layer,

	#[serde(with = "tuple")]
	pub width: mm,

	#[serde(with = "option_tuple")]
	pub fill: Option<FillType>,

	#[serde(with = "option_unit")]
	pub locked: bool,

	#[serde(with = "tuple_or_default", skip_serializing_if = "crate::skip_uuid")]
	pub tstamp: Uuid
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{common::Point, sexpr_test_case, Unit};

	sexpr_test_case! {
		name: poly,
		input: r#"(fp_poly (pts (xy 1 1) (xy 1 2) (xy 2 2) (xy 2 1)) (layer "F.SilkS") (width 0.12) (tstamp "00000000-0000-0000-0000-000000000000"))"#,
		value: Polygon {
			pts: PointList::new(vec![
				Point::new(1.0.mm(), 1.0.mm()),
				Point::new(1.0.mm(), 2.0.mm()),
				Point::new(2.0.mm(), 2.0.mm()),
				Point::new(2.0.mm(), 1.0.mm())
			]),
			layer: Layer::new("F.SilkS"),
			width: 0.12.mm(),
			fill: None,
			locked: false,
			tstamp: Uuid::nil()
		}
	}
}
