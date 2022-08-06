#[macro_export]
macro_rules! impl_ops {
    ($MACRO:ident) => {
	    use std::ops::{Add, AddAssign};
	    $MACRO!(Add AddAssign add add_assign);
	    use std::ops::{Sub, SubAssign};
	    $MACRO!(Sub SubAssign sub sub_assign);
	    use std::ops::{Div, DivAssign};
	    $MACRO!(Div DivAssign div div_assign);
	    use std::ops::{Mul, MulAssign};
	    $MACRO!(Mul MulAssign mul mul_assign);
	    use std::ops::{Rem, RemAssign};
	    $MACRO!(Rem RemAssign rem rem_assign);
    };
}
