use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub(crate) fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
	D: Deserializer<'de>,
	T: Deserialize<'de> + Default
{
	super::option_tuple::deserialize(deserializer).map(Option::unwrap_or_default)
}

pub(crate) fn serialize<S, T>(this: &T, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
	T: Serialize
{
	super::tuple::serialize(this, serializer)
}
