use std::cmp::Ordering;
use std::ops::Deref;
use num_traits::Float;
use crate::impl_ops;
use crate::number::Number;
use crate::unit::{check_compatible, Unit, UnitCompatibility};

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Value<N: Number, U: Unit = ()> {
	pub(crate) value: N,
	pub(crate) unit: U,
}

impl<N: Number> Value<N> {
	pub const fn new_any(value: N) -> Value<N> {
		Self::new_u(value, ())
	}
}

impl<N: Number, U: Unit + Default> Value<N, U> {
	pub fn new(value: N) -> Value<N, U> {
		Self::new_u(value, U::default())
	}
}


impl<N: Number + Float, U: Unit> Value<N, U> {

	/// Linearly interpolated the value of `self` and `other` by the value `t` where 0 is self and 1 is other.
	/// # Arguments
	///
	/// * `other`: The target value.
	/// * `t`:  A value which says where the value should be.
	///
	/// returns: Vec2D<N, U>
	///
	/// # Examples
	/// ```
	/// let v0 = mathie::Value::new_any(1.0);
	/// let other = mathie::Value::new_any(2.0);
	/// assert_eq!(v0.lerp(other, 0.0), v0);
	/// assert_eq!(v0.lerp(other, 1.0), other);
	/// assert_eq!(v0.lerp(other, 0.5), mathie::Value::new_any(1.5));
	/// ```
	#[inline(always)]
	pub fn lerp(self, other: Value<N, U>, t: N) -> Value<N, U> {
		check_compatible(&self.unit, &other.unit);
		Value {
			value: ((other.value - self.value) * t) + self.value,
			unit: self.unit,
		}
	}
}


impl<N: Number, U: Unit> Value<N, U> {
	pub const fn new_u(value: N, unit: U) -> Value<N, U> {
		Value {
			value,
			unit,
		}
	}

	#[inline(always)]
	pub fn cast<NO: Number>(self) -> Option<Value<NO, U>> {
		Some(Value {
			value: NO::from(self.value)?,
			unit: self.unit,
		})
	}

	#[inline(always)]
	pub fn convert_def<UO: UnitCompatibility<N, U> + Default>(self) -> Option<Value<N, UO>> {
		self.convert_u(UO::default())
	}

	#[inline(always)]
	pub fn convert_u<UO: UnitCompatibility<N, U>>(self, to: UO) -> Option<Value<N, UO>> {
		to.convert_value(self)
	}

	#[inline(always)]
	pub fn any_unit(self) -> Value<N, ()> {
		Value {
			value: self.value,
			unit: (),
		}
	}

	#[inline(always)]
	pub fn unit(self) -> U {
		self.unit
	}

	#[inline(always)]
	pub fn val(self) -> N {
		self.value
	}
}

impl<T: Number, U: Unit> Deref for Value<T, U> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.value
	}
}

macro_rules! impl_op {
    ($TRAIT:ident $TRAIT_ASSIGN:ident $METHOD:ident $METHOD_ASSIGN:ident) => {
        impl<T: Number, U: Unit> $TRAIT for Value<T, U> {
            type Output = Value<T, U>;

            fn $METHOD(self, rhs: Self) -> Self::Output {
                check_compatible(&self.unit, &rhs.unit);
                Value {
                    value: self.value.$METHOD(rhs.value),
                    unit: self.unit,
                }
            }
        }

        impl<T: Number, U: Unit> $TRAIT<T> for Value<T, U> {
            type Output = Value<T, U>;

            fn $METHOD(self, rhs: T) -> Self::Output {
                Value {
                    value: self.value.$METHOD(rhs),
                    unit: self.unit,
                }
            }
        }

        impl<T: Number, U: Unit> $TRAIT_ASSIGN<Value<T, U>> for Value<T, U> {
            fn $METHOD_ASSIGN(&mut self, rhs: Self) {
                *self = self.$METHOD(rhs);
            }
        }

	    impl<T: Number, U: Unit> $TRAIT_ASSIGN<T> for Value<T, U> {
            fn $METHOD_ASSIGN(&mut self, rhs: T) {
                *self = self.$METHOD(rhs);
            }
        }
    };
}

impl_ops!(impl_op);


impl<N: Number, U: Unit> PartialEq<Self> for Value<N, U> {
	fn eq(&self, other: &Self) -> bool {
		self.value == other.value
	}
}

impl<N: Number, U: Unit> Eq for Value<N, U> {}

impl<N: Number, U: Unit> PartialOrd<Self> for Value<N, U> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		self.value.partial_cmp(&other.value)
	}
}

impl<N: Number + Ord, U: Unit> Ord for Value<N, U> {
	fn cmp(&self, other: &Self) -> Ordering {
		self.value.cmp(&other.value)
	}
}