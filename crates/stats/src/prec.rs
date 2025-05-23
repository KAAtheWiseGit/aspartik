//! Provides utility functions for working with floating point precision

use approx::AbsDiffEq;

/// Targeted accuracy instantiated over `f64`
pub const ACCURACY: f64 = 10e-11;

/// Standard epsilon, maximum relative precision of IEEE 754 double-precision
/// floating point numbers (64 bit) e.g. `2^-53`
pub const F64_PREC: f64 = 0.00000000000000011102230246251565;

/// Default accuracy for `f64`, equivalent to `0.0 * F64_PREC`
pub const DEFAULT_F64_ACC: f64 = 0.0000000000000011102230246251565;

pub fn almost_eq(a: f64, b: f64, acc: f64) -> bool {
	if a.is_infinite() && b.is_infinite() {
		return a == b;
	}
	a.abs_diff_eq(&b, acc)
}

/// Compares if two floats are close via `approx::abs_diff_eq`
/// using a maximum absolute difference (epsilon) of `acc`.
#[macro_export]
macro_rules! assert_almost_eq {
    ($a:expr, $b:expr, $prec:expr $(,)?) => {
        if !$crate::almost_eq($a, $b, $prec) {
            panic!(
                "assertion failed: `abs(left - right) < {:e}`, (left: `{}`, right: `{}`)",
                $prec, $a, $b
            );
        }
    };
}

/// Compares if two floats are close via `approx::relative_eq!`
/// and `crate::consts::ACC` relative precision.
/// Updates first argument to value of second argument
pub fn convergence(x: &mut f64, x_new: f64) -> bool {
	let res = approx::relative_eq!(*x, x_new, max_relative = ACCURACY);
	*x = x_new;
	res
}
