use std::{
	mem::{self, MaybeUninit},
	ops::{Add, AddAssign, Mul},
};

pub struct Vector<T, const N: usize> {
	v: [T; N],
}

impl<T, const N: usize> From<[T; N]> for Vector<T, N> {
	fn from(value: [T; N]) -> Self {
		Self { v: value }
	}
}

impl<T, const N: usize> Add for Vector<T, N>
where
	T: Add + Copy,
{
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		let mut out = [const { MaybeUninit::uninit() }; N];

		#[allow(clippy::needless_range_loop)]
		for i in 0..N {
			out[i].write(self.v[i] + rhs.v[i]);
		}

		// SAFETY: for the integer/float types [T; N] has the same
		// layout as [MaybeUninit<T>; N].  Verifying this for
		// non-primitive types is a TODO.
		unsafe { mem::transmute_copy::<_, [T; N]>(&out).into() }
	}
}

impl<T, const N: usize> Vector<T, N>
where
	T: Add + AddAssign + Mul<Output = T> + Copy,
{
	pub fn dot(&self, other: &Vector<T, N>) -> T {
		let mut out = self.v[0] * other.v[0];

		for i in 1..N {
			out += self.v[i] * other.v[i];
		}

		out
	}
}
