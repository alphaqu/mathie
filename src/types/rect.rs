use std::cmp::Ordering;
use crate::impl_ops;
use crate::number::Number;
use crate::types::vec2::Vec2D;
use crate::unit::{check_compatible, Unit, UnitCompatibility};

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Rect<T: Number, U: Unit = ()> {
	pub(crate) origin: [T; 2],
	pub(crate) size: [T; 2],
	pub(crate) unit: U,
}

impl<N: Number> Rect<N> {
	pub fn new_any(origin: impl Into<[N; 2]>, size: impl Into<[N; 2]>) -> Rect<N> {
		Rect::new_u(origin, size, ())
	}

	pub fn new_any_min_max(min: impl Into<[N; 2]>, max: impl Into<[N; 2]>) -> Rect<N> {
		Rect::new_u_min_max(min, max, ())
	}

	pub fn zero() -> Rect<N> {
		Rect::new_any(Vec2D::zero(), Vec2D::zero())
	}

	pub fn one() -> Rect<N> {
		Rect::new_any(Vec2D::one(), Vec2D::one())
	}
}

impl<N: Number, U: Unit + Default> Rect<N, U> {
	pub fn new(origin: impl Into<[N; 2]>, size: impl Into<[N; 2]>) -> Rect<N, U> {
		Rect::new_u(origin, size, U::default())
	}

	pub fn new_min_max(min: impl Into<[N; 2]>, max: impl Into<[N; 2]>) -> Rect<N, U> {
		Rect::new_u_min_max(min, max, U::default())
	}
}

impl<N: Number, U: Unit> Rect<N, U> {
	pub fn new_u(origin: impl Into<[N; 2]>, size: impl Into<[N; 2]>, unit: U) -> Rect<N, U> {
		Rect {
			origin: origin.into(),
			size: size.into(),
			unit,
		}
	}

	pub fn new_u_min_max(min: impl Into<[N; 2]>, max: impl Into<[N; 2]>, unit: U) -> Rect<N, U> {
		let max = max.into();
		let min = min.into();
		Rect {
			origin: min,
			size: [
				max[0] - min[0],
				max[1] - min[1]
			],
			unit,
		}
	}

	#[inline(always)]
	pub fn cast<NO: Number>(self) -> Option<Rect<NO, U>> {
		Some(Rect {
			origin: [
				NO::from(self.origin[0])?,
				NO::from(self.origin[1])?
			],
			size: [
				NO::from(self.size[0])?,
				NO::from(self.size[1])?
			],
			unit: self.unit,
		})
	}

	#[inline(always)]
	pub fn convert_def<UO: UnitCompatibility<N, U> + Default>(self) -> Option<Rect<N, UO>> {
		self.convert_u(UO::default())
	}

	#[inline(always)]
	pub fn convert_u<UO: UnitCompatibility<N, U>>(self, to: UO) -> Option<Rect<N, UO>> {
		to.convert_rect(self)
	}


	/// Checks if self intersects other. In other words it check if any of these rectangles touch each other.
	/// This is very useful in cull testing.
	///
	/// # Arguments
	///
	/// * `other`: The other rectangle to check intersection with.
	///
	/// returns: bool
	///
	/// # Examples
	///
	/// ```
	/// use mathie::Rect;
	/// let rect = Rect::new_any([0.0, 0.0], [1.0, 1.0]);
	/// assert!(rect.intersects_rect(Rect::new_any([0.0, 0.0], [1.0, 1.0])));
	/// assert!(rect.intersects_rect(Rect::new_any([0.4, 0.4], [0.2, 0.2])));
	/// assert!(rect.intersects_rect(Rect::new_any([1.0, 1.0], [1.0, 1.0])));
	/// assert!(rect.intersects_rect(Rect::new_any([0.0, 0.5], [0.5, 1.0])));
	/// assert!(!rect.intersects_rect(Rect::new_any([1.1, 1.1], [1.0, 1.0])));
	/// assert!(!rect.intersects_rect(Rect::new_any([-0.1, -0.1], [0.09, 0.09])));
	/// ```
	pub fn intersects_rect(&self, other: Rect<N, U>) -> bool {
		let s_min = self.min();
		let s_max = self.max();
		let o_min = other.min();
		let o_max = other.max();
		s_min.x() <= (o_max.x()) && (s_max.x()) >= o_min.x() &&
			s_min.y() <= (o_max.y()) && (s_max.y()) >= o_min.y()
	}

	/// Checks if `self` contains `other`, in other words, it checks if `self` fully contains `other`.
	///
	/// # Arguments
	///
	/// * `other`: The other rectangle to check if it is inside self.
	///
	/// returns: bool
	///
	/// # Examples
	///
	/// ```
	/// use mathie::Rect;
	/// let rect = Rect::new_any([0.0, 0.0], [1.0, 1.0]);
	/// assert!(rect.contains_rect(Rect::new_any([0.0, 0.0], [1.0, 1.0])));
	/// assert!(rect.contains_rect(Rect::new_any([0.4, 0.4], [0.2, 0.2])));
	/// assert!(!rect.contains_rect(Rect::new_any([1.0, 1.0], [1.0, 1.0])));
	/// assert!(!rect.contains_rect(Rect::new_any([0.0, 0.5], [0.5, 1.0])));
	/// assert!(!rect.contains_rect(Rect::new_any([1.1, 1.1], [1.0, 1.0])));
	/// assert!(!rect.contains_rect(Rect::new_any([-0.1, -0.1], [0.09, 0.09])));
	/// ```
	pub fn contains_rect(&self, rect: Rect<N, U>) -> bool {
		self.contains_pos(rect.min()) && self.contains_pos(rect.max())
	}

	/// Checks if this position is inside this rectangle
	pub fn contains_pos(&self, pos: Vec2D<N, U>) -> bool {
		let s_min = self.min();
		let s_max = self.max();
		s_min.x() <= pos.x() && pos.x() <= s_max.x() && s_min.y() <= pos.y() && pos.y() <= s_max.y()
	}

	/// Converts this rectangle to a generic any unit.
	#[inline(always)]
	pub fn any_unit(self) -> Rect<N, ()> {
		Rect {
			origin: self.origin,
			size: self.size,
			unit: (),
		}
	}

	/// Gets the unit of this vector.
	#[inline(always)]
	pub fn unit(self) -> U {
		self.unit
	}

	/// Gets the top left corner
	#[inline(always)]
	pub fn top_left(self) -> Vec2D<N, U> {
		self.origin()
	}

	/// Gets the top right corner
	#[inline(always)]
	pub fn top_right(self) -> Vec2D<N, U> {
		self.origin() + Vec2D::new_u(self.size[0], N::zero(), self.unit)
	}

	/// Gets the bottom right corner
	#[inline(always)]
	pub fn bottom_right(self) -> Vec2D<N, U> {
		self.origin() + Vec2D::new_u(self.size[0], self.size[1], self.unit)
	}

	/// Gets the bottom left corner
	#[inline(always)]
	pub fn bottom_left(self) -> Vec2D<N, U> {
		self.origin() + Vec2D::new_u(N::zero(), self.size[1], self.unit)
	}

	#[inline(always)]
	pub fn top(self) -> N {
		self.min().y()
	}

	#[inline(always)]
	pub fn left(self) -> N {
		self.min().x()
	}

	#[inline(always)]
	pub fn bottom(self) -> N {
		self.max().y()
	}

	#[inline(always)]
	pub fn right(self) -> N {
		self.max().x()
	}

	#[inline(always)]
	pub fn center(self) -> Vec2D<N, U> {
		self.origin() + (self.size() / N::from_u8(2).unwrap())
	}

	#[inline(always)]
	pub fn min(self) -> Vec2D<N, U> {
		self.origin()
	}

	#[inline(always)]
	pub fn max(self) -> Vec2D<N, U> {
		self.origin() + self.size()
	}

	/// Gets the origin of the rectangle. (top-left)
	#[inline(always)]
	pub fn origin(self) -> Vec2D<N, U> {
		Vec2D {
			value: self.origin,
			unit: self.unit,
		}
	}

	/// Gets the size of the rectangle
	#[inline(always)]
	pub fn size(self) -> Vec2D<N, U> {
		Vec2D {
			value: self.size,
			unit: self.unit,
		}
	}
}

macro_rules! impl_op {
    ($TRAIT:ident $TRAIT_ASSIGN:ident $METHOD:ident $METHOD_ASSIGN:ident) => {
        impl<T: Number, U: Unit> $TRAIT for Rect<T, U> {
            type Output = Rect<T, U>;

            fn $METHOD(self, rhs: Self) -> Self::Output {
                check_compatible(&self.unit, &rhs.unit);
                Rect {
                    origin: self.origin().$METHOD(rhs.origin()).value,
                    size: self.size().$METHOD(rhs.size()).value,
                    unit: self.unit,
                }
            }
        }
        impl<T: Number, U: Unit> $TRAIT<Vec2D<T, U>> for Rect<T, U> {
            type Output = Rect<T, U>;

            fn $METHOD(self, rhs: Vec2D<T, U>) -> Self::Output {
                check_compatible(&self.unit, &rhs.unit);
                Rect {
                    origin: self.origin().$METHOD(rhs).value,
                    size: self.size().$METHOD(rhs).value,
                    unit: self.unit,
                }
            }
        }

        impl<T: Number, U: Unit> $TRAIT<T> for Rect<T, U> {
            type Output = Rect<T, U>;

            fn $METHOD(self, rhs: T) -> Self::Output {
                Rect {
                    origin: self.origin().$METHOD(rhs).value,
                    size: self.size().$METHOD(rhs).value,
                    unit: self.unit,
                }
            }
        }

        impl<T: Number, U: Unit> $TRAIT_ASSIGN<Rect<T, U>> for Rect<T, U> {
            fn $METHOD_ASSIGN(&mut self, rhs: Self) {
                *self = self.$METHOD(rhs);
            }
        }

        impl<T: Number, U: Unit> $TRAIT_ASSIGN<Vec2D<T, U>> for Rect<T, U> {
            fn $METHOD_ASSIGN(&mut self, rhs: Vec2D<T, U>) {
                *self = self.$METHOD(rhs);
            }
        }

        impl<T: Number, U: Unit> $TRAIT_ASSIGN<T> for Rect<T, U> {
            fn $METHOD_ASSIGN(&mut self, rhs: T) {
                *self = self.$METHOD(rhs);
            }
        }
    };
}

impl_ops!(impl_op);

impl<N: Number, U: Unit> PartialEq<Self> for Rect<N, U> {
	fn eq(&self, other: &Self) -> bool {
		self.origin == other.origin && self.size == other.size
	}
}

impl<N: Number, U: Unit> Eq for Rect<N, U> {}

impl<N: Number, U: Unit> PartialOrd<Self> for Rect<N, U> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		let origin = self.origin().partial_cmp(&other.origin())?;
		let size = self.size().partial_cmp(&other.size())?;
		origin.partial_cmp(&size)
	}
}

impl<N: Number + Ord, U: Unit> Ord for Rect<N, U> {
	fn cmp(&self, other: &Self) -> Ordering {
		let origin = self.origin().cmp(&other.origin());
		let size = self.size().cmp(&other.size());
		origin.cmp(&size)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn contains() {
		let rect = Rect::new_any([0.0f32, 0.0], [1.0, 1.0]);
		assert!(rect.contains_rect(Rect::new_any([0.0f32, 0.0], [1.0, 1.0])));
		assert!(rect.contains_rect(Rect::new_any([0.1f32, 0.1], [0.8, 0.8])));
		assert!(!rect.contains_rect(Rect::new_any([0.1f32, 0.1], [1.0, 1.0])));
		assert!(!rect.contains_rect(Rect::new_any([-0.1, -0.1], [1.1, 1.1])));
	}

	#[test]
	fn overlaps() {
		let rect = Rect::new_any([0.0f32, 0.0], [1.0, 1.0]);
		assert!(rect.intersects_rect(Rect::new_any([0.0f32, 0.0], [1.0, 1.0])));
		assert!(rect.intersects_rect(Rect::new_any([0.1f32, 0.1], [0.8, 0.8])));
		assert!(rect.intersects_rect(Rect::new_any([0.1f32, 0.1], [1.0, 1.0])));
		assert!(rect.intersects_rect(Rect::new_any([-0.1, -0.1], [1.2, 1.2])));
		assert!(rect.intersects_rect(Rect::new_any([-0.1f32, -0.1f32], [0.1, 0.1])));
		assert!(!rect.intersects_rect(Rect::new_any([1.11f32, 1.11], [1.0, 1.0])));
	}
}