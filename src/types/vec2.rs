use std::cmp::Ordering;
use std::fmt::Debug;
use crate::number::Number;
use crate::unit::{check_compatible, Unit, UnitCompatibility};
use crate::impl_ops;
use crate::types::value::Value;


#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Vec2<T: Number, U: Unit = ()> {
    pub(crate) value: [T; 2],
    pub(crate) unit: U,
}

impl<T: Number> Vec2<T> {
    pub fn new(x: T, y: T) -> Vec2<T> {
        Vec2::new_u(x, y, ())
    }

    pub fn zero() -> Vec2<T> {
        Vec2::new(T::zero(), T::zero())
    }

    pub fn one() -> Vec2<T> {
        Vec2::new(T::one(), T::one())
    }
}


impl<T: Number, U: Unit + Default> Vec2<T, U> {
    pub fn new_def(x: T, y: T) -> Vec2<T, U> {
        Vec2::new_u(x, y, U::default())
    }
}

impl<N: Number, U: Unit> Vec2<N, U> {
    pub fn new_u(x: N, y: N, unit: U) -> Vec2<N, U> {
        Vec2 {
            value: [x, y],
            unit
        }
    }

    #[inline(always)]
    pub fn cast<NO: Number>(self) ->  Option<Vec2<NO, U>> {
       Some(Vec2 {
           value: [
               NO::from(self.value[0])?,
               NO::from(self.value[1])?
           ],
           unit: self.unit
       })
    }

    #[inline(always)]
    pub fn convert_def<UO: UnitCompatibility<N, U> + Default>(self) ->  Option<Vec2<N, UO>> {
        self.convert_u(UO::default())
    }

    #[inline(always)]
    pub fn convert_u<UO: UnitCompatibility<N, U>>(self, to: UO) -> Option<Vec2<N, UO>> {
        to.convert_pos2(self)
    }

    #[inline(always)]
    pub fn any_unit(self) -> Vec2<N, ()> {
        Vec2 {
            value: self.value,
            unit: ()
        }
    }

    #[inline(always)]
    pub fn unit(self) -> U {
        self.unit
    }

    #[inline(always)]
    pub fn x(self) -> Value<N, U> {
        Value::new(self.value[0], self.unit)
    }

    #[inline(always)]
    pub fn y(self) -> Value<N, U> {
        Value::new(self.value[1], self.unit)
    }

    #[inline(always)]
    pub fn xy(self) -> Vec2<N, U> {
        self
    }

    #[inline(always)]
    pub fn yx(self) -> Vec2<N, U> {
        Vec2 {
            value: [
                self.value[1],
                self.value[0],
            ],
            unit: self.unit
        }
    }
}

impl<N: Number, U: Unit> Into<[N; 2]> for Vec2<N, U> {
    fn into(self) -> [N; 2] {
        self.value
    }
}

impl<N: Number, U: Unit> Into<(N, N)> for Vec2<N, U> {
    fn into(self) -> (N, N) {
        (self.value[0], self.value[1])
    }
}

impl<N: Number, U: Unit> PartialEq<Self> for Vec2<N, U> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<N: Number, U: Unit> Eq for Vec2<N, U> {

}

impl<N: Number, U: Unit> PartialOrd<Self> for Vec2<N, U> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let x = self.value[0].partial_cmp(&other.value[0])?;
        let y = self.value[1].partial_cmp(&other.value[1])?;
        x.partial_cmp(&y)
    }
}

impl<N: Number + Ord, U: Unit> Ord for Vec2<N, U> {
    fn cmp(&self, other: &Self) -> Ordering {
        let x = self.value[0].cmp(&other.value[0]);
        let y = self.value[1].cmp(&other.value[1]);
        x.cmp(&y)
    }
}


macro_rules! impl_op {
    ($TRAIT:ident $TRAIT_ASSIGN:ident $METHOD:ident $METHOD_ASSIGN:ident) => {
        impl<T: Number, U: Unit> $TRAIT for Pos2D<T, U> {
            type Output = Pos2D<T, U>;

            fn $METHOD(self, rhs: Self) -> Self::Output {
                check_compatible(&self.unit, &rhs.unit);
                Pos2D {
                    value: [self.value[0].$METHOD(rhs.value[0]), self.value[1].$METHOD(rhs.value[1])],
                    unit: self.unit,
                }
            }
        }

        impl<T: Number, U: Unit> $TRAIT<T> for Pos2D<T, U> {
            type Output = Pos2D<T, U>;

            fn $METHOD(self, rhs: T) -> Self::Output {
                Pos2D {
                    value: [self.value[0].$METHOD(rhs), self.value[1].$METHOD(rhs)],
                    unit: self.unit,
                }
            }
        }

        impl<T: Number, U: Unit> $TRAIT_ASSIGN<Pos2D<T, U>> for Pos2D<T, U> {
            fn $METHOD_ASSIGN(&mut self, rhs: Self) {
                *self = self.$METHOD(rhs);
            }
        }

	    impl<T: Number, U: Unit> $TRAIT_ASSIGN<T> for Pos2D<T, U> {
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
        let v0 = Vec2::new(0.5, 0.5);
        let v1 = Vec2::new(1.0, 1.0);

        assert_eq!(v0 + v1, Vec2::new(1.5, 1.5));
        assert_eq!(v0 + 2.0, Vec2::new(2.5, 2.5));
    }
}