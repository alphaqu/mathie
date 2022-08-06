use std::cmp::Ordering;
use std::ops::Deref;
use crate::number::Number;
use crate::unit::Unit;

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