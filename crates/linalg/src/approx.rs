use approx::{AbsDiffEq, RelativeEq, UlpsEq};

use crate::{RowMatrix, Vector};

impl<T: AbsDiffEq, const N: usize> AbsDiffEq for Vector<T, N>
where
	T::Epsilon: Copy,
{
	type Epsilon = T::Epsilon;

	fn default_epsilon() -> T::Epsilon {
		T::default_epsilon()
	}

	fn abs_diff_eq(&self, other: &Self, epsilon: T::Epsilon) -> bool {
		for i in 0..N {
			if !T::abs_diff_eq(&self[i], &other[i], epsilon) {
				return false;
			}
		}

		true
	}
}

impl<T: RelativeEq, const N: usize> RelativeEq for Vector<T, N>
where
	T::Epsilon: Copy,
{
	fn default_max_relative() -> T::Epsilon {
		T::default_max_relative()
	}

	fn relative_eq(
		&self,
		other: &Self,
		epsilon: Self::Epsilon,
		max_relative: T::Epsilon,
	) -> bool {
		for i in 0..N {
			if !T::relative_eq(
				&self[i],
				&other[i],
				epsilon,
				max_relative,
			) {
				return false;
			}
		}

		true
	}
}

impl<T: UlpsEq, const N: usize> UlpsEq for Vector<T, N>
where
	T::Epsilon: Copy,
{
	fn default_max_ulps() -> u32 {
		T::default_max_ulps()
	}

	fn ulps_eq(
		&self,
		other: &Self,
		epsilon: Self::Epsilon,
		max_ulps: u32,
	) -> bool {
		for i in 0..N {
			if !T::ulps_eq(&self[i], &other[i], epsilon, max_ulps) {
				return false;
			}
		}

		true
	}
}

// XXX: double indexing requires copy.  See if there's a way to avoid that
impl<T: AbsDiffEq, const N: usize, const M: usize> AbsDiffEq
	for RowMatrix<T, N, M>
where
	T: Copy,
	T::Epsilon: Copy,
{
	type Epsilon = T::Epsilon;

	fn default_epsilon() -> T::Epsilon {
		T::default_epsilon()
	}

	fn abs_diff_eq(&self, other: &Self, epsilon: T::Epsilon) -> bool {
		for i in 0..N {
			for j in 0..M {
				if !T::abs_diff_eq(
					&self[i][j],
					&other[i][j],
					epsilon,
				) {
					return false;
				}
			}
		}

		true
	}
}

impl<T: RelativeEq, const N: usize, const M: usize> RelativeEq
	for RowMatrix<T, N, M>
where
	T: Copy,
	T::Epsilon: Copy,
{
	fn default_max_relative() -> T::Epsilon {
		T::default_max_relative()
	}

	fn relative_eq(
		&self,
		other: &Self,
		epsilon: Self::Epsilon,
		max_relative: T::Epsilon,
	) -> bool {
		for i in 0..N {
			for j in 0..M {
				if !T::relative_eq(
					&self[i][j],
					&other[i][j],
					epsilon,
					max_relative,
				) {
					return false;
				}
			}
		}

		true
	}
}

impl<T: UlpsEq, const N: usize, const M: usize> UlpsEq for RowMatrix<T, N, M>
where
	T: Copy,
	T::Epsilon: Copy,
{
	fn default_max_ulps() -> u32 {
		T::default_max_ulps()
	}

	fn ulps_eq(
		&self,
		other: &Self,
		epsilon: Self::Epsilon,
		max_ulps: u32,
	) -> bool {
		for i in 0..N {
			for j in 0..M {
				if !T::ulps_eq(
					&self[i][j],
					&other[i][j],
					epsilon,
					max_ulps,
				) {
					return false;
				}
			}
		}

		true
	}
}
