//! Custom (de)serializer that transforms the boolean into an Option<()>.

use serde::{Deserializer, Serializer};

pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
	D: Deserializer<'de>
{
	serde_sexpr::Option::deserialize(deserializer).map(|t: Option<()>| t.is_some())
}

pub(crate) fn serialize<S>(this: &bool, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer
{
	serde_sexpr::Option::serialize(&this.then(|| Some(())), serializer)
}
