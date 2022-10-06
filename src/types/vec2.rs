use std::cmp::Ordering;
use std::fmt::Debug;
use std::ops::Neg;
use num_traits::{Float};
use crate::number::Number;
use crate::impl_ops;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vec2<T: Number>(pub(crate) [T; 2]);

impl<N: Number> Vec2<N> {
	/// Creates a new Vec2D with a unit value.
	#[inline(always)]
	pub const fn new(x: N, y: N) -> Vec2<N> {
		Vec2([x, y])
	}

	#[inline(always)]
	pub const fn split(v: N) -> Vec2<N> {
		Vec2([v, v])
	}

	#[inline(always)]
	pub fn zero() -> Vec2<N> {
		Vec2::new(N::zero(), N::zero())
	}

	#[inline(always)]
	pub fn one() -> Vec2<N> {
		Vec2::new(N::one(), N::one())
	}

	/// Tries to cast the values to different number type.
	/// # Examples
	/// ```
	///    use mathie::Vec2;
	///    assert_eq!(Vec2::<f32>::new(250.5, 250.5).cast(), Vec2::<u32>::new(250, 250));
	///    assert_eq!(Vec2::<f32>::new(0.5, 0.5).cast(), Vec2::<u32>::new(0, 0));
	/// ```
	#[inline(always)]
	pub fn cast<NO: Number>(self) -> Vec2<NO> {
		self.try_cast().expect("Failed to cast number")
	}

	/// Same as [Self::cast] but returns None if the cast failed.
	#[inline(always)]
	pub fn try_cast<NO: Number>(self) -> Option<Vec2<NO>> {
		Some(Vec2([
			NO::from(self.x())?,
			NO::from(self.y())?
		]))
	}

	/// Checks if any of the values match a condition.
	/// # Examples
	///
	/// ```
	/// let v0 = mathie::Vec2::new(2.0, 1.0);
	/// assert!(v0.any(|v| v == 1.0))
	/// ```
	#[inline(always)]
	pub fn any(self, func: impl Fn(N) -> bool) -> bool {
		func(self.x()) || func(self.y())
	}

	/// Checks if all of the values match a condition.
	/// # Examples
	///
	/// ```
	/// let v0 = mathie::Vec2::new(2.0, 1.0);
	/// assert!(!v0.all(|v| v == 1.0))
	/// ```
	#[inline(always)]
	pub fn all(self, func: impl Fn(N) -> bool) -> bool {
		func(self.x()) && func(self.y())
	}

	/// Maps both of the values to the function result.
	/// # Examples
	///
	/// ```
	/// let v0 = mathie::Vec2::new(1.0, 1.0);
	/// assert_eq!(v0.map(|v| v + 2.0), mathie::Vec2::new(1.0 + 2.0, 1.0 + 2.0))
	/// ```
	#[inline(always)]
	pub fn map<NO: Number>(self, func: impl Fn(N) -> NO) -> Vec2<NO> {
		Vec2([
			func(self.x()),
			func(self.y()),
		])
	}

	/// Maps the X value to the function result.
	/// # Examples
	///
	/// ```
	/// let v0 = mathie::Vec2::new(1.0, 1.0);
	/// assert_eq!(v0.map_x(|v| v + 2.0), mathie::Vec2::new(1.0 + 2.0, 1.0))
	/// ```
	#[inline(always)]
	pub fn map_x(self, func: impl FnOnce(N) -> N) -> Vec2<N> {
		Vec2([
			func(self.x()),
			self.y()
		])
	}

	/// Maps the Y value to the function result.
	/// # Examples
	///
	/// ```
	/// let v0 = mathie::Vec2::new(1.0, 1.0);
	/// assert_eq!(v0.map_y(|v| v + 2.0), mathie::Vec2::new(1.0, 1.0 + 2.0))
	/// ```
	#[inline(always)]
	pub fn map_y(self, func: impl FnOnce(N) -> N) -> Vec2<N> {
		Vec2([
			self.x(),
			func(self.y())
		])
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
	/// let v0 = mathie::Vec2::new(1.0, 1.0);
	/// let other = mathie::Vec2::new(0.0, 0.0);
	/// assert_eq!(v0.move_x(other), mathie::Vec2::new(0.0, 1.0))
	/// ```
	#[inline(always)]
	pub fn move_x(self, other: Vec2<N>) -> Vec2<N> {
		Vec2([
			other.x(),
			self.y()
		])
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
	/// let v0 = mathie::Vec2::new(1.0, 1.0);
	/// let other = mathie::Vec2::new(0.0, 0.0);
	/// assert_eq!(v0.move_y(other), mathie::Vec2::new(1.0, 0.0))
	/// ```
	#[inline(always)]
	pub fn move_y(self, other: Vec2<N>) -> Vec2<N> {
		Vec2([
			self.x(),
			other.y()
		])
	}
	/// Adds both of the values together.
	/// # Examples
	///
	/// ```
	/// let v0 = mathie::Vec2::new(1.0, 1.0);
	/// assert_eq!(v0.add_xy(), 2.0)
	/// ```
	#[inline(always)]
	pub fn add_xy (self) -> N {
		self.x() + self.y()
	}

	/// Subtracts both of the values together.
	/// # Examples
	///
	/// ```
	/// let v0 = mathie::Vec2::new(1.0, 2.0);
	/// assert_eq!(v0.sub_xy(), -1.0)
	/// ```
	#[inline(always)]
	pub fn sub_xy(self) -> N {
		self.x() - self.y()
	}

	/// Multiplies both of the values together.
	/// # Examples
	///
	/// ```
	/// let v0 = mathie::Vec2::new(2.0, 2.0);
	/// assert_eq!(v0.mul_xy(), 4.0)
	/// ```
	#[inline(always)]
	pub fn mul_xy(self) -> N {
		self.x() * self.y()
	}

	/// Divides x/y of the values together.
	/// # Examples
	///
	/// ```
	/// let v0 = mathie::Vec2::new(2.0, 4.0);
	/// assert_eq!(v0.div_xy(), 0.5)
	/// ```
	#[inline(always)]
	pub fn div_xy(self) -> N {
		self.x() / self.y()
	}

	/// Divides y/x of the values together.
	/// # Examples
	///
	/// ```
	/// let v0 = mathie::Vec2::new(2.0, 4.0);
	/// assert_eq!(v0.div_yx(), 2.0)
	/// ```
	#[inline(always)]
	pub fn div_yx(self) -> N {
		self.y() / self.x()
	}

	/// Returns the X value.
	#[inline(always)]
	pub fn x(self) -> N {
		self.0[0]
	}

	/// Returns the Y value.
	#[inline(always)]
	pub fn y(self) -> N {
		self.0[1]
	}

	/// Does nothing.
	#[inline(always)]
	pub fn xy(self) -> Vec2<N> {
		self
	}

	/// Returns X as Y and Y as X. (Basically inverts the values.)
	#[inline(always)]
	pub fn yx(self) -> Vec2<N> {
		Vec2([
			self.y(),
			self.x(),
		])
	}
}

impl<N: Number + Ord> Vec2<N> {
	/// Gets the smallest coordinate of the Vector.
	/// # Examples
	///
	/// ```
	/// let v0 = mathie::Vec2::new(1, 2);
	/// assert_eq!(v0.min_val(), 1)
	/// ```
	#[inline(always)]
	pub fn min_val(self) -> N {
		N::min(self.x(), self.y())
	}

	/// Gets the biggest coordinate of the Vector.
	/// # Examples
	///
	/// ```
	/// let v0 = mathie::Vec2::new(1, 2);
	/// assert_eq!(v0.max_val(), 2)
	/// ```
	#[inline(always)]
	pub fn max_val(self) ->N  {
		N::max(self.x(), self.y())
	}

	/// Gets the smallest coordinates of both of the vectors.
	/// # Examples
	///
	/// ```
	/// let v0 = mathie::Vec2::new(1, 2);
	/// let other = mathie::Vec2::new(5, 1);
	/// assert_eq!(v0.min(other), mathie::Vec2::new(1, 1))
	/// ```
	#[inline(always)]
	pub fn min(self, other: Vec2<N>) -> Vec2<N> {
		Vec2(
			[
				self.x().min(other.x()),
				self.y().min(other.y()),
			]
		)
	}

	/// Gets the biggest coordinates of both of the vectors.
	/// # Examples
	///
	/// ```
	/// let v0 = mathie::Vec2::new(1, 2);
	/// let other = mathie::Vec2::new(5, 1);
	/// assert_eq!(v0.max(other), mathie::Vec2::new(5, 2))
	/// ```
	#[inline(always)]
	pub fn max(self, other: Vec2<N>) -> Vec2<N> {
		Vec2(
			[
				self.x().max(other.x()),
				self.y().max(other.y()),
			]
		)
	}
}

impl<F: Number + Float> Vec2<F> {
	/// Gets the normalized vector from this vector. Meaning a vector the length of 1
	/// # Examples
	/// ```
	/// use mathie::Vec2;
	/// let value = Vec2::new(4.0, 3.0);
	/// assert_eq!(value.norm(), Vec2::new(4.0 / 5.0, 3.0 / 5.0));
	/// assert_eq!(value.norm().hypot(), 1.0);
	///
	/// let value = Vec2::new(69.0, 420.0);
	/// assert_eq!(value.norm().hypot(), 1.0);
	/// ```
	#[inline(always)]
	pub fn norm(self) -> Vec2<F> {
		let hypot = self.hypot();
		Vec2([
			self.x() / hypot,
			self.y() / hypot,
		])
	}


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
	/// let v0 = mathie::Vec2::new(1.0, 1.0);
	/// let other = mathie::Vec2::new(2.0, 2.0);
	/// assert_eq!(v0.lerp(other, 0.0), v0);
	/// assert_eq!(v0.lerp(other, 1.0), other);
	/// assert_eq!(v0.lerp(other, 0.5), mathie::Vec2::new(1.5, 1.5));
	/// ```
	#[inline(always)]
	pub fn lerp(self, other: Vec2<F>, t: F) -> Vec2<F> {
		Vec2([
			self.x() + ((other.x() - self.x()) * t),
			self.y() + ((other.y() - self.y()) * t),
		])
	}

	/// The same as [Self::min_val] but for floating-point numbers.
	#[inline(always)]
	pub fn minf_val(self) -> F {
		F::min(self.x(), self.y())
	}

	/// The same as [Self::max_val] but for floating-point numbers.
	#[inline(always)]
	pub fn maxf_val(self) -> F {
		F::max(self.x(), self.y())
	}

	/// The same as [Self::min] but for floating-point numbers.
	#[inline(always)]
	pub fn minf(self, other: Vec2<F>) -> Vec2<F>  {
		Vec2(
			[
				self.x().min(other.x()),
				self.y().min(other.y()),
			]
		)
	}

	/// The same as [Self::max] but for floating-point numbers.
	#[inline(always)]
	pub fn maxf(self, other: Vec2<F>) -> Vec2<F> {
		Vec2(
			[
				self.x().max(other.x()),
				self.y().max(other.y()),
			]
		)
	}

	/// Gets the hypotenuse of the vector. In other terms the length.
	#[inline(always)]
	pub fn hypot(self) -> F {
		F::hypot(self.x(), self.y())
	}
}

impl<N: Number> From<[N; 2]> for Vec2<N> {
	#[inline(always)]
	fn from([x, y]: [N; 2]) -> Self {
		Vec2::new(x, y)
	}
}

impl<N: Number> From<(N, N)> for Vec2<N> {
	#[inline(always)]
	fn from((x, y): (N, N)) -> Self {
		Vec2::new(x, y)
	}
}

impl<N: Number> From<Vec2<N>> for [N; 2] {
	#[inline(always)]
	fn from(vec: Vec2<N>) -> Self {
		vec.0
	}
}

impl<N: Number> From<Vec2<N>> for (N, N) {
	#[inline(always)]
	fn from(vec: Vec2<N>) -> Self {
		(vec.0[0], vec.0[1])
	}
}

impl<N: Number> PartialEq<Self> for Vec2<N> {
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool {
		self.0 == other.0
	}
}

impl<N: Number> Eq for Vec2<N> {}

impl<N: Number> PartialOrd<Self> for Vec2<N> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		let x = self.x().partial_cmp(&other.x())?;
		let y = self.y().partial_cmp(&other.y())?;
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

impl<N: Number + Ord> Ord for Vec2<N> {
	fn cmp(&self, other: &Self) -> Ordering {
		let x = self.x().cmp(&other.x());
		let y = self.y().cmp(&other.y());
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
        impl<T: Number + $TRAIT> $TRAIT for Vec2<T> {
            type Output = Vec2<T>;

            fn $METHOD(self, rhs: Self) -> Self::Output {
                Vec2([
	                self.x().$METHOD(rhs.x()), 
	                self.y().$METHOD(rhs.y())
                ])
            }
        }

        impl<T: Number + $TRAIT> $TRAIT<T> for Vec2<T> {
            type Output = Vec2<T>;

            fn $METHOD(self, rhs: T) -> Self::Output {
                Vec2([self.x().$METHOD(rhs), self.y().$METHOD(rhs)])
            }
        }

        impl<T: Number + $TRAIT> $TRAIT_ASSIGN<Vec2<T>> for Vec2<T> {
            fn $METHOD_ASSIGN(&mut self, rhs: Self) {
                *self = self.$METHOD(rhs);
            }
        }

	    impl<T: Number + $TRAIT> $TRAIT_ASSIGN<T> for Vec2<T> {
            fn $METHOD_ASSIGN(&mut self, rhs: T) {
                *self = self.$METHOD(rhs);
            }
        }
    };
}

impl_ops!(impl_op);

impl<N: Number + Neg<Output = N>> Neg for Vec2<N> {
	type Output = Vec2<N>;

	fn neg(self) -> Self::Output {
		Vec2([
			-self.x(),
			-self.y(),
		])
	}
}