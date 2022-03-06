use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename = "pin_numbers")]
struct PinNumbers {
	hide: bool
}

pub(super) fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
	D: Deserializer<'de>
{
	serde_sexpr::Option::deserialize(deserializer).map(|v| {
		v.map(|pin_numbers: PinNumbers| pin_numbers.hide)
			.unwrap_or(false)
	})
}

pub(super) fn serialize<S>(this: &bool, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer
{
	this.then(|| PinNumbers { hide: true })
		.serialize(serializer)
}
