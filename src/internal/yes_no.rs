use super::tuple;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum YesNo {
	Yes,
	No
}

pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
	D: Deserializer<'de>
{
	tuple::deserialize(deserializer).map(|yn: YesNo| match yn {
		YesNo::Yes => true,
		YesNo::No => false
	})
}

pub(crate) fn serialize<S>(this: &bool, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer
{
	tuple::serialize(&this.then(|| YesNo::Yes).unwrap_or(YesNo::No), serializer)
}
