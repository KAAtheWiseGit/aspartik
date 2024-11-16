use std::ops::{Index, IndexMut};

mod bitvec;
use bitvec::BitVec;

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

	/// Appends the value as an accepted one.
	pub fn push(&mut self, value: T) {
		self.inner.push(value);
		self.inner.push(T::default());

		self.validity.push(0);
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

	pub fn last(&self) -> Option<&T> {
		if self.is_empty() {
			None
		} else {
			Some(&self[self.len() - 1])
		}
	}
}

impl<T: Default + Copy> ShchurVec<T> {
	pub fn repeat(value: T, length: usize) -> Self {
		let mut out = ShchurVec::with_capacity(length);

		for _ in 0..length {
			out.push(value);
		}

		out
	}
}

// Memoization-related methods
impl<T: Default> ShchurVec<T> {
	pub fn accept(&mut self) {
		for (i, bit) in self.validity.iter().enumerate() {
			if bit > 0 {
				self.inner[i * 2] = std::mem::take(
					&mut self.inner[i * 2 + 1],
				);
			}
		}
		self.validity.zero_out();
	}

	pub fn reject(&mut self) {
		self.validity.zero_out();
	}

	pub fn set(&mut self, index: usize, value: T) {
		self.inner[index * 2 + 1] = value;
		self.validity.set(index);
	}
}

impl<T: Default> Index<usize> for ShchurVec<T> {
	type Output = T;

	fn index(&self, index: usize) -> &T {
		&self.inner[index * 2 + self.validity.get(index) as usize]
	}
}

impl<T: Default> IndexMut<usize> for ShchurVec<T> {
	fn index_mut(&mut self, index: usize) -> &mut T {
		&mut self.inner[index * 2 + self.validity.get(index) as usize]
	}
}

pub struct Iter<'a, T: Default> {
	vec: &'a ShchurVec<T>,
	index: usize,
}

impl<'a, T: Default> Iterator for Iter<'a, T> {
	type Item = &'a T;

	fn next(&mut self) -> Option<&'a T> {
		if self.index == self.vec.len() {
			None
		} else {
			let out = &self.vec[self.index];
			self.index += 1;
			Some(out)
		}
	}
}

impl<T: Default> ShchurVec<T> {
	pub fn iter(&self) -> Iter<'_, T> {
		Iter {
			vec: self,
			index: 0,
		}
	}
}

impl<'a, T: Default> IntoIterator for &'a ShchurVec<T> {
	type Item = &'a T;
	type IntoIter = Iter<'a, T>;

	fn into_iter(self) -> Iter<'a, T> {
		self.iter()
	}
}

impl<T: Default + Clone> From<&[T]> for ShchurVec<T> {
	fn from(values: &[T]) -> Self {
		let mut out = Self::with_capacity(values.len());

		for value in values {
			out.push(value.clone());
		}

		out
	}
}
