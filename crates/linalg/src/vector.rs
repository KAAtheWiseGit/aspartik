use num_traits::{Float, Num, NumAssign};

use std::{
	fmt::{self, Display},
	ops::{
		Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign,
	},
};

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
#[repr(C)]
pub struct Vector<T: Copy, const N: usize> {
	v: [T; N],
}

#[cfg(feature = "bytemuck")]
mod bytemuck {
	use super::Vector;
	use bytemuck::{Pod, Zeroable};

	unsafe impl<T, const N: usize> Zeroable for Vector<T, N> where T: Copy + Zeroable
	{}

	unsafe impl<T, const N: usize> Pod for Vector<T, N> where T: Copy + Pod {}
}

// `From` conversions
impl<T: Copy, const N: usize> From<[T; N]> for Vector<T, N> {
	fn from(value: [T; N]) -> Self {
		Self { v: value }
	}
}

impl<T: Copy, const N: usize> Vector<T, N> {
	fn from_element(value: T) -> Self {
		[value; N].into()
	}
}

impl<T: Copy + Display, const N: usize> Display for Vector<T, N> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str("[")?;

		for i in 0..N {
			self[i].fmt(f)?;
			if i != N - 1 {
				f.write_str(", ")?;
			}
		}

		f.write_str("]")?;

		Ok(())
	}
}

// This can be derived, but then every single trait impl would have to carry a
// `Default` constraint around, which is verbose.
impl<T: Copy + Default, const N: usize> Default for Vector<T, N> {
	fn default() -> Self {
		[T::default(); N].into()
	}
}

// Mathematical constructors
impl<T: Copy + Num, const N: usize> Vector<T, N> {
	pub fn zeros() -> Self {
		Self::from_element(T::zero())
	}

	pub fn ones() -> Self {
		Self::from_element(T::one())
	}

	/// A standard basis vector: all elements are zero, except the one at
	/// index `i`, which is set to one.
	pub fn sbv(i: usize) -> Self {
		let mut out = Self::zeros();
		out[i] = T::one();
		out
	}
}

// Operators and overloading in general.
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

	fn mul(mut self, rhs: Self) -> Self::Output {
		self *= rhs;
		self
	}
}

impl<T: Copy + MulAssign, const N: usize> MulAssign<T> for Vector<T, N> {
	fn mul_assign(&mut self, rhs: T) {
		for i in 0..N {
			self[i] *= rhs;
		}
	}
}

impl<T: Copy + MulAssign, const N: usize> Mul<T> for Vector<T, N> {
	type Output = Self;

	fn mul(mut self, rhs: T) -> Self::Output {
		for i in 0..N {
			self[i] *= rhs;
		}
		self
	}
}

impl<T: Copy + DivAssign, const N: usize> DivAssign for Vector<T, N> {
	fn div_assign(&mut self, rhs: Self) {
		for i in 0..N {
			self[i] /= rhs[i];
		}
	}
}

impl<T: Copy + DivAssign, const N: usize> Div for Vector<T, N> {
	type Output = Self;

	fn div(mut self, rhs: Self) -> Self::Output {
		self /= rhs;
		self
	}
}

impl<T: Copy + DivAssign, const N: usize> DivAssign<T> for Vector<T, N> {
	fn div_assign(&mut self, rhs: T) {
		for i in 0..N {
			self[i] /= rhs;
		}
	}
}

impl<T: Copy + DivAssign, const N: usize> Div<T> for Vector<T, N> {
	type Output = Self;

	fn div(mut self, rhs: T) -> Self::Output {
		for i in 0..N {
			self[i] /= rhs;
		}
		self
	}
}

// Type-agnostic implementations.
impl<T: Copy, const N: usize> Vector<T, N> {
	const LENGTH: usize = N;

	pub fn as_ptr(&self) -> *const T {
		self.v.as_ptr()
	}

	pub fn as_mut_ptr(&mut self) -> *mut T {
		self.v.as_mut_ptr()
	}

	pub fn apply<F>(&mut self, f: F)
	where
		F: Fn(&mut T),
	{
		self.v.each_mut().map(f);
	}

	pub fn map<F, U>(&self, f: F) -> Vector<U, N>
	where
		U: Copy,
		F: Fn(T) -> U,
	{
		self.v.map(f).into()
	}

	pub fn truncate<const M: usize>(&self) -> Vector<T, M> {
		assert!(M < N, "New length must be smaller");

		let subslice: &[T; M] = self.v.first_chunk().unwrap();

		Vector::from(*subslice)
	}
}

// Numeric methods.
impl<T: Copy + NumAssign, const N: usize> Vector<T, N> {
	pub fn sum(&self) -> T {
		let mut out = self[0];
		for i in 1..N {
			out += self[i];
		}
		out
	}

	pub fn product(&self) -> T {
		let mut out = self[0];
		for i in 1..N {
			out *= self[i];
		}
		out
	}

	pub fn dot(&self, other: &Vector<T, N>) -> T {
		let mut out = self[0] * other[0];

		for i in 1..N {
			out += self[i] * other[i];
		}

		out
	}
}

impl<T: Copy + Float + NumAssign, const N: usize> Vector<T, N> {
	pub fn norm(&self) -> T {
		let mut out = self[0] * self[0];

		for i in 1..N {
			out += self[i] * self[i];
		}

		out.sqrt()
	}

	pub fn normalize(&self) -> Self {
		*self / self.norm()
	}
}
