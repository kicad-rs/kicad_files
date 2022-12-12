use crate::{common::Position, internal::tuple_or_default};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename = "no_connect")]
pub struct NoConnect {
	pub pos: Position,

	#[serde(with = "tuple_or_default", skip_serializing_if = "crate::skip_uuid")]
	pub uuid: Uuid
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{sexpr_test_case, Unit};

	sexpr_test_case! {
		name: no_connect,
		input: r#"(no_connect (at 25 31) (uuid "00000000-0000-0000-0000-000000000000"))"#,
		value: NoConnect {
			pos: Position::new(25.0.mm(), 31.0.mm()),
			uuid: Uuid::nil()
		}
	}
}
