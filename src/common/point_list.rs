use super::Point;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename = "pts")]
pub struct PointList {
	#[serde(default, rename = "")]
	pub pts: Vec<Point>
}

impl PointList {
	pub fn new(pts: Vec<Point>) -> Self {
		Self { pts }
	}

	pub fn empty() -> Self {
		Self { pts: Vec::new() }
	}
}

impl Deref for PointList {
	type Target = Vec<Point>;

	fn deref(&self) -> &Self::Target {
		&self.pts
	}
}

impl DerefMut for PointList {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.pts
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{sexpr_test_case, Unit};

	sexpr_test_case! {
		name: empty_list,
		input: "(pts)",
		value: PointList::empty()
	}

	sexpr_test_case! {
		name: list_with_one_point,
		input: "(pts (xy 0 0))",
		value: PointList::new(vec![Point::new(0.0.mm(), 0.0.mm())])
	}

	sexpr_test_case! {
		name: list_with_many_points,
		input: "(pts (xy 0 0) (xy 0 2.54) (xy 2.54 2.54) (xy 2.54 0))",
		value: PointList::new(vec![
			Point::new(0.0.mm(), 0.0.mm()),
			Point::new(0.0.mm(), 2.54.mm()),
			Point::new(2.54.mm(), 2.54.mm()),
			Point::new(2.54.mm(), 0.0.mm())
		])
	}
}
