use std::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign};

use crate::vector::Vector;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct RowMatrix<T: Copy, const N: usize, const M: usize> {
	m: [Vector<T, N>; M],
}

impl<T: Copy, const N: usize, const M: usize> From<[Vector<T, N>; M]>
	for RowMatrix<T, N, M>
{
	fn from(value: [Vector<T, N>; M]) -> Self {
		RowMatrix { m: value }
	}
}

impl<T: Copy, const N: usize, const M: usize> From<[[T; N]; M]>
	for RowMatrix<T, N, M>
{
	fn from(value: [[T; N]; M]) -> Self {
		value.map(|row| -> Vector<T, N> { row.into() }).into()
	}
}

impl<T: Copy, const N: usize, const M: usize> From<T> for RowMatrix<T, N, M> {
	fn from(value: T) -> Self {
		[Vector::from([value; N]); M].into()
	}
}

impl<T: Copy + Default, const N: usize, const M: usize> Default
	for RowMatrix<T, N, M>
{
	fn default() -> Self {
		Self::from(T::default())
	}
}

impl<T: Copy, const N: usize, const M: usize> Index<usize>
	for RowMatrix<T, N, M>
{
	type Output = Vector<T, N>;

	fn index(&self, i: usize) -> &Vector<T, N> {
		&self.m[i]
	}
}

impl<T: Copy, const N: usize, const M: usize> IndexMut<usize>
	for RowMatrix<T, N, M>
{
	fn index_mut(&mut self, i: usize) -> &mut Vector<T, N> {
		&mut self.m[i]
	}
}

impl<T: Copy, const N: usize, const M: usize> Index<(usize, usize)>
	for RowMatrix<T, N, M>
{
	type Output = T;

	fn index(&self, (i, j): (usize, usize)) -> &T {
		&self.m[i][j]
	}
}

impl<T: Copy, const N: usize, const M: usize> IndexMut<(usize, usize)>
	for RowMatrix<T, N, M>
{
	fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut T {
		&mut self.m[i][j]
	}
}

impl<T: Copy + AddAssign, const N: usize, const M: usize> AddAssign
	for RowMatrix<T, N, M>
{
	fn add_assign(&mut self, rhs: Self) {
		for i in 0..M {
			self[i] += rhs[i];
		}
	}
}

impl<T: Copy + AddAssign, const N: usize, const M: usize> Add
	for RowMatrix<T, N, M>
{
	type Output = Self;

	fn add(mut self, rhs: Self) -> Self {
		for i in 0..M {
			self[i] += rhs[i];
		}
		self
	}
}

impl<T: Copy + MulAssign, const N: usize, const M: usize> RowMatrix<T, N, M> {
	fn component_mul_assign(&mut self, rhs: Self) {
		for i in 0..M {
			self[i] *= rhs[i];
		}
	}

	fn component_mul(mut self, rhs: Self) -> Self {
		for i in 0..M {
			self[i] *= rhs[i];
		}
		self
	}
}

impl<T: Copy + AddAssign, const N: usize, const M: usize> RowMatrix<T, N, M> {
	pub fn trace(&self) -> T {
		let mut out = self[(0, 0)];
		for i in 1..N {
			out += self[(i, i)];
		}
		out
	}
}

impl<T, const N: usize, const M: usize> Mul<Vector<T, N>> for RowMatrix<T, N, M>
where
	T: Copy + AddAssign + MulAssign + Default,
{
	type Output = Vector<T, M>;

	fn mul(self, rhs: Vector<T, N>) -> Vector<T, M> {
		// TODO: uninitialized
		let mut out = Vector::default();

		for i in 0..M {
			out[i] = (self[i] * rhs).sum();
		}

		out
	}
}

impl<T: Copy, const N: usize, const M: usize> RowMatrix<T, N, M> {
	const NUM_ROWS: usize = N;
	const NUM_COLUMNS: usize = M;
	const NUM_ITEMS: usize = N * M;
	const IS_SQUARE: bool = N == M;
}

impl<T, const N: usize, const M: usize, const P: usize> Mul<RowMatrix<T, M, P>>
	for RowMatrix<T, N, M>
where
	T: Copy + AddAssign + Mul<Output = T> + Default,
{
	type Output = RowMatrix<T, N, P>;

	// This is a suboptimal algorithm.  There's a more cache-friendly one,
	// but it requires calculating a bunch of things, including a square
	// root.  I implement both, compare the assembly output, and benchmark.
	//
	// https://en.wikipedia.org/wiki/Matrix_multiplication_algorithm
	fn mul(self, rhs: RowMatrix<T, M, P>) -> Self::Output {
		let mut out = RowMatrix::default();

		for i in 0..N {
			for j in 0..P {
				for k in 0..M {
					out[(i, j)] +=
						self[(i, k)] * rhs[(k, j)];
				}
			}
		}

		out
	}
}
