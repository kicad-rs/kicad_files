use crate::{
	board::Layer,
	common::Point,
	deg,
	internal::{option_tuple, rename, tuple, tuple_or_default},
	mm
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename = "gr_line")]
pub struct Line {
	#[serde(with = "rename::start")]
	pub start: Point,

	#[serde(with = "rename::end")]
	pub end: Point,

	#[serde(with = "option_tuple")]
	pub angle: Option<deg>,

	#[serde(with = "serde_sexpr::Option")]
	pub layer: Option<Layer>,

	#[serde(with = "tuple")]
	pub width: mm,

	#[serde(with = "tuple_or_default", skip_serializing_if = "crate::skip_uuid")]
	pub tstamp: Uuid
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{sexpr_test_case, Deg, Unit};

	sexpr_test_case! {
		name: line,
		input: r#"(gr_line (start 1 0) (end 2 0) (width 0.12) (tstamp "00000000-0000-0000-0000-000000000000"))"#,
		value: Line {
			start: Point::new(1.0.mm(), 0.0.mm()),
			end: Point::new(2.0.mm(), 0.0.mm()),
			angle: None,
			layer: None,
			width: 0.12.mm(),
			tstamp: Uuid::nil()
		}
	}

	sexpr_test_case! {
		name: line_with_angle,
		input: r#"(gr_line (start 1 0) (end 2 0) (angle -90) (width 0.12) (tstamp "00000000-0000-0000-0000-000000000000"))"#,
		value: Line {
			start: Point::new(1.0.mm(), 0.0.mm()),
			end: Point::new(2.0.mm(), 0.0.mm()),
			angle: Some(-90.0.deg()),
			layer: None,
			width: 0.12.mm(),
			tstamp: Uuid::nil()
		}
	}
}
