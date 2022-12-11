//! A custom (de)serializer that places the value in a single-element tuple.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub(crate) fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
	D: Deserializer<'de>,
	T: Deserialize<'de>
{
	Deserialize::deserialize(deserializer).map(|t: (T,)| t.0)
}

pub(crate) fn serialize<S, T>(this: &T, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
	T: Serialize
{
	(this,).serialize(serializer)
}
