use crate::{
	board::Layer,
	common::Point,
	internal::{option_unit, rename, tuple},
	mm
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename = "fp_line")]
pub struct Line {
	#[serde(with = "rename::start")]
	pub start: Point,

	#[serde(with = "rename::end")]
	pub end: Point,

	pub layer: Layer,

	#[serde(with = "tuple")]
	pub width: mm,

	#[serde(with = "option_unit")]
	pub locked: bool,

	#[serde(with = "tuple")]
	pub tstamp: Uuid
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{sexpr_test_case, Unit};

	sexpr_test_case! {
		name: line,
		input: r#"(fp_line (start 1 0) (end 2 0) (layer "F.SilkS") (width 0.12) (tstamp "00000000-0000-0000-0000-000000000000"))"#,
		value: Line {
			start: Point::new(1.0.mm(), 0.0.mm()),
			end: Point::new(2.0.mm(), 0.0.mm()),
			layer: Layer::new("F.SilkS"),
			width: 0.12.mm(),
			locked: false,
			tstamp: Uuid::nil()
		}
	}

	sexpr_test_case! {
		name: line_locked,
		input: r#"(fp_line (start 1 0) (end 2 0) (layer "F.SilkS") (width 0.12) (locked) (tstamp "00000000-0000-0000-0000-000000000000"))"#,
		value: Line {
			start: Point::new(1.0.mm(), 0.0.mm()),
			end: Point::new(2.0.mm(), 0.0.mm()),
			layer: Layer::new("F.SilkS"),
			width: 0.12.mm(),
			locked: true,
			tstamp: Uuid::nil()
		}
	}
}
