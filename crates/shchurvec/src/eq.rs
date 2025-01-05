use std::cmp::PartialEq;

use crate::ShchurVec;

macro_rules! impl_inside {
	($self:ident, $other:ident) => {{
		if $self.len() != $other.len() {
			return false;
		}

		for (a, b) in $self.iter().zip($other.iter()) {
			if a != b {
				return false;
			}
		}

		true
	}};
}

impl<T: PartialEq> PartialEq for ShchurVec<T> {
	fn eq(&self, other: &Self) -> bool {
		impl_inside!(self, other)
	}
}

impl<T: PartialEq> PartialEq<[T]> for ShchurVec<T> {
	fn eq(&self, other: &[T]) -> bool {
		impl_inside!(self, other)
	}
}

impl<T: PartialEq, const N: usize> PartialEq<[T; N]> for ShchurVec<T> {
	fn eq(&self, other: &[T; N]) -> bool {
		impl_inside!(self, other)
	}
}

impl<T: PartialEq> PartialEq<Vec<T>> for ShchurVec<T> {
	fn eq(&self, other: &Vec<T>) -> bool {
		impl_inside!(self, other)
	}
}

impl<T: PartialEq> PartialEq<ShchurVec<T>> for [T] {
	fn eq(&self, other: &ShchurVec<T>) -> bool {
		impl_inside!(self, other)
	}
}

impl<T: PartialEq, const N: usize> PartialEq<ShchurVec<T>> for [T; N] {
	fn eq(&self, other: &ShchurVec<T>) -> bool {
		impl_inside!(self, other)
	}
}

impl<T: PartialEq> PartialEq<ShchurVec<T>> for Vec<T> {
	fn eq(&self, other: &ShchurVec<T>) -> bool {
		impl_inside!(self, other)
	}
}
