use serde::{
	de::{Deserializer, Visitor},
	ser::{Serialize, Serializer}
};

struct RenamingDeserializer<D> {
	de: D,
	rename: &'static str
}

macro_rules! forward_deserializer {
	($name:ident; $(fn $ident:ident <$visitor:ident>(
		self $(, $arg:ident : $arg_ty:ty)*
	);)+) => {
		$(
			#[allow(unused_variables)]
			fn $ident<$visitor>(
				self $(, $arg: $arg_ty)*
			) -> Result<$visitor::Value, Self::Error>
			where
				$visitor: Visitor<'de>
			{
				let $name = self.rename;
				self.de.$ident($($arg,)*)
			}
		)+
	};
}

impl<'de, D> Deserializer<'de> for RenamingDeserializer<D>
where
	D: Deserializer<'de>
{
	type Error = D::Error;

	forward_deserializer! {
		name;
		fn deserialize_any<V>(self, visitor: V);
		fn deserialize_bool<V>(self, visitor: V);
		fn deserialize_i8<V>(self, visitor: V);
		fn deserialize_i16<V>(self, visitor: V);
		fn deserialize_i32<V>(self, visitor: V);
		fn deserialize_i64<V>(self, visitor: V);
		fn deserialize_i128<V>(self, visitor: V);
		fn deserialize_u8<V>(self, visitor: V);
		fn deserialize_u16<V>(self, visitor: V);
		fn deserialize_u32<V>(self, visitor: V);
		fn deserialize_u64<V>(self, visitor: V);
		fn deserialize_u128<V>(self, visitor: V);
		fn deserialize_f32<V>(self, visitor: V);
		fn deserialize_f64<V>(self, visitor: V);
		fn deserialize_char<V>(self, visitor: V);
		fn deserialize_str<V>(self, visitor: V);
		fn deserialize_string<V>(self, visitor: V);
		fn deserialize_bytes<V>(self, visitor: V);
		fn deserialize_byte_buf<V>(self, visitor: V);
		fn deserialize_option<V>(self, visitor: V);
		fn deserialize_unit<V>(self, visitor: V);
		fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V);
		fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V);
		fn deserialize_seq<V>(self, visitor: V);
		fn deserialize_tuple<V>(self, len: usize, visitor: V);
		fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V);
		fn deserialize_map<V>(self, visitor: V);
		fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V);
		fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V);
		fn deserialize_identifier<V>(self, visitor: V);
		fn deserialize_ignored_any<V>(self, visitor: V);
	}

	fn is_human_readable(&self) -> bool {
		self.de.is_human_readable()
	}
}

struct RenamingSerializer<S> {
	ser: S,
	rename: &'static str
}

macro_rules! forward_serializer {
	($(type $ident:ident;)+) => {
		$(type $ident = S::$ident;)+
	};

	($name:ident; $(fn $ident:ident $(<$t:ident>)?(
		self $(, $arg:ident : $arg_ty:ty)*
	) -> $ret:ty;)+) => {
		$(
			#[allow(unused_variables)]
			fn $ident$(<$t>)?(
				self $(, $arg: $arg_ty)*
			) -> Result<$ret, Self::Error>
			$(where $t: ?Sized + Serialize)?
			{
				let $name = self.rename;
				self.ser.$ident($($arg,)*)
			}
		)+
	};
}

impl<S> Serializer for RenamingSerializer<S>
where
	S: Serializer
{
	forward_serializer! {
		type Ok;
		type Error;
		type SerializeSeq;
		type SerializeTuple;
		type SerializeTupleStruct;
		type SerializeTupleVariant;
		type SerializeMap;
		type SerializeStruct;
		type SerializeStructVariant;
	}

	forward_serializer! {
		name;
		fn serialize_bool(self, v: bool) -> Self::Ok;
		fn serialize_i8(self, v: i8) -> Self::Ok;
		fn serialize_i16(self, v: i16) -> Self::Ok;
		fn serialize_i32(self, v: i32) -> Self::Ok;
		fn serialize_i64(self, v: i64) -> Self::Ok;
		fn serialize_i128(self, v: i128) -> Self::Ok;
		fn serialize_u8(self, v: u8) -> Self::Ok;
		fn serialize_u16(self, v: u16) -> Self::Ok;
		fn serialize_u32(self, v: u32) -> Self::Ok;
		fn serialize_u64(self, v: u64) -> Self::Ok;
		fn serialize_u128(self, v: u128) -> Self::Ok;
		fn serialize_f32(self, v: f32) -> Self::Ok;
		fn serialize_f64(self, v: f64) -> Self::Ok;
		fn serialize_char(self, v: char) -> Self::Ok;
		fn serialize_str(self, v: &str) -> Self::Ok;
		fn serialize_bytes(self, v: &[u8]) -> Self::Ok;
		fn serialize_none(self) -> Self::Ok;
		fn serialize_some<T>(self, v: &T) -> Self::Ok;
		fn serialize_unit(self) -> Self::Ok;
		fn serialize_unit_struct(self, name: &'static str) -> Self::Ok;
		fn serialize_unit_variant(self, name: &'static str, idx: u32, var: &'static str) -> Self::Ok;
		fn serialize_newtype_struct<T>(self, name: &'static str, v: &T) -> Self::Ok;
		fn serialize_newtype_variant<T>(self, name: &'static str, idx: u32, var: &'static str, v: &T) -> Self::Ok;
		fn serialize_seq(self, len: Option<usize>) -> Self::SerializeSeq;
		fn serialize_tuple(self, len: usize) -> Self::SerializeTuple;
		fn serialize_tuple_struct(self, name: &'static str, len: usize) -> Self::SerializeTupleStruct;
		fn serialize_tuple_variant(self, name: &'static str, idx: u32, var: &'static str, len: usize) -> Self::SerializeTupleVariant;
		fn serialize_map(self, len: Option<usize>) -> Self::SerializeMap;
		fn serialize_struct(self, name: &'static str, len: usize) -> Self::SerializeStruct;
		fn serialize_struct_variant(self, name: &'static str, idx: u32, var: &'static str, len: usize) -> Self::SerializeStructVariant;
	}
}

macro_rules! rename {
	($($name:ident)+) => {
		$(
			pub(crate) mod $name {
				use super::{RenamingDeserializer, RenamingSerializer};
				use serde::{Deserialize, Deserializer, Serialize, Serializer};

				pub(crate) fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
				where
					D: Deserializer<'de>,
					T: Deserialize<'de>
				{
					T::deserialize(RenamingDeserializer {
						de: deserializer,
						rename: stringify!($name)
					})
				}

				pub(crate) fn serialize<S, T>(this: &T, serializer: S) -> Result<S::Ok, S::Error>
				where
					S: Serializer,
					T: Serialize
				{
					this.serialize(RenamingSerializer {
						ser: serializer,
						rename: stringify!($name)
					})
				}
			}
		)+
	};
}

rename! {
	start end
}
