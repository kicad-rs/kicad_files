use super::{Fill, Stroke};
use crate::{
	common::Point,
	internal::{rename, tuple},
	mm
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename = "circle")]
pub struct Circle {
	#[serde(with = "rename::center")]
	pub center: Point,

	#[serde(with = "tuple")]
	pub radius: mm,

	pub stroke: Stroke,

	pub fill: Fill
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{sexpr_test_case, symbol::FillType, Unit};

	sexpr_test_case! {
		name: circle,
		input: r#"(circle (center -2.54 2.54) (radius 2.54) (stroke (width 0.254) (type default) (color 0 0 0 0)) (fill (type background)))"#,
		value: Circle {
			center: Point::new(-0.1.inch(), 0.1.inch()),
			radius: 0.1.inch(),
			stroke: Stroke::new(0.254.mm(), Default::default()),
			fill: Fill::new(FillType::Background)
		}
	}
}
