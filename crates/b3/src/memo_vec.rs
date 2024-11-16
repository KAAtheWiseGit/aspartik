#![allow(dead_code)]

use std::{collections::HashMap, ops::Index};

pub struct MemoVec<T> {
	values: Box<[T]>,
	update: HashMap<usize, T>,
}

impl<T: Copy> MemoVec<T> {
	pub fn new(value: T, length: usize) -> Self {
		let values = vec![value; length];

		Self {
			values: values.into(),
			update: HashMap::new(),
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
