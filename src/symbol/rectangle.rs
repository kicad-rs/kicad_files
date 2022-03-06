use super::{Fill, Stroke};
use crate::{common::Point, internal::rename};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename = "rectangle")]
pub struct Rectangle {
	#[serde(with = "rename::start")]
	pub start: Point,

	#[serde(with = "rename::end")]
	pub end: Point,

	pub stroke: Stroke,

	pub fill: Fill
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{sexpr_test_case, symbol::FillType, Unit};

	sexpr_test_case! {
		name: rectangle,
		input: r#"(rectangle (start -5.08 3.81) (end 5.08 -3.81) (stroke (width 0.254) (type default) (color 0 0 0 0)) (fill (type background)))"#,
		value: Rectangle {
			start: Point::new(-0.2.inch(), 3.81.mm()),
			end: Point::new(0.2.inch(), -3.81.mm()),
			stroke: Stroke::new(0.254.mm(), Default::default()),
			fill: Fill::new(FillType::Background)
		}
	}
}
