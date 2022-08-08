use std::cmp::Ordering;
use std::fmt::Debug;
use num_traits::{Float, Inv, PrimInt};
use crate::number::Number;
use crate::unit::{check_compatible, Unit, UnitCompatibility};
use crate::impl_ops;
use crate::types::value::Value;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Vec2D<T: Number, U: Unit = ()> {
	pub(crate) value: [T; 2],
	pub(crate) unit: U,
}

impl<T: Number> Vec2D<T> {
	/// Creates a new Vec2D with an unknown unit.
	#[inline(always)]
	pub const fn new_any(x: T, y: T) -> Vec2D<T> {
		Vec2D::new_u(x, y, ())
	}

	pub fn zero() -> Vec2D<T> {
		Vec2D::new_any(T::zero(), T::zero())
	}

	pub fn one() -> Vec2D<T> {
		Vec2D::new_any(T::one(), T::one())
	}
}


impl<T: Number, U: Unit + Default> Vec2D<T, U> {
	/// Creates a new Vec2D with the default unit value.
	#[inline(always)]
	pub fn new(x: T, y: T) -> Vec2D<T, U> {
		Vec2D::new_u(x, y, U::default())
	}
}

impl<N: Number, U: Unit> Vec2D<N, U> {
	/// Creates a new Vec2D with a unit value.
	#[inline(always)]
	pub const fn new_u(x: N, y: N, unit: U) -> Vec2D<N, U> {
		Vec2D {
			value: [x, y],
			unit,
		}
	}


	/// Tries to cast the values to different number type.
	/// # Examples
	/// ```
	///    use mathie::Vec2D;
	///    assert_eq!(Vec2D::<f32>::new_any(250.5, 250.5).cast(), Vec2D::<u32>::new_any(250, 250));
	///    assert_eq!(Vec2D::<f32>::new_any(0.5, 0.5).cast(), Vec2D::<u32>::new_any(0, 0));
	/// ```
	#[inline(always)]
	pub fn cast<NO: Number>(self) -> Vec2D<NO, U> {
		self.try_cast().expect("Failed to cast number")
	}

	/// Same as [Self::cast] but returns None if the cast failed.
	#[inline(always)]
	pub fn try_cast<NO: Number>(self) -> Option<Vec2D<NO, U>> {
		Some(Vec2D {
			value: [
				NO::from(self.value[0])?,
				NO::from(self.value[1])?
			],
			unit: self.unit,
		})
	}

	/// Same as [Self::try_convert] but panics if the conversion failed.
	#[inline(always)]
	pub fn convert<UO: UnitCompatibility<N, U>  + Default>(self) -> Vec2D<N, UO> {
		self.try_convert().expect("Failed to convert units.")
	}

	/// Converts the value to a different unit type by using [UnitCompatibility::convert_pos2].
	/// This is [Self::convert_u] but it uses the target unit default value for easier usage.
	///
	/// # Arguments
	///
	/// returns: Option<Vec2D<N, UO>>
	///
	/// # Examples
	///
	/// ```
	///    use mathie::unit::metric::Centimeter;
	///
	///    let v0 = mathie::Vec2D::<f32, Centimeter>::new(250.0, 250.0);
	///    let v1 = v0.try_convert_u(()).unwrap();
	///    assert_eq!(v1, mathie::Vec2D::new(2.5, 2.5));
	/// ```
	///
	/// # See also
	/// [Self::convert]
	#[inline(always)]
	pub fn try_convert<UO: UnitCompatibility<N, U> + Default>(self) -> Option<Vec2D<N, UO>> {
		self.try_convert_u(UO::default())
	}

	/// Same as [Self::try_convert_u] but panics if the conversion failed.
	#[inline(always)]
	pub fn convert_u<UO: UnitCompatibility<N, U>>(self, to: UO) -> Vec2D<N, UO> {
		self.try_convert_u(to).expect("Failed to convert units.")
	}

	/// Converts the value to a different unit type by using [UnitCompatibility::convert_pos2]
	///
	/// # Arguments
	///
	/// * `to`: The target unit to convert to.
	///
	/// returns: Option<Vec2D<N, UO>>
	///
	/// # Examples
	///
	/// ```
	///    use mathie::unit::metric::Centimeter;
	///
	///    let v0 = mathie::Vec2D::<f32, Centimeter>::new(250.0, 250.0);
	///    let v1 = v0.try_convert_u(()).unwrap();
	///    assert_eq!(v1, mathie::Vec2D::new(2.5, 2.5));
	/// ```
	///
	/// # See also
	/// [Self::convert]
	#[inline(always)]
	pub fn try_convert_u<UO: UnitCompatibility<N, U>>(self, to: UO) -> Option<Vec2D<N, UO>> {
		to.convert_pos2(self)
	}

	/// Clears the unit value and converts this to an un-united vector.
	#[inline(always)]
	pub fn any_unit(self) -> Vec2D<N, ()> {
		Vec2D {
			value: self.value,
			unit: (),
		}
	}

	/// Checks if any of the values match a condition.
	/// # Examples
	///
	/// ```
	/// let v0 = mathie::Vec2D::new_any(2.0, 1.0);
	/// assert!(v0.any(|v| v == 1.0))
	/// ```
	#[inline(always)]
	pub fn any(self, func: impl Fn(N) -> bool) -> bool {
		func(self.value[0]) || func(self.value[1])
	}

	/// Checks if all of the values match a condition.
	/// # Examples
	///
	/// ```
	/// let v0 = mathie::Vec2D::new_any(2.0, 1.0);
	/// assert!(!v0.all(|v| v == 1.0))
	/// ```
	#[inline(always)]
	pub fn all(self, func: impl Fn(N) -> bool) -> bool {
		func(self.value[0]) && func(self.value[1])
	}

	/// Maps both of the values to the function result.
	/// # Examples
	///
	/// ```
	/// let v0 = mathie::Vec2D::new_any(1.0, 1.0);
	/// assert_eq!(v0.map(|v| v + 2.0), mathie::Vec2D::new_any(1.0 + 2.0, 1.0 + 2.0))
	/// ```
	#[inline(always)]
	pub fn map<NO: Number>(self, func: impl Fn(N) -> NO) -> Vec2D<NO, U> {
		Vec2D {
			value: [
				func(self.value[0]),
				func(self.value[1]),
			],
			unit: self.unit,
		}
	}

	/// Maps the X value to the function result.
	/// # Examples
	///
	/// ```
	/// let v0 = mathie::Vec2D::new_any(1.0, 1.0);
	/// assert_eq!(v0.map_x(|v| v + 2.0), mathie::Vec2D::new_any(1.0 + 2.0, 1.0))
	/// ```
	#[inline(always)]
	pub fn map_x(self, func: impl FnOnce(N) -> N) -> Vec2D<N, U> {
		Vec2D {
			value: [
				func(self.value[0]),
				self.value[1]
			],
			unit: self.unit,
		}
	}

	/// Maps the Y value to the function result.
	/// # Examples
	///
	/// ```
	/// let v0 = mathie::Vec2D::new_any(1.0, 1.0);
	/// assert_eq!(v0.map_y(|v| v + 2.0), mathie::Vec2D::new_any(1.0, 1.0 + 2.0))
	/// ```
	#[inline(always)]
	pub fn map_y(self, func: impl FnOnce(N) -> N) -> Vec2D<N, U> {
		Vec2D {
			value: [
				self.value[0],
				func(self.value[1])
			],
			unit: self.unit,
		}
	}

	/// Moves the `x` value from the `other` value and keeps its original `y`
	///
	/// # Arguments
	///
	/// * `other`: The vector to pull x from.
	///
	/// returns: Vec2D<N, U>
	///
	/// # Examples
	///
	/// ```
	/// let v0 = mathie::Vec2D::new_any(1.0, 1.0);
	/// let other = mathie::Vec2D::new_any(0.0, 0.0);
	/// assert_eq!(v0.move_x(other), mathie::Vec2D::new_any(0.0, 1.0))
	/// ```
	#[inline(always)]
	pub fn move_x(self, other: Vec2D<N, U>) -> Vec2D<N, U> {
		check_compatible(&self.unit, &other.unit);
		Vec2D {
			value: [
				other.value[0],
				self.value[1]
			],
			unit: self.unit,
		}
	}

	/// Moves the `y` value from the `other` value and keeps its original `x`
	///
	/// # Arguments
	///
	/// * `other`: The vector to pull y from.
	///
	/// returns: Vec2D<N, U>
	///
	/// # Examples
	///
	/// ```
	/// let v0 = mathie::Vec2D::new_any(1.0, 1.0);
	/// let other = mathie::Vec2D::new_any(0.0, 0.0);
	/// assert_eq!(v0.move_y(other), mathie::Vec2D::new_any(1.0, 0.0))
	/// ```
	#[inline(always)]
	pub fn move_y(self, other: Vec2D<N, U>) -> Vec2D<N, U> {
		check_compatible(&self.unit, &other.unit);
		Vec2D {
			value: [
				self.value[0],
				other.value[1]
			],
			unit: self.unit,
		}
	}
	/// Adds both of the values together.
	/// # Examples
	///
	/// ```
	/// let v0 = mathie::Vec2D::new_any(1.0, 1.0);
	/// assert_eq!(*v0.add_vals(), 2.0)
	/// ```
	#[inline(always)]
	pub fn add_vals(self) -> Value<N, U> {
		Value::new_u(self.value[0] + self.value[1], self.unit)
	}

	/// Subtracts both of the values together.
	/// # Examples
	///
	/// ```
	/// let v0 = mathie::Vec2D::new_any(1.0, 2.0);
	/// assert_eq!(*v0.sub_vals(), -1.0)
	/// ```
	#[inline(always)]
	pub fn sub_vals(self) -> Value<N, U> {
		Value::new_u(self.value[0] - self.value[1], self.unit)
	}

	/// Multiplies both of the values together.
	/// # Examples
	///
	/// ```
	/// let v0 = mathie::Vec2D::new_any(2.0, 2.0);
	/// assert_eq!(*v0.mul_vals(), 4.0)
	/// ```
	#[inline(always)]
	pub fn mul_vals(self) -> Value<N, U> {
		Value::new_u(self.value[0] * self.value[1], self.unit)
	}

	/// Divides both of the values together.
	/// # Examples
	///
	/// ```
	/// let v0 = mathie::Vec2D::new_any(2.0, 4.0);
	/// assert_eq!(*v0.div_vals(), 0.5)
	/// ```
	#[inline(always)]
	pub fn div_vals(self) -> Value<N, U> {
		Value::new_u(self.value[0] / self.value[1], self.unit)
	}

	/// Returns the unit of this vector.
	#[inline(always)]
	pub fn unit(self) -> U {
		self.unit
	}

	/// Returns the X value.
	#[inline(always)]
	pub fn x(self) -> Value<N, U> {
		Value::new_u(self.value[0], self.unit)
	}

	/// Returns the Y value.
	#[inline(always)]
	pub fn y(self) -> Value<N, U> {
		Value::new_u(self.value[1], self.unit)
	}

	/// Does nothing.
	#[inline(always)]
	pub fn xy(self) -> Vec2D<N, U> {
		self
	}

	/// Returns X as Y and Y as X. (Basically inverts the values.)
	#[inline(always)]
	pub fn yx(self) -> Vec2D<N, U> {
		Vec2D {
			value: [
				self.value[1],
				self.value[0],
			],
			unit: self.unit,
		}
	}
}

impl<N: Number + Ord, U: Unit> Vec2D<N, U> {
	/// Gets the smallest value of the Vector.
	/// # Examples
	///
	/// ```
	/// let v0 = mathie::Vec2D::new_any(1, 2);
	/// assert_eq!(*v0.min(), 1)
	/// ```
	#[inline(always)]
	pub fn min(self) -> Value<N, U> {
		Value {
			value: N::min(self.value[0], self.value[1]),
			unit: self.unit,
		}
	}

	/// Gets the biggest value of the Vector.
	/// # Examples
	///
	/// ```
	/// let v0 = mathie::Vec2D::new_any(1, 2);
	/// assert_eq!(*v0.max(), 2)
	/// ```
	#[inline(always)]
	pub fn max(self) -> Value<N, U> {
		Value {
			value: N::max(self.value[0], self.value[1]),
			unit: self.unit,
		}
	}
}

impl<N: Number + Float, U: Unit> Vec2D<N, U> {
	/// Gets the normalized vector from this vector.
	#[inline(always)]
	pub fn normalize(self) -> Vec2D<N, U> {
		let hypot = self.hypot();
		Vec2D {
			value: [
				self.value[0] / *hypot,
				self.value[1] / *hypot,
			],
			unit: self.unit,
		}
	}

	/// The same as [Self::min] but for floating-point numbers.
	#[inline(always)]
	pub fn minf(self) -> Value<N, U> {
		Value {
			value: N::min(self.value[0], self.value[1]),
			unit: self.unit,
		}
	}

	/// The same as [Self::max] but for floating-point numbers.
	#[inline(always)]
	pub fn maxf(self) -> Value<N, U> {
		Value {
			value: N::max(self.value[0], self.value[1]),
			unit: self.unit,
		}
	}

	/// Gets the hypotenuse of the vector. In other terms the length.
	#[inline(always)]
	pub fn hypot(self) -> Value<N, U> {
		Value {
			value: N::hypot(self.value[0], self.value[1]),
			unit: self.unit,
		}
	}
}

impl<N: Number, U: Unit> Into<[N; 2]> for Vec2D<N, U> {
	fn into(self) -> [N; 2] {
		self.value
	}
}

impl<N: Number, U: Unit> Into<(N, N)> for Vec2D<N, U> {
	fn into(self) -> (N, N) {
		(self.value[0], self.value[1])
	}
}

impl<N: Number, U: Unit> PartialEq<Self> for Vec2D<N, U> {
	fn eq(&self, other: &Self) -> bool {
		self.value == other.value
	}
}

impl<N: Number, U: Unit> Eq for Vec2D<N, U> {}

impl<N: Number, U: Unit> PartialOrd<Self> for Vec2D<N, U> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		let x = self.value[0].partial_cmp(&other.value[0])?;
		let y = self.value[1].partial_cmp(&other.value[1])?;
		let ordering = x as i8 + y as i8;
		if ordering > 0 {
			Some(Ordering::Greater)
		} else if ordering < 0 {
			Some(Ordering::Less)
		} else {
			Some(Ordering::Equal)
		}
	}
}

impl<N: Number + Ord, U: Unit> Ord for Vec2D<N, U> {
	fn cmp(&self, other: &Self) -> Ordering {
		let x = self.value[0].cmp(&other.value[0]);
		let y = self.value[1].cmp(&other.value[1]);
		let ordering = x as i8 + y as i8;
		if ordering > 0 {
			Ordering::Greater
		} else if ordering < 0 {
			Ordering::Less
		} else {
			Ordering::Equal
		}
	}
}


macro_rules! impl_op {
    ($TRAIT:ident $TRAIT_ASSIGN:ident $METHOD:ident $METHOD_ASSIGN:ident) => {
        impl<T: Number, U: Unit> $TRAIT for Vec2D<T, U> {
            type Output = Vec2D<T, U>;

            fn $METHOD(self, rhs: Self) -> Self::Output {
                check_compatible(&self.unit, &rhs.unit);
                Vec2D {
                    value: [self.value[0].$METHOD(rhs.value[0]), self.value[1].$METHOD(rhs.value[1])],
                    unit: self.unit,
                }
            }
        }

        impl<T: Number, U: Unit> $TRAIT<T> for Vec2D<T, U> {
            type Output = Vec2D<T, U>;

            fn $METHOD(self, rhs: T) -> Self::Output {
                Vec2D {
                    value: [self.value[0].$METHOD(rhs), self.value[1].$METHOD(rhs)],
                    unit: self.unit,
                }
            }
        }

        impl<T: Number, U: Unit> $TRAIT_ASSIGN<Vec2D<T, U>> for Vec2D<T, U> {
            fn $METHOD_ASSIGN(&mut self, rhs: Self) {
                *self = self.$METHOD(rhs);
            }
        }

	    impl<T: Number, U: Unit> $TRAIT_ASSIGN<T> for Vec2D<T, U> {
            fn $METHOD_ASSIGN(&mut self, rhs: T) {
                *self = self.$METHOD(rhs);
            }
        }
    };
}

impl_ops!(impl_op);

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn add() {
		let v0 = Vec2D::new_any(0.5, 0.5);
		let v1 = Vec2D::new_any(1.0, 1.0);

		assert_eq!(v0 + v1, Vec2D::new_any(1.5, 1.5));
		assert_eq!(v0 + 2.0, Vec2D::new_any(2.5, 2.5));
	}
}