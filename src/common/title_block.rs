use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename = "comment")]
pub struct TitleBlockComment {
	// TODO only supported values are 1..=9
	pub n: u32,
	pub comment: String
}

mod tuple {
	use serde::{Deserializer, Serializer};

	pub(super) fn deserialize<'de, D>(
		deserializer: D
	) -> Result<Option<String>, D::Error>
	where
		D: Deserializer<'de>
	{
		serde_sexpr::Option::deserialize(deserializer)
			.map(|t: Option<(String,)>| t.map(|t| t.0))
	}

	pub(super) fn serialize<S>(
		this: &Option<String>,
		serializer: S
	) -> Result<S::Ok, S::Error>
	where
		S: Serializer
	{
		serde_sexpr::Option::serialize(&this.as_deref().map(|t| (t,)), serializer)
	}
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename = "title_block")]
pub struct TitleBlock {
	#[serde(with = "tuple")]
	pub title: Option<String>,

	#[serde(with = "tuple")]
	pub date: Option<String>,

	#[serde(rename = "rev", with = "tuple")]
	pub revision: Option<String>,

	#[serde(with = "tuple")]
	pub company: Option<String>,

	#[serde(default, rename = "")]
	pub comments: Vec<TitleBlockComment>
}

impl TitleBlock {
	pub const fn new() -> Self {
		Self {
			title: None,
			date: None,
			revision: None,
			company: None,
			comments: Vec::new()
		}
	}

	pub fn with_title<T>(mut self, title: T) -> Self
	where
		T: Into<String>
	{
		self.title = Some(title.into());
		self
	}

	pub fn with_date<T>(mut self, date: T) -> Self
	where
		T: Into<String>
	{
		self.date = Some(date.into());
		self
	}

	pub fn with_revision<T>(mut self, revision: T) -> Self
	where
		T: Into<String>
	{
		self.revision = Some(revision.into());
		self
	}

	pub fn with_company<T>(mut self, company: T) -> Self
	where
		T: Into<String>
	{
		self.company = Some(company.into());
		self
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::sexpr_test_case;

	sexpr_test_case! {
		name: empty,
		input: "(title_block)",
		value: TitleBlock::new()
	}

	sexpr_test_case! {
		name: with_title,
		input: r#"(title_block (title "Cool Title"))"#,
		value: TitleBlock::new().with_title("Cool Title")
	}

	sexpr_test_case! {
		name: with_date,
		input: r#"(title_block (date "2021-12-31"))"#,
		value: TitleBlock::new().with_date("2021-12-31")
	}

	sexpr_test_case! {
		name: with_rev,
		input: r#"(title_block (rev "Rev. 1"))"#,
		value: TitleBlock::new().with_revision("Rev. 1")
	}

	sexpr_test_case! {
		name: with_company,
		input: r#"(title_block (company "Example GmbH"))"#,
		value: TitleBlock::new().with_company("Example GmbH")
	}

	sexpr_test_case! {
		name: example,
		input: r#"(title_block (title "Cool Title") (date "2021-12-31") (rev "Rev. 1") (company "Example GmbH"))"#,
		value: TitleBlock::new()
			.with_title("Cool Title")
			.with_date("2021-12-31")
			.with_revision("Rev. 1")
			.with_company("Example GmbH")
	}

	// TODO add test cases with comments
}
