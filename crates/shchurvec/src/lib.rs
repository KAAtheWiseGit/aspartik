use bitvec::prelude::*;

#[derive(Debug, Clone)]
pub struct ShchurVec<T> {
	inner: Vec<T>,
	validity: BitVec,
}

// XXX: somewhat slower, but eliminates the need for juggling uninitialized
// memory.
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

impl<T: Default> Default for ShchurVec<T> {
	fn default() -> Self {
		Self::new()
	}
}
