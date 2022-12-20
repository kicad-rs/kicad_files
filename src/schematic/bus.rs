use crate::{common::PointList, internal::tuple_or_default, symbol::Stroke};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename = "bus")]
pub struct Bus {
	pub points: PointList,

	pub stroke: Stroke,

	#[serde(with = "tuple_or_default", skip_serializing_if = "crate::skip_uuid")]
	pub uuid: Uuid
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{common::Point, sexpr_test_case, symbol::StrokeType, Unit};

	sexpr_test_case! {
		name: bus,
		input: r#"(bus (pts (xy 25 31) (xy 25 32)) (stroke (width 0.15) (type default) (color 0 0 0 0)) (uuid "00000000-0000-0000-0000-000000000000"))"#,
		value: Bus {
			points: PointList::new(vec![Point::new(25.0.mm(), 31.0.mm()), Point::new(25.0.mm(), 32.0.mm())]),
			stroke: Stroke::new(0.15.mm(), StrokeType::Default),
			uuid: Uuid::nil()
		}
	}
}
