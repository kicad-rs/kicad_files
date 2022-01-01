use crate::{
	board::Layer,
	common::Point,
	deg,
	internal::{option_tuple, option_unit, rename, tuple, tuple_or_default},
	mm
};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Deserialize)]
#[serde(deny_unknown_fields, rename = "fp_arc")]
struct ArcDef {
	#[serde(with = "rename::start")]
	start: Point,

	#[serde(with = "rename::option::mid")]
	mid: Option<Point>,

	#[serde(with = "rename::end")]
	end: Point,

	#[serde(with = "option_tuple")]
	angle: Option<deg>,

	layer: Layer,

	#[serde(with = "tuple")]
	width: mm,

	#[serde(with = "option_unit")]
	locked: bool,

	#[serde(with = "tuple_or_default")]
	tstamp: Uuid
}

#[derive(Debug, Error)]
enum InvalidArc {
	#[error("Missing mid or angle definition")]
	MissingMidOrAngle,

	#[error("Conflicting mid and angle definitions")]
	ConflictingMidAndAngle
}

impl TryFrom<ArcDef> for Arc {
	type Error = InvalidArc;

	fn try_from(def: ArcDef) -> Result<Self, InvalidArc> {
		match (def.mid, def.angle) {
			(Some(mid), None) => Ok(Self {
				start: def.start,
				mid,
				end: def.end,
				layer: def.layer,
				width: def.width,
				locked: def.locked,
				tstamp: def.tstamp
			}),

			(None, Some(angle)) => {
				let mid = def.end.rotate_around(def.start, angle / 2.0);
				let start = def.end.rotate_around(def.start, angle);
				Ok(Self {
					start: start.round_nm_precision(),
					mid: mid.round_nm_precision(),
					end: def.end,
					layer: def.layer,
					width: def.width,
					locked: def.locked,
					tstamp: def.tstamp
				})
			},

			(None, None) => Err(InvalidArc::MissingMidOrAngle),
			(Some(_), Some(_)) => Err(InvalidArc::ConflictingMidAndAngle)
		}
	}
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(try_from = "ArcDef", rename = "fp_arc")]
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
		name: arc_with_mid,
		input: r#"(fp_arc (start 8.9 2) (mid -2.403861 0) (end 8.9 -2) (layer "F.SilkS") (width 0.12) (tstamp "00000000-0000-0000-0000-000000000000"))"#,
		value: Arc {
			start: Point::new(8.9.mm(), 2.0.mm()),
			mid: Point::new(-2.403861.mm(), 0.0.mm()),
			end: Point::new(8.9.mm(), -2.0.mm()),
			layer: Layer::new("F.SilkS"),
			width: 0.12.mm(),
			locked: false,
			tstamp: Uuid::nil()
		}
	}

	#[test]
	fn test_deserialize_arc_with_angle() {
		let input = r#"(fp_arc (start 3.425 0) (end 8.9 -2) (angle -319.8658258) (layer "F.SilkS") (width 0.12) (tstamp "00000000-0000-0000-0000-000000000000"))"#;
		let value = Arc {
			start: Point::new(8.9.mm(), 2.0.mm()),
			mid: Point::new(-2.403861.mm(), 0.0.mm()),
			end: Point::new(8.9.mm(), -2.0.mm()),
			layer: Layer::new("F.SilkS"),
			width: 0.12.mm(),
			locked: false,
			tstamp: Uuid::nil()
		};

		let parsed: Arc =
			serde_sexpr::from_str(input).expect("Failed to parse input");
		pretty_assertions::assert_eq!(parsed, value);
	}
}
