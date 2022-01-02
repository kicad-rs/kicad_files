use crate::{
	board::{footprint::FillType, Layer},
	common::PointList,
	internal::{option_tuple, tuple, tuple_or_default},
	mm
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename = "gr_poly")]
pub struct Polygon {
	pub pts: PointList,

	#[serde(with = "serde_sexpr::Option")]
	pub layer: Option<Layer>,

	#[serde(with = "tuple")]
	pub width: mm,

	#[serde(with = "option_tuple")]
	pub fill: Option<FillType>,

	#[serde(with = "tuple_or_default")]
	pub tstamp: Uuid
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{common::Point, sexpr_test_case, Unit};

	sexpr_test_case! {
		name: poly,
		input: r#"(gr_poly (pts (xy 1 1) (xy 1 2) (xy 2 2) (xy 2 1)) (width 0.12) (tstamp "00000000-0000-0000-0000-000000000000"))"#,
		value: Polygon {
			pts: PointList::new(vec![
				Point::new(1.0.mm(), 1.0.mm()),
				Point::new(1.0.mm(), 2.0.mm()),
				Point::new(2.0.mm(), 2.0.mm()),
				Point::new(2.0.mm(), 1.0.mm())
			]),
			layer: None,
			width: 0.12.mm(),
			fill: None,
			tstamp: Uuid::nil()
		}
	}
}
