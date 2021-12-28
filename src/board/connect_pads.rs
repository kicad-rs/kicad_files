use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(try_from = "u8", into = "u8")]
#[repr(u8)]
pub enum ConnectPads {
	/// Pads are not connect to zone.
	NoConnect = 0,

	/// Pads are connected to zone using thermal reliefs.
	ThermalReliefs = 1,

	/// Pads are connected to zone using solid fill.
	SolidFill = 2,

	/// Only through hole pads are connected to zone using thermal reliefs.
	ThroughHoleOnly = 3
}

impl From<ConnectPads> for u8 {
	fn from(c: ConnectPads) -> u8 {
		c as u8
	}
}

#[derive(Clone, Copy, Debug, Error, Eq, PartialEq)]
#[error("invalid value {0}, expected value in the range 0..=3")]
pub struct InvalidValue(u8);

impl TryFrom<u8> for ConnectPads {
	type Error = InvalidValue;

	fn try_from(i: u8) -> Result<Self, Self::Error> {
		Ok(match i {
			0 => Self::NoConnect,
			1 => Self::ThermalReliefs,
			2 => Self::SolidFill,
			3 => Self::ThroughHoleOnly,
			_ => return Err(InvalidValue(i))
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::sexpr_test_case;

	#[derive(Debug, Deserialize, PartialEq, Serialize)]
	#[serde(rename = "zone_connect")]
	struct ZoneConnect(ConnectPads);

	sexpr_test_case! {
		name: no_connect,
		input: "(zone_connect 0)",
		value: ZoneConnect(ConnectPads::NoConnect)
	}

	sexpr_test_case! {
		name: thermal_reliefs,
		input: "(zone_connect 1)",
		value: ZoneConnect(ConnectPads::ThermalReliefs)
	}

	sexpr_test_case! {
		name: solid_fill,
		input: "(zone_connect 2)",
		value: ZoneConnect(ConnectPads::SolidFill)
	}

	sexpr_test_case! {
		name: through_hole_only,
		input: "(zone_connect 3)",
		value: ZoneConnect(ConnectPads::ThroughHoleOnly)
	}
}
