//! Implements From for euclid types for inter-compatibility.
use euclid::{Point2D, Vector2D};
use crate::number::Number;
use crate::types::rect::Rect;
use crate::types::vec2::Vec2;

impl<T: Number, U> From<Vector2D<T, U>> for Vec2<T> {
	fn from(vec: Vector2D<T, U>) -> Self {
		Vec2::new(vec.x, vec.y)
	}
}

impl<T: Number, U> From<Point2D<T, U>> for Vec2<T> {
	fn from(point: Point2D<T, U>) -> Self {
		Vec2::new(point.x, point.y)
	}
}

impl<T: Number, U> From<euclid::Rect<T, U>> for Rect<T> {
	fn from(rect: euclid::Rect<T, U>) -> Self {
		Rect::new(rect.origin.to_array(), rect.size.to_array())
	}
}

impl<T: Number, U> From<euclid::Box2D<T, U>> for Rect<T> {
	fn from(b: euclid::Box2D<T, U>) -> Self {
		Rect::new_min_max(b.min.to_array(), b.max.to_array())
	}
}