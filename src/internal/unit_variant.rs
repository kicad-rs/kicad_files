use serde::{Serialize, Serializer};

pub(crate) struct UnitVariant(
	pub(crate) &'static str,
	pub(crate) u32,
	pub(crate) &'static str
);

impl Serialize for UnitVariant {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer
	{
		serializer.serialize_unit_variant(self.0, self.1, self.2)
	}
}
