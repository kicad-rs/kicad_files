use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename = "layer")]
pub struct Layer(pub String);

impl Layer {
	pub fn new<T>(layer: T) -> Self
	where
		T: Into<String>
	{
		Self(layer.into())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::sexpr_test_case;

	sexpr_test_case! {
		name: f_cu,
		input: r#"(layer "F.Cu")"#,
		value: Layer::new("F.Cu")
	}
}
