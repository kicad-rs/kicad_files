use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FootprintType {
	Smd,
	ThroughHole
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename = "attr")]
pub struct Attributes {
	/// Defines the type of footprint.
	#[serde(with = "serde_sexpr::Option")]
	pub ty: Option<FootprintType>,

	/// Indicates that the footprint is only defined in the board and has no
	/// reference to any schematic symbol.
	pub board_only: bool,

	/// Indicates that the footprint position information should not be included
	/// when creating position files.
	pub exclude_from_pos_files: bool,

	/// indicates that the footprint should be excluded when creating bill of
	/// materials (BOM) files.
	pub exclude_from_bom: bool
}

impl Attributes {
	pub const fn new(ty: FootprintType) -> Self {
		Self {
			ty: Some(ty),
			board_only: false,
			exclude_from_pos_files: false,
			exclude_from_bom: false
		}
	}

	pub const fn new_virtual() -> Self {
		Self {
			ty: None,
			board_only: false,
			exclude_from_pos_files: true,
			exclude_from_bom: true
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::sexpr_test_case;

	sexpr_test_case! {
		name: smd,
		input: "(attr smd)",
		value: Attributes::new(FootprintType::Smd)
	}

	sexpr_test_case! {
		name: through_hole,
		input: "(attr through_hole)",
		value: Attributes::new(FootprintType::ThroughHole)
	}

	sexpr_test_case! {
		name: board_only,
		input: "(attr smd board_only)",
		value: Attributes {
			ty: Some(FootprintType::Smd),
			board_only: true,
			exclude_from_pos_files: false,
			exclude_from_bom: false
		}
	}

	sexpr_test_case! {
		name: exclude_from_pos_list,
		input: "(attr smd exclude_from_pos_files)",
		value: Attributes {
			ty: Some(FootprintType::Smd),
			board_only: false,
			exclude_from_pos_files: true,
			exclude_from_bom: false
		}
	}

	sexpr_test_case! {
		name: exclude_from_bom,
		input: "(attr smd exclude_from_bom)",
		value: Attributes {
			ty: Some(FootprintType::Smd),
			board_only: false,
			exclude_from_pos_files: false,
			exclude_from_bom: true
		}
	}

	sexpr_test_case! {
		name: virtual,
		input: "(attr exclude_from_pos_files exclude_from_bom)",
		value: Attributes::new_virtual()
	}
}
