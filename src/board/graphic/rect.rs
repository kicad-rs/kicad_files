use crate::{
	board::{footprint::FillType, Layer},
	common::Point,
	internal::{option_tuple, rename, tuple, tuple_or_default},
	mm
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename = "gr_rect")]
pub struct Rectangle {
	#[serde(with = "rename::start")]
	pub start: Point,

	#[serde(with = "rename::end")]
	pub end: Point,

	#[serde(with = "serde_sexpr::Option")]
	pub layer: Option<Layer>,

	#[serde(with = "tuple")]
	pub width: mm,

	#[serde(with = "option_tuple")]
	pub fill: Option<FillType>,

	#[serde(with = "tuple_or_default", skip_serializing_if = "crate::skip_uuid")]
	pub tstamp: Uuid
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{sexpr_test_case, Unit};

	sexpr_test_case! {
		name: rect,
		input: r#"(gr_rect (start 1 1) (end 2 2) (width 0.12) (tstamp "00000000-0000-0000-0000-000000000000"))"#,
		value: Rectangle {
			start: Point::new(1.0.mm(), 1.0.mm()),
			end: Point::new(2.0.mm(), 2.0.mm()),
			layer: None,
			width: 0.12.mm(),
			fill: None,
			tstamp: Uuid::nil()
		}
	}

	sexpr_test_case! {
		name: rect_filled,
		input: r#"(gr_rect (start 1 1) (end 2 2) (width 0.12) (fill solid) (tstamp "00000000-0000-0000-0000-000000000000"))"#,
		value: Rectangle {
			start: Point::new(1.0.mm(), 1.0.mm()),
			end: Point::new(2.0.mm(), 2.0.mm()),
			layer: None,
			width: 0.12.mm(),
			fill: Some(FillType::Solid),
			tstamp: Uuid::nil()
		}
	}
}
