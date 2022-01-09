use super::FillType;
use crate::{
	board::Layer,
	common::Point,
	internal::{option_tuple, option_unit, rename, tuple, tuple_or_default},
	mm
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename = "fp_rect")]
pub struct Rectangle {
	#[serde(with = "rename::start")]
	pub start: Point,

	#[serde(with = "rename::end")]
	pub end: Point,

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
	use crate::{sexpr_test_case, Unit};

	sexpr_test_case! {
		name: rect,
		input: r#"(fp_rect (start 1 1) (end 2 2) (layer "F.SilkS") (width 0.12) (tstamp "00000000-0000-0000-0000-000000000000"))"#,
		value: Rectangle {
			start: Point::new(1.0.mm(), 1.0.mm()),
			end: Point::new(2.0.mm(), 2.0.mm()),
			layer: Layer::new("F.SilkS"),
			width: 0.12.mm(),
			fill: None,
			locked: false,
			tstamp: Uuid::nil()
		}
	}

	sexpr_test_case! {
		name: rect_filled,
		input: r#"(fp_rect (start 1 1) (end 2 2) (layer "F.SilkS") (width 0.12) (fill solid) (tstamp "00000000-0000-0000-0000-000000000000"))"#,
		value: Rectangle {
			start: Point::new(1.0.mm(), 1.0.mm()),
			end: Point::new(2.0.mm(), 2.0.mm()),
			layer: Layer::new("F.SilkS"),
			width: 0.12.mm(),
			fill: Some(FillType::Solid),
			locked: false,
			tstamp: Uuid::nil()
		}
	}
}
