use crate::{
	common::{Position, Size},
	internal::tuple_or_default,
	symbol::Stroke
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename = "bus_entry")]
pub struct BusEntry {
	pub pos: Position,

	pub size: Size,

	pub stroke: Stroke,

	#[serde(with = "tuple_or_default", skip_serializing_if = "crate::skip_uuid")]
	pub uuid: Uuid
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{sexpr_test_case, symbol::StrokeType, Unit};

	sexpr_test_case! {
		name: bus_entry,
		input: r#"(bus_entry (at 25 31) (size 1 2) (stroke (width 0.15) (type default) (color 0 0 0 0)) (uuid "00000000-0000-0000-0000-000000000000"))"#,
		value: BusEntry {
			pos: Position::new(25.0.mm(), 31.0.mm()),
			size: Size::new(1.0.mm(), 2.0.mm()),
			stroke: Stroke::new(0.15.mm(), StrokeType::Default),
			uuid: Uuid::nil()
		}
	}
}
