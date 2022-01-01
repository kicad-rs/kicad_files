use crate::{deg, mm, Unit};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename = "xy")]
pub struct Point {
	pub x: mm,

	pub y: mm
}

impl Point {
	pub fn new(x: mm, y: mm) -> Self {
		Self { x, y }
	}

	pub fn rotate_around(self, mid: Point, angle: deg) -> Self {
		let angle = angle.to_radians();
		let sin = angle.sin();
		let cos = angle.cos();

		let dx = self.x - mid.x;
		let dy = self.y - mid.y;
		Self {
			x: mid.x + cos * dx - sin * dy,
			y: mid.y + sin * dx + cos * dy
		}
	}

	#[must_use]
	pub fn round_nm_precision(self) -> Self {
		fn d(v: mm) -> mm {
			let mut d = v % 1.0.nm();
			if d >= 0.5.nm() {
				d -= 1.0.nm();
			} else if d <= -0.5.nm() {
				d += 1.0.nm();
			}
			d
		}

		Self {
			x: self.x - d(self.x),
			y: self.y - d(self.y)
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{sexpr_test_case, Unit};

	sexpr_test_case! {
		name: point,
		input: "(xy 1.27 -2.54)",
		value: Point::new(1.27.mm(), -2.54.mm())
	}
}
