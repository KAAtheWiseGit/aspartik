use std::{mem::MaybeUninit, ops::Index};

#[derive(Debug)]
pub struct ShchurVec<T> {
	/// The actual storage.  It's twice as long as the number of items
	/// `ShchurVec` can hold at a time.  Each item takes up two values in
	/// `inner`, only one of which is active, determined by the `mask` at
	/// the index.
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
	/// - Masks must have the values of either 0 or 1.
	///
	/// - Masks must point at initialized memory.  When an value pair is
	///   created for the first time, the 0th one is presumed to be
	///   initialized.  This can change after an `accept` call.
	mask: Vec<u8>,
}

// Memoization-related methods
impl<T> ShchurVec<T> {
	fn active_inner(&self, i: usize) -> &MaybeUninit<T> {
		&self.inner[i * 2 + self.mask[i] as usize]
	}

	/// The slot at index `i` pointed at by the mask.
	fn active_inner_mut(&mut self, i: usize) -> &mut MaybeUninit<T> {
		&mut self.inner[i * 2 + self.mask[i] as usize]
	}

	/// The other slot at index `i`, which is not being pointed to by the
	/// mask.
	fn inactive_inner_mut(&mut self, i: usize) -> &mut MaybeUninit<T> {
		&mut self.inner[i * 2 + (self.mask[i] ^ 1) as usize]
	}

	fn clear_edited(&mut self) {
		self.edited.iter_mut().for_each(|v| *v = false);
	}

	pub fn accept(&mut self) {
		// Don't waste time dropping values which don't need it.
		if std::mem::needs_drop::<T>() {
			for i in 0..self.len() {
				if self.edited[i] {
					// SAFETY: Only initialized values can
					// be edited.  Since the value at index
					// `i` has been set, it must've been
					// initialized.
					unsafe {
						// drop the old value
						self.inactive_inner_mut(i)
							.assume_init_drop();
					}
				}
			}
		}

		self.clear_edited();
	}

	pub fn reject(&mut self) {
		if std::mem::needs_drop::<T>() {
			for i in 0..self.len() {
				if self.edited[i] {
					// SAFETY: only initialized values can
					// be set.  Since the value at index `i`
					// was set, it must've been initialized.
					unsafe {
						// drop the edited value
						self.active_inner_mut(i)
							.assume_init_drop();
					}
				}
			}
		}

		for i in 0..self.len() {
			if self.edited[i] {
				// Point back to the old value.
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

	pub fn unset(&mut self, index: usize) {
		if self.edited[index] {
			if std::mem::needs_drop::<T>() {
				// SAFETY: because `edited[index]` is true, it
				// must've been set before.
				unsafe {
					self.active_inner_mut(index)
						.assume_init_drop();
				}
			}

			self.edited[index] = false;
			self.mask[index] ^= 1;
		}
	}
}

// Trait implementations

impl<T> Drop for ShchurVec<T> {
	/// Necessary because `MaybeUninit` doesn't drop on deinitialization.
	fn drop(&mut self) {
		// Make sure that we only have one initialized value per index.
		// Accept is used because it is faster than reject.
		self.accept();

		for i in 0..self.len() {
			// SAFETY: masks must always point to initialized
			// values.
			unsafe {
				self.active_inner_mut(i).assume_init_drop();
			}
		}
	}
}

impl<T> Index<usize> for ShchurVec<T> {
	type Output = T;

	fn index(&self, index: usize) -> &T {
		// SAFETY:
		//
		// - When a value is added to the vector for the first time,
		//   it's initialized and mask points at it.
		//
		// - When a value is set, mask is moved to point to that
		//   initialized value.
		//
		// - During `accept` and `reject` the invariant of `mask`
		//   pointing to the initialized values should be preserved.
		//
		// All of that means that this is sound, as long as mutating
		// methods, constructors, `accept`, `reject`, `set`, and in
		// general all of the methods which mutate the vector are sound
		// and uphold the invariants.
		unsafe { self.active_inner(index).assume_init_ref() }
	}
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

// Iterator implementations

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

		self.edited.push(false);
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

// Custom
impl<T: Copy> ShchurVec<T> {
	pub fn repeat(value: T, length: usize) -> Self {
		let mut out = ShchurVec::with_capacity(length);

		for _ in 0..length {
			out.push(value);
		}

		out
	}
}

// From implementations

impl<T: Clone> From<&[T]> for ShchurVec<T> {
	fn from(values: &[T]) -> Self {
		let mut out = Self::with_capacity(values.len());

		for value in values {
			out.push(value.clone());
		}

		out
	}
}
