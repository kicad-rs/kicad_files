use crate::{
	board::{footprint::arc, Layer},
	common::Point,
	deg,
	internal::{option_tuple, rename, tuple, tuple_or_default},
	mm
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
#[serde(deny_unknown_fields, rename = "gr_arc")]
struct ArcDef {
	#[serde(with = "rename::start")]
	start: Point,

	#[serde(with = "rename::option::mid")]
	mid: Option<Point>,

	#[serde(with = "rename::end")]
	end: Point,

	#[serde(with = "option_tuple")]
	angle: Option<deg>,

	#[serde(with = "serde_sexpr::Option")]
	layer: Option<Layer>,

	#[serde(with = "tuple")]
	width: mm,

	#[serde(with = "tuple_or_default", skip_serializing_if = "crate::skip_uuid")]
	tstamp: Uuid
}

impl TryFrom<ArcDef> for Arc {
	type Error = arc::InvalidArc;

	fn try_from(def: ArcDef) -> Result<Self, Self::Error> {
		match (def.mid, def.angle) {
			(Some(mid), None) => Ok(Self {
				start: def.start,
				mid,
				end: def.end,
				layer: def.layer,
				width: def.width,
				tstamp: def.tstamp
			}),

			(None, Some(angle)) => {
				let (start, mid) =
					arc::start_mid_from_start_end_angle(def.start, def.end, angle);
				Ok(Self {
					start,
					mid,
					end: def.end,
					layer: def.layer,
					width: def.width,
					tstamp: def.tstamp
				})
			},

			(None, None) => Err(arc::InvalidArc::MissingMidOrAngle),
			(Some(_), Some(_)) => Err(arc::InvalidArc::ConflictingMidAndAngle)
		}
	}
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(try_from = "ArcDef", rename = "gr_arc")]
pub struct Arc {
	#[serde(with = "rename::start")]
	pub start: Point,

	#[serde(with = "rename::mid")]
	pub mid: Point,

	#[serde(with = "rename::end")]
	pub end: Point,

	pub layer: Option<Layer>,

	#[serde(with = "tuple")]
	pub width: mm,

	#[serde(with = "tuple", skip_serializing_if = "crate::skip_uuid")]
	pub tstamp: Uuid
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{sexpr_test_case, Unit};

	sexpr_test_case! {
		name: arc_with_mid,
		input: r#"(gr_arc (start 8.9 2) (mid -2.403861 0) (end 8.9 -2) (width 0.12) (tstamp "00000000-0000-0000-0000-000000000000"))"#,
		value: Arc {
			start: Point::new(8.9.mm(), 2.0.mm()),
			mid: Point::new(-2.403861.mm(), 0.0.mm()),
			end: Point::new(8.9.mm(), -2.0.mm()),
			layer: None,
			width: 0.12.mm(),
			tstamp: Uuid::nil()
		}
	}

	#[test]
	fn test_deserialize_arc_with_angle() {
		let input = r#"(gr_arc (start 3.425 0) (end 8.9 -2) (angle -319.8658258) (width 0.12) (tstamp "00000000-0000-0000-0000-000000000000"))"#;
		let value = Arc {
			start: Point::new(8.9.mm(), 2.0.mm()),
			mid: Point::new(-2.403861.mm(), 0.0.mm()),
			end: Point::new(8.9.mm(), -2.0.mm()),
			layer: None,
			width: 0.12.mm(),
			tstamp: Uuid::nil()
		};

		let parsed: Arc =
			serde_sexpr::from_str(input).expect("Failed to parse input");
		pretty_assertions::assert_eq!(parsed, value);
	}
}
