use std::ops::{Index, IndexMut};

// XXX: somewhat slower, but eliminates the need for juggling uninitialized
// memory.
#[derive(Debug, Clone, Default)]
pub struct ShchurVec<T: Default> {
	inner: Vec<T>,
	edited: Vec<u8>,
	mask: Vec<u8>,
}

// Methods from `Vec`.
impl<T: Default> ShchurVec<T> {
	pub fn new() -> Self {
		Self {
			inner: Vec::new(),
			edited: Vec::new(),
			mask: Vec::new(),
		}
	}

	pub fn with_capacity(capacity: usize) -> Self {
		Self {
			inner: Vec::with_capacity(capacity * 2),
			edited: Vec::with_capacity(capacity),
			mask: Vec::with_capacity(capacity),
		}
	}

	pub fn capacity(&self) -> usize {
		self.mask.capacity()
	}

	pub fn reserve(&mut self, additional: usize) {
		self.inner.reserve(additional * 2);
		self.edited.reserve(additional);
		self.mask.reserve(additional);
	}

	pub fn shrink_to_fit(&mut self) {
		self.inner.shrink_to_fit();
		self.edited.shrink_to_fit();
		self.mask.shrink_to_fit();
	}

	/// Appends the value as an accepted one.
	pub fn push(&mut self, value: T) {
		self.inner.push(value);
		self.inner.push(T::default());

		self.edited.push(0);
		self.mask.push(0);
	}

	pub fn clear(&mut self) {
		self.inner.clear();
		self.edited.clear();
		self.mask.clear();
	}

	pub fn len(&self) -> usize {
		self.mask.len()
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
		self.edited.iter_mut().for_each(|v| *v = 0);
	}

	pub fn reject(&mut self) {
		// rollback edits
		for i in 0..self.len() {
			if self.edited[i] == 1 {
				self.mask[i] ^= 1;
			}
		}

		self.edited.iter_mut().for_each(|v| *v = 0);
	}

	pub fn set(&mut self, index: usize, value: T) {
		self.mask[index] ^= 1;
		self.inner[index * 2 + self.mask[index] as usize] = value;
		self.edited[index] = 1;
	}
}

impl<T: Default> Index<usize> for ShchurVec<T> {
	type Output = T;

	fn index(&self, index: usize) -> &T {
		&self.inner[index * 2 + self.mask[index] as usize]
	}
}

impl<T: Default> IndexMut<usize> for ShchurVec<T> {
	fn index_mut(&mut self, index: usize) -> &mut T {
		&mut self.inner[index * 2 + self.mask[index] as usize]
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
