use crate::{
	common::{Effects, Position},
	internal::tuple,
	mm
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PinElectricalType {
	/// Pin is an input.
	Input,

	/// Pin is an output.
	Output,

	/// Pin can be both input and output.
	Bidirectional,

	/// Pin is a tri-state output.
	TriState,

	/// Pin is electrically passive.
	Passive,

	/// Not internally connected.
	Free,

	/// Pin does not have a specified electrical type.
	Unspecified,

	/// Pin is a power input.
	PowerIn,

	/// Pin is a power output.
	PowerOut,

	/// Pin is an open collector output.
	OpenCollector,

	/// Pin is an open emitter output.
	OpenEmitter,

	/// Pin has no electrical connection.
	NoConnect
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PinGraphicalStyle {
	/// ![Style](https://dev-docs.kicad.org/en/file-formats/sexpr-intro/images/pinshape_normal_16.png)
	Line,

	/// ![Style](https://dev-docs.kicad.org/en/file-formats/sexpr-intro/images/pinshape_invert_16.png)
	Inverted,

	/// ![Style](https://dev-docs.kicad.org/en/file-formats/sexpr-intro/images/pinshape_clock_normal_16.png)
	Clock,

	/// ![Style](https://dev-docs.kicad.org/en/file-formats/sexpr-intro/images/pinshape_clock_invert_16.png)
	InvertedClock,

	/// ![Style](https://dev-docs.kicad.org/en/file-formats/sexpr-intro/images/pinshape_active_low_input_16.png)
	InputLow,

	/// ![Style](https://dev-docs.kicad.org/en/file-formats/sexpr-intro/images/pinshape_clock_active_low_16.png)
	ClockLow,

	/// ![Style](https://dev-docs.kicad.org/en/file-formats/sexpr-intro/images/pinshape_active_low_output_16.png)
	OutputLow,

	/// ![Style](https://dev-docs.kicad.org/en/file-formats/sexpr-intro/images/pinshape_clock_fall_16.png)
	EdgeClockHigh,

	/// ![Style](https://dev-docs.kicad.org/en/file-formats/sexpr-intro/images/pinshape_nonlogic_16.png)
	NonLogic
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename = "name")]
pub struct PinName {
	pub name: String,
	pub effects: Effects
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename = "number")]
pub struct PinNumber {
	pub number: String,
	pub effects: Effects
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename = "pin")]
pub struct Pin {
	pub electrical_type: PinElectricalType,

	pub graphical_style: PinGraphicalStyle,

	pub at: Position,

	#[serde(with = "tuple")]
	pub length: mm,

	pub hide: bool,

	pub name: PinName,

	pub number: PinNumber
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{common::Font, sexpr_test_case, Unit};

	sexpr_test_case! {
		name: pin,
		input: r#"(pin passive line (at -5.08 0) (length 2.54) (name "~" (effects (font (size 1.27 1.27)))) (number "1" (effects (font (size 1.27 1.27)))))"#,
		value: Pin {
			electrical_type: PinElectricalType::Passive,
			graphical_style: PinGraphicalStyle::Line,
			at: Position::new(-5.08.mm(), 0.0.mm()),
			length: 2.54.mm(),
			hide: false,
			name: PinName {
				name: "~".to_owned(),
				effects: Effects::new(Font::new(1.27.mm()))
			},
			number: PinNumber {
				number: "1".to_owned(),
				effects: Effects::new(Font::new(1.27.mm()))
			}
		}
	}

	sexpr_test_case! {
		name: pin_hidden,
		input: r#"(pin passive line (at -5.08 0) (length 2.54) hide (name "~" (effects (font (size 1.27 1.27)))) (number "1" (effects (font (size 1.27 1.27)))))"#,
		value: Pin {
			electrical_type: PinElectricalType::Passive,
			graphical_style: PinGraphicalStyle::Line,
			at: Position::new(-5.08.mm(), 0.0.mm()),
			length: 2.54.mm(),
			hide: true,
			name: PinName {
				name: "~".to_owned(),
				effects: Effects::new(Font::new(1.27.mm()))
			},
			number: PinNumber {
				number: "1".to_owned(),
				effects: Effects::new(Font::new(1.27.mm()))
			}
		}
	}
}
