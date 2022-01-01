use core::{
	cmp::{Eq, Ord, Ordering},
	fmt::{self, Debug, Display, Formatter},
	hash::{Hash, Hasher},
	ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
	str::FromStr
};
use paste::paste;
use serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};
use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum InvalidNumber<T> {
	#[error("non-finite number")]
	NonFinite,

	#[error("invalid value: {value}, expected value from {min} through {max}")]
	InvalidValue { value: T, min: T, max: T }
}

macro_rules! unit {
	(pub const struct $name:ident($min:literal <= $inner:ident <= $max:literal);) => {
		unit!(@internal [const]; $name($min, $inner, $max););
	};

	(pub struct $name:ident($min:literal <= $inner:ident <= $max:literal);) => {
		unit!(@internal []; $name($min, $inner, $max););
	};

	(@internal [$($const:tt)?]; $name:ident($min:literal, $inner:ident, $max:literal);) => {
		paste! {
			mod [<unit_ $name>] {
				use super::InvalidNumber;

				#[derive(Clone, Copy, PartialEq, PartialOrd)]
				#[allow(non_camel_case_types)]
				pub struct $name($inner);

				impl $name {
					pub const MIN: $name = $name($min);
					pub const MAX: $name = $name($max);

					/// This is a helper method for creating this type. You might
					/// want to use the [`Deg`] trait if you don't need a `const`
					/// function.
					///
					///  [`Deg`]: super::Deg
					pub $($const)? fn try_new(inner: $inner) -> Result<Self, InvalidNumber<$inner>> {
						if !inner.is_finite() {
							return Err(InvalidNumber::NonFinite);
						}

						if inner < $min || inner > $max {
							return Err(InvalidNumber::InvalidValue {
								value: inner,
								min: $min,
								max: $max
							});
						}

						Ok(Self(inner))
					}

					/// This is a helper method for creating this type. You might
					/// want to use the [`Deg`] trait if you don't need a `const`
					/// function.
					///
					/// ### Panics
					///
					/// This method panics if the inner value is non-finite or not
					/// in the correct range.
					///
					///  [`Deg`]: super::Unit
					pub $($const)? fn new(inner: $inner) -> Self {
						match Self::try_new(inner) {
							Ok(unit) => unit,
							Err(err) => panic!(
								"{} only supports finite numbers from {} through {}: {}",
								stringify!($name), $min, $max, err
							)
						}
					}

					/// Return the raw value.
					pub const fn raw_value(self) -> $inner {
						self.0
					}
				}
			}

			// Rust is stupid and doesn't notice that this needs to be public
			#[allow(unreachable_pub)]
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

		impl Eq for $name {}

		impl Ord for $name {
			fn cmp(&self, other: &Self) -> Ordering {
				// unwrap: Self will never store non-finite values, so all values
				// should be comparable
				self.partial_cmp(other).unwrap()
			}
		}

		impl Hash for $name {
			fn hash<H: Hasher>(&self, state: &mut H) {
				Hash::hash::<H>(&self.raw_value().to_bits(), state);
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

unit! {
	pub struct deg(-360.0 <= f32 <= 360.0);
}

impl deg {
	pub fn to_radians(self) -> f32 {
		self.raw_value().to_radians()
	}
}

mod private {
	pub trait Sealed {}
	impl Sealed for f32 {}
}

pub trait Deg: private::Sealed {
	fn deg(self) -> deg;
}

impl Deg for f32 {
	fn deg(self) -> deg {
		deg::new(self)
	}
}
