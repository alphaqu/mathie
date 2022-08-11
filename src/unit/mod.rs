#[cfg(feature = "metric_units")]
pub mod metric;
#[cfg(feature = "imperial_units")]
pub mod imperial;
#[cfg(feature = "nautical_units")]
pub mod nautical;

use std::fmt::Debug;
use crate::number::Number;
use crate::types::vec2::Vec2D;
use crate::types::rect::Rect;

pub trait Unit: Copy + Debug {
	/// When for example adding two numbers together.
	/// The two unit types need to be checked if both sides are compatible with each other.
	/// If this returns false the function will panic saying the units are not compatible.
	#[inline(always)]
	fn is_self_compatible(&self, rhs: &Self) -> bool {
		true
	}
}

pub trait UnitCompatibility<N: Number, Rhs: Unit>: Unit {
	fn convert_value(&self, value: N, unit: Rhs) -> Option<N>;
	/// Converts the Position with the unit `Rhs` to the desired unit.
	fn convert_pos2(&self, pos: Vec2D<N, Rhs>) -> Option<Vec2D<N, Self>> {
		let x = self.convert_value(pos.x(), pos.unit)?;
		let y = self.convert_value(pos.y(), pos.unit)?;
		Some(Vec2D::new_u(x, y, *self))
	}
	fn convert_rect(&self, rect: Rect<N, Rhs>) -> Option<Rect<N, Self>> {
		let origin = rect.origin().try_convert_u(*self)?;
		let size = rect.size().try_convert_u(*self)?;
		Some(Rect::new_u(origin, size, *self))
	}
}


impl Unit for () {
}

#[inline(always)]
pub(crate) fn check_compatible<U: Unit>(v0: &U, v1: &U)  {
	let compatible = v0.is_self_compatible(v1);
	#[cfg(feature = "debug_unit_check")]
	debug_assert!(compatible);
	#[cfg(not(feature = "debug_unit_check"))]
	assert!(compatible);
}

#[macro_export]
macro_rules! prefix {

    ($NAME:ident $BASE:literal $($SYMBOL:ident)*) => {
	    $(
	    pub type $SYMBOL = $NAME;
	    )*
	    
        #[derive(Copy, Clone, Debug, Default)]
        pub struct $NAME;

        impl crate::unit::BasePrefix for $NAME {
            fn base_factor() -> f64 {
                $BASE
            }
        }

        impl crate::unit::Unit for $NAME {}
    };
}

pub trait BasePrefix: Copy + Debug + Default {
	fn base_factor() -> f64;
}


impl BasePrefix for () {
	fn base_factor() -> f64 {
		1.0
	}
}


impl<N: Number, F: BasePrefix + Unit, T: BasePrefix + Unit> UnitCompatibility<N, F> for T {
	fn convert_value(&self, value: N, _: F) -> Option<N> {
		let ratio = F::base_factor() / T::base_factor();
		N::from_f64(value.to_f64()? * ratio)
	}
}