use super::{Fill, Stroke};
use crate::common::PointList;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename = "curve")]
pub struct Curve {
	pub pts: PointList,

	pub stroke: Stroke,

	pub fill: Fill
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{common::Point, sexpr_test_case, symbol::FillType, Unit};

	sexpr_test_case! {
		name: curve,
		input: r#"(curve (pts (xy 1 1) (xy 1 2) (xy 2 2) (xy 2 1)) (stroke (width 0.254) (type default) (color 0 0 0 0)) (fill (type background)))"#,
		value: Curve {
			pts: PointList::new(vec![
				Point::new(1.0.mm(), 1.0.mm()),
				Point::new(1.0.mm(), 2.0.mm()),
				Point::new(2.0.mm(), 2.0.mm()),
				Point::new(2.0.mm(), 1.0.mm())
			]),
			stroke: Stroke::new(0.254.mm(), Default::default()),
			fill: Fill::new(FillType::Background)
		}
	}
}
