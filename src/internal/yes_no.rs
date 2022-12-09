use super::tuple;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub(super) enum YesNo {
	Yes,
	No
}

impl From<YesNo> for bool {
	fn from(yn: YesNo) -> Self {
		match yn {
			YesNo::Yes => true,
			YesNo::No => false
		}
	}
}

impl From<bool> for YesNo {
	fn from(b: bool) -> Self {
		match b {
			true => YesNo::Yes,
			false => YesNo::No
		}
	}
}

pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
	D: Deserializer<'de>
{
	tuple::deserialize(deserializer).map(YesNo::into)
}

pub(crate) fn serialize<S>(this: &bool, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer
{
	tuple::serialize(&YesNo::from(*this), serializer)
}
