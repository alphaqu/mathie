use std::cmp::Ordering;
use crate::impl_ops;
use crate::Number;
use crate::Vec2;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Rect<T: Number> {
	pub(crate) origin: Vec2<T>,
	pub(crate) size: Vec2<T>,
}

impl<N: Number> Rect<N> {
	pub fn new(origin: impl Into<[N; 2]>, size: impl Into<[N; 2]>) -> Rect<N> {
		Rect {
			origin: Vec2::from(origin.into()),
			size: Vec2::from(size.into()),
		}
	}


	pub fn new_min_max(min: impl Into<[N; 2]>, max: impl Into<[N; 2]>) -> Rect<N> {
		let max = Vec2::from(max.into());
		let min = Vec2::from(min.into());
		Rect {
			origin: min,
			size: max - min,
		}
	}

	pub fn zero() -> Rect<N> {
		Rect::new(Vec2::zero(), Vec2::zero())
	}

	/// Returns a rectangle that is size of one and has an origin on 0
	pub fn one() -> Rect<N> {
		Rect::new(Vec2::zero(), Vec2::one())
	}

	/// Casts the rectangle to another primitive unit, panicking if the unit cannot be represented.
	#[inline(always)]
	pub fn cast<NO: Number>(self) -> Rect<NO> {
		Rect {
			origin: self.origin.try_cast().expect("Failed to cast origin"),
			size: self.size.try_cast().expect("Failed to cast size"),
		}
	}

	/// Same as [Self::cast] but returns None if the cast failed.
	#[inline(always)]
	pub fn try_cast<NO: Number>(self) -> Option<Rect<NO>> {
		Some(Rect {
			origin: self.origin.try_cast()?,
			size: self.size.try_cast()?,
		})
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
	/// let rect = Rect::one();
	/// assert!(rect.intersects_rect(Rect::new([0.0, 0.0], [1.0, 1.0])));
	/// assert!(rect.intersects_rect(Rect::new([0.4, 0.4], [0.2, 0.2])));
	/// assert!(rect.intersects_rect(Rect::new([1.0, 1.0], [1.0, 1.0])));
	/// assert!(rect.intersects_rect(Rect::new([0.0, 0.5], [0.5, 1.0])));
	/// assert!(!rect.intersects_rect(Rect::new([1.1, 1.1], [1.0, 1.0])));
	/// assert!(!rect.intersects_rect(Rect::new([-0.1, -0.1], [0.09, 0.09])));
	/// ```
	pub fn intersects_rect(&self, other: Rect<N>) -> bool {
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
	/// let rect = Rect::one();
	/// assert!(rect.contains_rect(Rect::new([0.0, 0.0], [1.0, 1.0])));
	/// assert!(rect.contains_rect(Rect::new([0.4, 0.4], [0.2, 0.2])));
	/// assert!(!rect.contains_rect(Rect::new([1.0, 1.0], [1.0, 1.0])));
	/// assert!(!rect.contains_rect(Rect::new([0.0, 0.5], [0.5, 1.0])));
	/// assert!(!rect.contains_rect(Rect::new([1.1, 1.1], [1.0, 1.0])));
	/// assert!(!rect.contains_rect(Rect::new([-0.1, -0.1], [0.09, 0.09])));
	/// ```
	pub fn contains_rect(&self, rect: Rect<N>) -> bool {
		self.contains_pos(rect.min()) && self.contains_pos(rect.max())
	}

	/// Checks if this position is inside this rectangle
	/// # Examples
	/// ```
	/// use mathie::{Rect, Vec2};
	/// let rect = Rect::one();
	/// assert!(rect.contains_pos(Vec2::new(0.5, 0.5)));
	/// assert!(rect.contains_pos(Vec2::new(0.0, 0.0)));
	/// assert!(rect.contains_pos(Vec2::new(1.0, 1.0)));
	/// assert!(rect.contains_pos(Vec2::new(0.0, 1.0)));
	/// assert!(rect.contains_pos(Vec2::new(1.0, 0.0)));
	/// assert!(!rect.contains_pos(Vec2::new(0.0, -0.1)));
	/// assert!(!rect.contains_pos(Vec2::new(-0.1, 0.0)));
	/// ```
	pub fn contains_pos(&self, pos: Vec2<N>) -> bool {
		let min = self.min();
		let max = self.max();
		min.x() <= pos.x() && pos.x() <= max.x() && min.y() <= pos.y() && pos.y() <= max.y()
	}

	/// Checks if the rectangle has a negative area
	/// # Examples
	/// ```
	/// use mathie::{Rect, Vec2};
	/// assert!(!Rect::new([0.0, 0.0], [1.0, 1.0]).is_negative());
	/// assert!(!Rect::new([0.0, 0.0], [0.0, 0.0]).is_negative());
	/// assert!(Rect::new([0.0, 0.0], [-1.0, -1.0]).is_negative());
	/// ```
	pub fn is_negative(&self) -> bool {
		let min = self.min();
		let max = self.max();
		max.x() < min.x() || max.y() < min.y()
	}

	/// Checks if the rectangle area is zero, negative or NaN.
	/// # Examples
	/// ```
	/// use mathie::{Rect, Vec2};
	/// assert!(!Rect::new([0.0, 0.0], [1.0, 1.0]).is_empty());
	/// assert!(Rect::new([0.0, 0.0], [0.0, 0.0]).is_empty());
	/// assert!(Rect::new([0.0, 0.0], [-1.0, -1.0]).is_empty());
	/// ```
	pub fn is_empty(&self) -> bool {
		let min = self.min();
		let max = self.max();
		!(max.x() > min.x() && max.y() > min.y())
	}

	/// Makes the rectangle smaller in the x and y directions keeping its center.
	/// # Examples
	/// ```
	/// use mathie::{Rect, Vec2};
	/// let rect = Rect::one();
	/// assert_eq!(rect.shrink(Vec2::one()), Rect::new([0.5, 0.5], [0.0, 0.0]));
	/// assert_eq!(rect.shrink(Vec2::one() / 2.0), Rect::new([0.25, 0.25], [0.5, 0.5]));
	/// ```
	pub fn shrink(&self, value: Vec2<N>) -> Self {
		let mut out = *self;
		let half_value = value / N::from_u8(2).unwrap();
		out.origin += half_value;
		out.size -= half_value;
		out.size -= half_value;
		out
	}

	/// Makes the rectangle bigger in the x and y directions keeping its center.
	/// # Examples
	/// ```
	/// use mathie::{Rect, Vec2};
	/// let rect = Rect::one();
	/// assert_eq!(rect.expand(Vec2::one()), Rect::new([-0.5, -0.5], [2.0, 2.0]));
	/// assert_eq!(rect.expand(Vec2::one() / 2.0), Rect::new([-0.25, -0.25], [1.5, 1.5]));
	/// ```
	pub fn expand(&self, value: Vec2<N>) -> Self {
		let mut out = *self;
		let half_value = value / N::from_u8(2).unwrap();
		out.origin -= half_value;
		out.size += half_value;
		out.size += half_value;
		out
	}

	/// Gets the top left corner
	#[inline(always)]
	pub fn top_left(self) -> Vec2<N> {
		self.origin()
	}

	/// Gets the top right corner
	#[inline(always)]
	pub fn top_right(self) -> Vec2<N> {
		self.origin() + Vec2::new(self.size.x(), N::zero())
	}

	/// Gets the bottom right corner
	#[inline(always)]
	pub fn bottom_right(self) -> Vec2<N> {
		self.origin() + self.size()
	}

	/// Gets the bottom left corner
	#[inline(always)]
	pub fn bottom_left(self) -> Vec2<N> {
		self.origin() + Vec2::new(N::zero(), self.size.y())
	}

	/// Gets the top Y coordinate
	#[inline(always)]
	pub fn top(self) -> N {
		self.origin.y()
	}

	/// Gets the left X coordinate
	#[inline(always)]
	pub fn left(self) -> N {
		self.origin.x()
	}

	/// Gets the bottom Y coordinate
	#[inline(always)]
	pub fn bottom(self) -> N {
		self.origin.y() + self.size.y()
	}

	/// Gets the right X coordinate
	#[inline(always)]
	pub fn right(self) -> N {
		self.origin.x() + self.size.x()
	}

	/// Gets the coordinates to the center of the rectangle
	#[inline(always)]
	pub fn center(self) -> Vec2<N> {
		self.origin() + (self.size() / N::from_u8(2).unwrap())
	}

	/// Gets the coordinates to the top of the rectangle while being centered on the x axis
	#[inline(always)]
	pub fn top_center(self) -> Vec2<N> {
		Vec2::new(self.center().x(), self.top())
	}

	/// Gets the coordinates to the bottom of the rectangle while being centered on the x axis
	#[inline(always)]
	pub fn bottom_center(self) -> Vec2<N> {
		Vec2::new(self.center().x(), self.bottom())
	}

	/// Gets the coordinates to the left of the rectangle while being centered on the y axis
	#[inline(always)]
	pub fn center_left(self) -> Vec2<N> {
		Vec2::new(self.left(), self.center().y())
	}

	/// Gets the coordinates to the right of the rectangle while being centered on the y axis
	#[inline(always)]
	pub fn center_right(self) -> Vec2<N> {
		Vec2::new(self.right(), self.center().y())
	}

	#[inline(always)]
	pub fn min(self) -> Vec2<N> {
		self.origin()
	}

	#[inline(always)]
	pub fn max(self) -> Vec2<N> {
		self.origin() + self.size()
	}

	/// Gets the origin of the rectangle. (top-left)
	#[inline(always)]
	pub fn origin(self) -> Vec2<N> {
		self.origin
	}

	/// Gets the size of the rectangle
	#[inline(always)]
	pub fn size(self) -> Vec2<N> {
		self.size
	}
}

impl<N: Number + Ord> Rect<N> {
	/// Returns the intersection between two rectangles.
	pub fn intersection(&self, other: Self) -> Self {
		let min = Rect::min(*self);
		let max = Rect::max(*self);
		let o_min = other.min();
		let o_max = other.max();

		Rect {
			origin: Vec2::new(min.x().max(o_min.x()), min.y().max(o_min.y())),
			size: Vec2::new(max.x().min(o_max.x()), max.y().min(o_max.y()))
		}
	}
}


impl<N: Number> PartialEq<Self> for Rect<N> {
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool {
		self.origin == other.origin && self.size == other.size
	}
}

impl<N: Number> Eq for Rect<N> {}

impl<N: Number> PartialOrd<Self> for Rect<N> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		let origin = self.origin().partial_cmp(&other.origin())?;
		let size = self.size().partial_cmp(&other.size())?;
		origin.partial_cmp(&size)
	}
}

impl<N: Number + Ord> Ord for Rect<N> {
	fn cmp(&self, other: &Self) -> Ordering {
		let origin = self.origin().cmp(&other.origin());
		let size = self.size().cmp(&other.size());
		origin.cmp(&size)
	}
}

macro_rules! impl_op {
    ($TRAIT:ident $TRAIT_ASSIGN:ident $METHOD:ident $METHOD_ASSIGN:ident) => {
        impl<T: Number> $TRAIT for crate::Rect<T> {
            type Output = crate::Rect<T>;

            fn $METHOD(self, rhs: Self) -> Self::Output {
                crate::Rect {
                    origin: self.origin().$METHOD(rhs.origin()),
                    size: self.size().$METHOD(rhs.size()),
                }
            }
        }
        impl<T: Number> $TRAIT<crate::Vec2<T>> for crate::Rect<T> {
            type Output = crate::Rect<T>;

            fn $METHOD(self, rhs: crate::Vec2<T>) -> Self::Output {
               crate::Rect {
                    origin: self.origin().$METHOD(rhs),
                    size: self.size().$METHOD(rhs),
                }
            }
        }

        impl<T: Number> $TRAIT<T> for crate::Rect<T> {
            type Output = crate::Rect<T>;

            fn $METHOD(self, rhs: T) -> Self::Output {
                crate::Rect {
                    origin: self.origin().$METHOD(rhs),
                    size: self.size().$METHOD(rhs),
                }
            }
        }

        impl<T: Number> $TRAIT_ASSIGN<crate::Rect<T>> for crate::Rect<T> {
            fn $METHOD_ASSIGN(&mut self, rhs: Self) {
                *self = self.$METHOD(rhs);
            }
        }

        impl<T: Number> $TRAIT_ASSIGN<crate::Vec2<T>> for crate::Rect<T> {
            fn $METHOD_ASSIGN(&mut self, rhs: crate::Vec2<T>) {
                *self = self.$METHOD(rhs);
            }
        }

        impl<T: Number> $TRAIT_ASSIGN<T> for crate::Rect<T> {
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
	fn contains() {
		let rect = Rect::new([0.0f32, 0.0], [1.0, 1.0]);
		assert!(rect.contains_rect(Rect::new([0.0f32, 0.0], [1.0, 1.0])));
		assert!(rect.contains_rect(Rect::new([0.1f32, 0.1], [0.8, 0.8])));
		assert!(!rect.contains_rect(Rect::new([0.1f32, 0.1], [1.0, 1.0])));
		assert!(!rect.contains_rect(Rect::new([-0.1, -0.1], [1.1, 1.1])));
	}

	#[test]
	fn overlaps() {
		let rect = Rect::new([0.0f32, 0.0], [1.0, 1.0]);
		assert!(rect.intersects_rect(Rect::new([0.0f32, 0.0], [1.0, 1.0])));
		assert!(rect.intersects_rect(Rect::new([0.1f32, 0.1], [0.8, 0.8])));
		assert!(rect.intersects_rect(Rect::new([0.1f32, 0.1], [1.0, 1.0])));
		assert!(rect.intersects_rect(Rect::new([-0.1, -0.1], [1.2, 1.2])));
		assert!(rect.intersects_rect(Rect::new([-0.1f32, -0.1f32], [0.1, 0.1])));
		assert!(!rect.intersects_rect(Rect::new([1.11f32, 1.11], [1.0, 1.0])));
	}
}