use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub(crate) fn deserialize<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
	D: Deserializer<'de>,
	T: Deserialize<'de>
{
	serde_sexpr::Option::deserialize(deserializer)
		.map(|t: Option<(T,)>| t.map(|t| t.0))
}

pub(crate) fn serialize<S, T>(
	this: &Option<T>,
	serializer: S
) -> Result<S::Ok, S::Error>
where
	S: Serializer,
	T: Serialize
{
	serde_sexpr::Option::serialize(&this.as_ref().map(|t| (t,)), serializer)
}
