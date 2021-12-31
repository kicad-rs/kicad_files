use crate::internal::{u32_hex, UnitVariant};
use paste::paste;
use serde::{
	de::{self, Deserializer, Unexpected, Visitor},
	Deserialize, Serialize, Serializer
};
use std::fmt::{self, Formatter};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields, rename = "tedit")]
pub struct Timestamp(#[serde(deserialize_with = "deserialize_timestamp")] pub u32);

impl Timestamp {
	pub fn new(unix_timestamp: u32) -> Self {
		Self(unix_timestamp)
	}
}

struct TimestampVisitor;

macro_rules! visit_try_from {
	($($int:ident)+) => {
		$(
			paste! {
				fn [<visit_ $int>]<E>(self, v: $int) -> Result<Self::Value, E>
				where
					E: de::Error
				{
					u32::try_from(v).map_err(|_| {
						let unexp = format!("{}", v);
						E::invalid_type(Unexpected::Other(&unexp), &self)
					})
				}
			}
		)+
	};
}

impl<'de> Visitor<'de> for TimestampVisitor {
	type Value = u32;

	fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str("a (hex-encoded) unix timestamp")
	}

	fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
	where
		E: de::Error
	{
		u32::from_str_radix(v, 16).map_err(E::custom)
	}

	fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
	where
		E: de::Error
	{
		Ok(v)
	}

	visit_try_from! {
		i8 i16 i32 i64 i128 u8 u16 u64 u128
	}
}

fn deserialize_timestamp<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
	D: Deserializer<'de>
{
	deserializer.deserialize_any(TimestampVisitor)
}

impl Serialize for Timestamp {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer
	{
		serializer.serialize_newtype_struct(
			"tedit",
			&UnitVariant("Timestamp", self.0, u32_hex(self.0))
		)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::sexpr_test_case;

	sexpr_test_case! {
		name: zero,
		input: "(tedit 00000000)",
		value: Timestamp(0)
	}

	sexpr_test_case! {
		name: one,
		input: "(tedit 00000001)",
		value: Timestamp(1)
	}

	sexpr_test_case! {
		name: ten,
		input: "(tedit 0000000A)",
		value: Timestamp(10)
	}

	sexpr_test_case! {
		name: deadbeef,
		input: "(tedit DEADBEEF)",
		value: Timestamp(0xDEADBEEF)
	}
}
