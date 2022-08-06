use crate::impl_ops;
use crate::number::Number;
use crate::types::vec2::Vec2D;
use crate::unit::{check_compatible, Unit, UnitCompatibility};
use std::cmp::Ordering;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Rect<T: Number, U: Unit = ()> {
    pub(crate) origin: [T; 2],
    pub(crate) size: [T; 2],
    pub(crate) unit: U,
}

impl<N: Number> Rect<N> {
    pub fn new(origin: impl Into<[N; 2]>, size: impl Into<[N; 2]>) -> Rect<N> {
        Rect::new_u(origin, size, ())
    }

    pub fn new_min_max(min: impl Into<[N; 2]>, max: impl Into<[N; 2]>) -> Rect<N> {
        Rect::new_min_max_u(min, max, ())
    }

    pub fn zero() -> Rect<N> {
        Rect::new(Vec2D::zero(), Vec2D::zero())
    }

    pub fn one() -> Rect<N> {
        Rect::new(Vec2D::one(), Vec2D::one())
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

    pub fn new_min_max_u(min: impl Into<[N; 2]>, max: impl Into<[N; 2]>, unit: U) -> Rect<N, U> {
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
    pub fn cast<NO: Number>(self) ->  Option<Rect<NO, U>> {
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
    pub fn convert_def<UO: UnitCompatibility<N, U> + Default>(self) ->  Option<Rect<N, UO>> {
        self.convert_u(UO::default())
    }

    #[inline(always)]
    pub fn convert_u<UO: UnitCompatibility<N, U>>(self, to: UO) -> Option<Rect<N, UO>> {
        to.convert_rect(self)
    }

    pub fn contains_pos(&self, pos: Vec2D<N, U>) -> bool {
        let s_min = self.min();
        let s_max = self.max();
        s_min.x() <= pos.x() && pos.x() < s_max.x() && s_min.y() <= pos.y() && pos.y() < s_max.y()
    }

    pub fn contains_rect(&self, rect: Rect<N, U>) -> bool {
        let s_min = self.min();
        let s_max = self.max();
        let o_min = rect.min();
        let o_max = rect.max();
        s_min.x() <= o_min.x()
            && o_max.x() <= s_max.x()
            && s_min.y() <= o_min.y()
            && o_max.y() <= s_max.y()
    }

    #[inline(always)]
    pub fn any_unit(self) -> Rect<N, ()> {
        Rect {
            origin: self.origin,
            size: self.size,
            unit: (),
        }
    }

    #[inline(always)]
    pub fn unit(self) -> U {
        self.unit
    }

    #[inline(always)]
    pub fn min(self) -> Vec2D<N, U> {
        self.origin()
    }

    #[inline(always)]
    pub fn max(self) -> Vec2D<N, U> {
        self.origin() + self.size()
    }

    #[inline(always)]
    pub fn origin(self) -> Vec2D<N, U> {
        Vec2D {
            value: self.origin,
            unit: self.unit,
        }
    }

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
