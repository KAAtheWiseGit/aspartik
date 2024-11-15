use std::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign};

use crate::vector::Vector;

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

impl<T: Copy + MulAssign, const N: usize, const M: usize> MulAssign
	for RowMatrix<T, N, M>
{
	fn mul_assign(&mut self, rhs: Self) {
		for i in 0..M {
			self[i] *= rhs[i];
		}
	}
}

impl<T: Copy + MulAssign, const N: usize, const M: usize> Mul
	for RowMatrix<T, N, M>
{
	type Output = Self;

	fn mul(mut self, rhs: Self) -> Self {
		for i in 0..M {
			self[i] *= rhs[i];
		}
		self
	}
}
