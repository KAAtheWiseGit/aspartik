use ahash::RandomState;

use std::{collections::HashMap, ops::Index};

#[derive(Debug, Clone)]
pub struct MemoVec<T> {
	values: Box<[T]>,
	update: HashMap<usize, T, RandomState>,
}

impl<T: Copy> From<&[T]> for MemoVec<T> {
	fn from(value: &[T]) -> Self {
		Self {
			values: value.into(),
			update: HashMap::default(),
		}
	}
}

impl<T: Copy> MemoVec<T> {
	pub fn new(value: T, length: usize) -> Self {
		let values = vec![value; length];

		Self {
			values: values.into(),
			update: HashMap::default(),
		}
	}

	pub fn accept(&mut self) {
		for (index, value) in std::mem::take(&mut self.update) {
			self.values[index] = value;
		}
	}

	pub fn reject(&mut self) {
		self.update.clear();
	}

	pub fn set(&mut self, index: usize, value: T) {
		self.update.insert(index, value);
	}

	pub fn slice(&self) -> &[T] {
		&self.values
	}
}

impl<T: Copy> MemoVec<T> {
	pub fn len(&self) -> usize {
		self.values.len()
	}

	pub fn last(&self) -> T {
		self[self.len() - 1]
	}
}

pub struct Iter<'a, T> {
	vec: &'a MemoVec<T>,
	index: usize,
}

impl<'a, T: Copy> Iterator for Iter<'a, T> {
	type Item = T;

	fn next(&mut self) -> Option<T> {
		if self.index == self.vec.len() {
			None
		} else {
			let out = self.vec[self.index];
			self.index += 1;
			Some(out)
		}
	}
}

impl<T: Copy> MemoVec<T> {
	pub fn iter(&self) -> Iter<'_, T> {
		Iter {
			vec: self,
			index: 0,
		}
	}
}

impl<'a, T: Copy> IntoIterator for &'a MemoVec<T> {
	type Item = T;
	type IntoIter = Iter<'a, T>;

	fn into_iter(self) -> Iter<'a, T> {
		self.iter()
	}
}

impl<T: Copy> Index<usize> for MemoVec<T> {
	type Output = T;

	fn index(&self, index: usize) -> &T {
		if let Some(value) = self.update.get(&index) {
			value
		} else {
			&self.values[index]
		}
	}
}
