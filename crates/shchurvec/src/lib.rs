use bitvec::prelude::*;

use std::ops::{Index, IndexMut};

// XXX: somewhat slower, but eliminates the need for juggling uninitialized
// memory.
#[derive(Debug, Clone, Default)]
pub struct ShchurVec<T: Default> {
	inner: Vec<T>,
	validity: BitVec,
}

// Methods from `Vec`.
impl<T: Default> ShchurVec<T> {
	pub fn new() -> Self {
		Self {
			inner: Vec::new(),
			validity: BitVec::new(),
		}
	}

	pub fn with_capacity(capacity: usize) -> Self {
		Self {
			inner: Vec::with_capacity(capacity * 2),
			validity: BitVec::with_capacity(capacity),
		}
	}

	pub fn capacity(&self) -> usize {
		self.inner.capacity() / 2
	}

	pub fn reserve(&mut self, additional: usize) {
		self.inner.reserve(additional * 2);
		self.validity.reserve(additional);
	}

	pub fn shrink_to_fit(&mut self) {
		self.inner.shrink_to_fit();
		self.validity.shrink_to_fit();
	}

	/// Appends the value as a proposed one.
	pub fn push(&mut self, value: T) {
		self.inner.push(T::default());
		self.inner.push(value);

		self.validity.push(true);
	}

	pub fn clear(&mut self) {
		self.inner.clear();
		self.validity.clear();
	}

	pub fn len(&self) -> usize {
		self.inner.len() / 2
	}

	pub fn is_empty(&self) -> bool {
		self.inner.is_empty()
	}
}

// Memoization-related methods
impl<T: Default> ShchurVec<T> {
	pub fn accept(&mut self) {
		for (i, bit) in self.validity.iter().enumerate() {
			if *bit {
				self.inner[i * 2] = std::mem::take(
					&mut self.inner[i * 2 + 1],
				);
			}
		}
	}

	pub fn reject(&mut self) {
		self.validity.set_elements(0);
	}

	pub fn set(&mut self, index: usize, value: T) {
		self.inner[index * 2 + 1] = value;
		self.validity.set(index, true);
	}
}

impl<T: Default> Index<usize> for ShchurVec<T> {
	type Output = T;

	fn index(&self, index: usize) -> &T {
		&self.inner[index + self.validity[index] as usize]
	}
}

impl<T: Default> IndexMut<usize> for ShchurVec<T> {
	fn index_mut(&mut self, index: usize) -> &mut T {
		&mut self.inner[index + self.validity[index] as usize]
	}
}
