use super::{RowMatrix, Vector};
use bytemuck::{Pod, Zeroable};

unsafe impl<T, const N: usize> Zeroable for Vector<T, N> where T: Copy + Zeroable
{}
unsafe impl<T, const N: usize> Pod for Vector<T, N> where T: Copy + Pod {}

unsafe impl<T, const N: usize, const M: usize> Zeroable for RowMatrix<T, N, M> where
	T: Copy + Zeroable
{
}
unsafe impl<T, const N: usize, const M: usize> Pod for RowMatrix<T, N, M> where
	T: Copy + Pod
{
}
