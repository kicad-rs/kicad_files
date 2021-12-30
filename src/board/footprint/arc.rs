use crate::{
	board::Layer,
	common::Point,
	internal::{option_unit, rename, tuple},
	mm
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename = "fp_arc")]
pub struct Arc {
	#[serde(with = "rename::start")]
	pub start: Point,

	#[serde(with = "rename::mid")]
	pub mid: Point,

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
		name: arc,
		input: r#"(fp_arc (start 1 0) (mid 1.5 0.5) (end 2 0) (layer "F.SilkS") (width 0.12) (tstamp "00000000-0000-0000-0000-000000000000"))"#,
		value: Arc {
			start: Point::new(1.0.mm(), 0.0.mm()),
			mid: Point::new(1.5.mm(), 0.5.mm()),
			end: Point::new(2.0.mm(), 0.0.mm()),
			layer: Layer::new("F.SilkS"),
			width: 0.12.mm(),
			locked: false,
			tstamp: Uuid::nil()
		}
	}
}
