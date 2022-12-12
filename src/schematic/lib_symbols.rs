use crate::symbol::Symbol;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename = "lib_symbols")]
pub struct LibSymbols {
	#[serde(default, rename = "")]
	pub symbols: Vec<Symbol>
}
