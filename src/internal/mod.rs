pub(crate) mod option_tuple;
pub(crate) mod option_unit;
pub(crate) mod option_yes_no;
pub(crate) mod rename;
pub(crate) mod tuple;
pub(crate) mod tuple_or_default;
pub(crate) mod yes_no;

mod u32_hex;
mod unit_variant;

use crate::Color;
use serde::{Deserialize, Serialize};
pub(crate) use u32_hex::u32_hex;
pub(crate) use unit_variant::UnitVariant;

#[derive(Deserialize, Serialize)]
#[serde(remote = "Color", deny_unknown_fields, rename = "color")]
pub(crate) struct ColorDef {
	r: u8,
	g: u8,
	b: u8,
	a: f32
}
