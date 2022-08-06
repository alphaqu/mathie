use std::cmp::Ordering;
use std::ops::Deref;
use crate::impl_ops;
use crate::number::Number;
use crate::unit::{check_compatible, Unit, UnitCompatibility};

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Value<T: Number, U: Unit> {
	pub(crate) value: T,
	pub(crate) unit: U
}

impl<N: Number, U: Unit> Value<N, U> {
	pub fn new(value: N, unit: U) -> Value<N, U> {
		Value {
			value,
			unit
		}
	}

	#[inline(always)]
	pub fn cast<NO: Number>(self) ->  Option<Value<NO, U>> {
		Some(Value {
			value: NO::from(self.value)?,
			unit: self.unit,
		})
	}

	#[inline(always)]
	pub fn convert_def<UO: UnitCompatibility<N, U> + Default>(self) ->  Option<Value<N, UO>> {
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
			unit: ()
		}
	}

	#[inline(always)]
	pub fn unit(self) -> U  {
		self.unit
	}

	#[inline(always)]
	pub fn val(self) -> N  {
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

impl<N: Number, U: Unit> Eq for Value<N, U> {

}

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