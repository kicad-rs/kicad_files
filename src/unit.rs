use paste::paste;
use serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};
use std::{
	fmt::{self, Debug, Display, Formatter},
	ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
	str::FromStr
};
use thiserror::Error;

#[derive(Clone, Copy, Debug, Error)]
#[error("non-finite number")]
pub struct NonFinite;

macro_rules! unit {
	(pub struct $name:ident($inner:ident);$($impl:tt)*) => {
		paste! {
			mod [<unit_ $name>] {
				use super::NonFinite;

				#[derive(Clone, Copy, PartialEq, PartialOrd)]
				#[allow(non_camel_case_types)]
				pub struct $name($inner);

				impl $name {
					/// This is a helper method for creating this type. You might
					/// want to use the [`Unit`] trait if you don't need a `const`
					/// function.
					pub const fn try_new(inner: $inner) -> Result<Self, NonFinite> {
						if !inner.is_finite() {
							return Err(NonFinite);
						}
						Ok(Self(inner))
					}

					/// This is a helper method for creating this type. You might
					/// want to use the [`Unit`] trait if you don't need a `const`
					/// function.
					///
					/// ### Panics
					///
					/// This method panics if the inner value is non-finite
					pub const fn new(inner: $inner) -> Self {
						match Self::try_new(inner) {
							Ok(unit) => unit,
							Err(NonFinite) => panic!(concat!(
								stringify!($name),
								" only supports finite numbers"
							))
						}
					}

					/// Return the raw value.
					pub const fn raw_value(self) -> $inner {
						self.0
					}
				}
			}
			pub use [<unit_ $name>]::$name;
		}

		impl $name {
			/// Returns the nearest integer to a number. Round half-way cases away
			/// from `0.0`.
			#[must_use = "method returns a new number and does not mutate the original value"]
			pub fn round(self) -> Self {
				Self::new(self.raw_value().round())
			}
		}

		$($impl)*

		impl std::cmp::Eq for $name {}

		impl std::cmp::Ord for $name {
			fn cmp(&self, other: &Self) -> std::cmp::Ordering {
				// unwrap: Self will never store non-finite values, so all values
				// should be comparable
				self.partial_cmp(other).unwrap()
			}
		}

		impl std::hash::Hash for $name {
			fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
				std::hash::Hash::hash::<H>(&self.raw_value().to_bits(), state);
			}
		}

		impl Neg for $name {
			type Output = Self;
			fn neg(self) -> Self {
				Self::new(-self.raw_value())
			}
		}

		impl Add for $name {
			type Output = Self;
			fn add(self, rhs: Self) -> Self {
				Self::new(self.raw_value() + rhs.raw_value())
			}
		}

		impl AddAssign for $name {
			fn add_assign(&mut self, rhs: Self) {
				*self = *self + rhs;
			}
		}

		impl Sub for $name {
			type Output = Self;
			fn sub(self, rhs: Self) -> Self {
				Self::new(self.raw_value() - rhs.raw_value())
			}
		}


		impl SubAssign for $name {
			fn sub_assign(&mut self, rhs: Self) {
				*self = *self - rhs;
			}
		}

		impl Mul<$inner> for $name {
			type Output = Self;
			fn mul(self, rhs: $inner) -> Self {
				Self::new(self.raw_value() * rhs)
			}
		}

		impl Mul<$name> for $inner {
			type Output = $name;
			fn mul(self, rhs: $name) -> $name {
				$name::new(self * rhs.raw_value())
			}
		}

		impl MulAssign<$inner> for $name {
			fn mul_assign(&mut self, rhs: $inner) {
				*self = *self * rhs;
			}
		}

		impl Div<$inner> for $name {
			type Output = Self;
			fn div(self, rhs: $inner) -> Self {
				if !rhs.is_finite() {
					panic!("Division through non-finite number");
				}
				if rhs == 0.0 {
					panic!("Division through zero");
				}
				Self::new(self.raw_value() / rhs)
			}
		}

		impl DivAssign<$inner> for $name {
			fn div_assign(&mut self, rhs: $inner) {
				*self = *self / rhs;
			}
		}

		impl FromStr for $name {
			type Err = <$inner as FromStr>::Err;
			fn from_str(s: &str) -> Result<Self, Self::Err> {
				Ok(Self::new(s.parse()?))
			}
		}

		impl Display for $name {
			fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
				Display::fmt(&self.raw_value(), f)
			}
		}

		impl Debug for $name {
			fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
				Debug::fmt(&self.raw_value(), f)?;
				f.write_str(stringify!($name))
			}
		}

		impl<'de> Deserialize<'de> for $name {
			fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
			where
				D: Deserializer<'de>
			{
				$inner::deserialize(deserializer).and_then(|inner| {
					Self::try_new(inner).map_err(|err| D::Error::custom(err))
				})
			}
		}

		impl Serialize for $name {
			fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
			where
				S: Serializer
			{
				self.raw_value().serialize(serializer)
			}
		}
	};
}

macro_rules! square_unit {
	(pub struct $name:ident($inner:ident) = $base:ident^2;$($impl:tt)*) => {
		unit! {
			pub struct $name($inner);
			$($impl)*
		}

		impl $name {
			pub fn sqrt(self) -> $base {
				$base::new(self.raw_value().sqrt())
			}
		}

		impl Mul<$base> for $base {
			type Output = $name;
			fn mul(self, rhs: $base) -> $name {
				$name::new(self.raw_value() * rhs.raw_value())
			}
		}

		impl Div<$base> for $name {
			type Output = $base;
			fn div(self, rhs: $base) -> $base {
				$base::new(self.raw_value() / rhs.raw_value())
			}
		}
	};
}

const MM_PER_INCH: f32 = 25.4;
const MM2_PER_INCH2: f32 = 645.16;

unit! {
	pub struct mm(f32);

	impl From<inch> for mm {
		fn from(inch: inch) -> Self {
			Self::new(inch.raw_value() * MM_PER_INCH)
		}
	}
}

square_unit! {
	pub struct mm2(f32) = mm^2;

	impl From<inch2> for mm2 {
		fn from(inch2: inch2) -> Self {
			Self::new(inch2.raw_value() * MM2_PER_INCH2)
		}
	}
}

unit! {
	pub struct inch(f32);

	impl From<mm> for inch {
		fn from(mm: mm) -> Self {
			Self::new(mm.raw_value() / MM_PER_INCH)
		}
	}
}

square_unit! {
	pub struct inch2(f32) = inch^2;

	impl From<mm2> for inch2 {
		fn from(mm2: mm2) -> Self {
			Self::new(mm2.raw_value() / MM2_PER_INCH2)
		}
	}
}

macro_rules! unit_trait {
	(pub trait $ident:ident {
		$(fn $name:ident(self) -> $unit:ident;)*
	}) => {
		paste! {
			mod [<private_ $ident:lower>] {
				pub trait Sealed {}
				impl Sealed for f32 {}
			}

			pub trait $ident: [<private_ $ident:lower>]::Sealed {
				$(fn $name(self) -> $unit;)*
			}

			impl $ident for f32 {
				$(fn $name(self) -> $unit {
					$unit::new(self)
				})*
			}
		}
	};
}

unit_trait! {
	pub trait Unit {
		fn mm(self) -> mm;
		fn mm2(self) -> mm2;
		fn inch(self) -> inch;
		fn inch2(self) -> inch2;
	}
}
