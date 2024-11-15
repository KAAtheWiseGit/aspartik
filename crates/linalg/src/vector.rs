use std::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign};

#[derive(Debug, Clone, Copy)]
pub struct Vector<T: Copy, const N: usize> {
	v: [T; N],
}

impl<T: Copy, const N: usize> From<[T; N]> for Vector<T, N> {
	fn from(value: [T; N]) -> Self {
		Self { v: value }
	}
}

impl<T: Copy, const N: usize> From<T> for Vector<T, N> {
	fn from(value: T) -> Self {
		[value; N].into()
	}
}

// This can be derived, but then every single trait impl would have to carry a
// `Default` constraint around, which is verbose.
impl<T: Copy + Default, const N: usize> Default for Vector<T, N> {
	fn default() -> Self {
		[T::default(); N].into()
	}
}

impl<T: Copy, const N: usize> Index<usize> for Vector<T, N> {
	type Output = T;

	fn index(&self, index: usize) -> &Self::Output {
		&self.v[index]
	}
}

impl<T: Copy, const N: usize> IndexMut<usize> for Vector<T, N> {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		&mut self.v[index]
	}
}

impl<T: Copy + AddAssign, const N: usize> AddAssign for Vector<T, N> {
	fn add_assign(&mut self, rhs: Self) {
		for i in 0..N {
			self[i] += rhs[i];
		}
	}
}

impl<T: Copy + AddAssign, const N: usize> Add for Vector<T, N> {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		let mut out = self;
		out += rhs;
		out
	}
}

impl<T: Copy + MulAssign, const N: usize> MulAssign for Vector<T, N> {
	fn mul_assign(&mut self, rhs: Self) {
		for i in 0..N {
			self[i] *= rhs[i];
		}
	}
}

impl<T: Copy + MulAssign, const N: usize> Mul for Vector<T, N> {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		let mut out = self;
		out *= rhs;
		out
	}
}

// Arithmetic-agnostic implementations.
impl<T: Copy, const N: usize> Vector<T, N> {
	pub fn len(&self) -> usize {
		N
	}
}

impl<T: Copy + AddAssign, const N: usize> Vector<T, N> {
	pub fn sum(&self) -> T {
		let mut out = self[0];
		for i in 1..N {
			out += self[i];
		}
		out
	}
}

impl<T: Copy + MulAssign, const N: usize> Vector<T, N> {
	pub fn product(&self) -> T {
		let mut out = self[0];
		for i in 1..N {
			out *= self[i];
		}
		out
	}
}

impl<T, const N: usize> Vector<T, N>
where
	T: Copy + AddAssign + Mul<Output = T>,
{
	pub fn dot(&self, other: &Vector<T, N>) -> T {
		let mut out = self.v[0] * other.v[0];

		for i in 1..N {
			out += self.v[i] * other.v[i];
		}

		out
	}
}
