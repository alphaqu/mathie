use num_traits::{FromPrimitive, Num, NumCast, ToPrimitive};

pub trait Number: Num + FromPrimitive + ToPrimitive + Copy + PartialOrd + NumCast {
}

impl<N: Num + Copy + FromPrimitive + ToPrimitive + PartialOrd + NumCast> Number for N {

}