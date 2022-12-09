use super::{option_tuple, yes_no::YesNo};
use serde::{Deserializer, Serializer};

pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
	D: Deserializer<'de>
{
	option_tuple::deserialize(deserializer).map(|opt| opt.map(YesNo::into))
}

pub(crate) fn serialize<S>(
	this: &Option<bool>,
	serializer: S
) -> Result<S::Ok, S::Error>
where
	S: Serializer
{
	option_tuple::serialize(&this.map(YesNo::from), serializer)
}
