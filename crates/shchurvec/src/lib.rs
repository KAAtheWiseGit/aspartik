use std::{mem::MaybeUninit, ops::Index};

#[derive(Debug)]
pub struct ShchurVec<T> {
	/// The actual storage.  It's twice as long as the number of items
	/// `ShchurVec` can hold at a time.  Each item takes up two elements in
	/// `inner`, only one of which is active, which is determined by `mask`
	/// elements.
	inner: Vec<MaybeUninit<T>>,
	/// True if the value had been edited.  It uses the `bool` type, which
	/// is guaranteed to be one byte:
	///
	/// https://doc.rust-lang.org/std/mem/fn.size_of.html#:~:text=bool
	/// https://github.com/rust-lang/rust/pull/46156
	edited: Vec<bool>,
	/// Mask points to the currently active item in `inner`.
	///
	/// # Safety
	///
	/// - Mask elements must have the values of either 0 or 1.
	/// - Mask elements must point at initialized memory.
	mask: Vec<u8>,
}

impl<T> Clone for ShchurVec<T>
where
	MaybeUninit<T>: Clone,
{
	fn clone(&self) -> Self {
		Self {
			inner: self.inner.clone(),
			edited: self.edited.clone(),
			mask: self.mask.clone(),
		}
	}
}

impl<T> Default for ShchurVec<T> {
	fn default() -> Self {
		Self::new()
	}
}

// Methods from `Vec`.
impl<T> ShchurVec<T> {
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
		(self.inner.capacity() / 2)
			.min(self.edited.capacity())
			.min(self.mask.capacity())
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
		self.inner.push(MaybeUninit::new(value));
		self.inner.push(MaybeUninit::uninit());

		self.edited.push(true);
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

impl<T: Copy> ShchurVec<T> {
	pub fn repeat(value: T, length: usize) -> Self {
		let mut out = ShchurVec::with_capacity(length);

		for _ in 0..length {
			out.push(value);
		}

		out
	}
}

// Memoization-related methods
impl<T> ShchurVec<T> {
	fn clear_edited(&mut self) {
		self.edited.iter_mut().for_each(|v| *v = false);
	}

	pub fn accept(&mut self) {
		self.clear_edited();
	}

	pub fn reject(&mut self) {
		// rollback edits
		for i in 0..self.len() {
			if self.edited[i] {
				self.mask[i] ^= 1;
			}
		}

		self.clear_edited();
	}

	pub fn set(&mut self, index: usize, value: T) {
		if !self.edited[index] {
			self.mask[index] ^= 1;
			self.edited[index] = true;
		}

		self.inner[index * 2 + self.mask[index] as usize] =
			MaybeUninit::new(value);
	}
}

impl<T> Index<usize> for ShchurVec<T> {
	type Output = T;

	fn index(&self, index: usize) -> &T {
		// SAFETY: TODO
		unsafe {
			self.inner[index * 2 + self.mask[index] as usize]
				.assume_init_ref()
		}
	}
}

pub struct Iter<'a, T> {
	vec: &'a ShchurVec<T>,
	index: usize,
}

impl<'a, T> Iterator for Iter<'a, T> {
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

impl<T> ShchurVec<T> {
	pub fn iter(&self) -> Iter<'_, T> {
		Iter {
			vec: self,
			index: 0,
		}
	}
}

impl<'a, T> IntoIterator for &'a ShchurVec<T> {
	type Item = &'a T;
	type IntoIter = Iter<'a, T>;

	fn into_iter(self) -> Iter<'a, T> {
		self.iter()
	}
}

impl<T: Clone> From<&[T]> for ShchurVec<T> {
	fn from(values: &[T]) -> Self {
		let mut out = Self::with_capacity(values.len());

		for value in values {
			out.push(value.clone());
		}

		out
	}
}
