#[derive(Debug, Clone, Default)]
pub struct BitVec {
	length: usize,
	inner: Vec<u8>,
}

impl BitVec {
	pub fn new() -> Self {
		Self {
			inner: Vec::new(),
			length: 0,
		}
	}

	pub fn with_capacity(capacity: usize) -> Self {
		Self {
			inner: Vec::with_capacity(capacity / 8 + 1),
			length: 0,
		}
	}

	pub fn capacity(&self) -> usize {
		self.inner.capacity() * 8
	}

	pub fn reserve(&mut self, additional: usize) {
		self.inner.reserve(additional / 8 + 1);
	}

	pub fn shrink_to_fit(&mut self) {
		self.inner.shrink_to_fit();
	}

	pub fn push(&mut self, value: u8) {
		self.length += 1;
		if (self.length + 8) / 8 > self.inner.len() {
			self.inner.push(0);
		}

		if (value & 1) > 0 {
			self.set(self.length - 1);
		}
	}

	pub fn clear(&mut self) {
		self.length = 0;
		self.inner.clear();
	}

	pub fn len(&self) -> usize {
		self.length
	}

	pub fn is_empty(&self) -> bool {
		self.length == 0
	}
}

impl BitVec {
	pub fn get(&self, index: usize) -> u8 {
		let byte_i = index / 8;
		let shift = index % 8;

		(self.inner[byte_i] >> shift) & 1
	}

	pub fn set(&mut self, index: usize) {
		let byte_i = index / 8;
		let shift = index % 8;

		self.inner[byte_i] |= 1 << shift;
	}

	pub fn unset(&mut self, index: usize) {
		let byte_i = index / 8;
		let shift = index % 8;

		self.inner[byte_i] &= !(1 << shift);
	}

	pub fn zero_out(&mut self) {
		self.inner.iter_mut().for_each(|b| *b = 0);
	}
}

pub struct Iter<'a> {
	vec: &'a BitVec,
	index: usize,
}

impl<'a> Iterator for Iter<'a> {
	type Item = u8;

	fn next(&mut self) -> Option<u8> {
		if self.index == self.vec.len() {
			None
		} else {
			let out = self.vec.get(self.index);
			self.index += 1;
			Some(out)
		}
	}
}

impl BitVec {
	pub fn iter(&self) -> Iter<'_> {
		Iter {
			vec: self,
			index: 0,
		}
	}
}

impl<'a> IntoIterator for &'a BitVec {
	type Item = u8;
	type IntoIter = Iter<'a>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn push() {
		let mut bitvec = BitVec::new();

		// Returns 0, 1, 1 in a loop
		fn value(i: usize) -> u8 {
			(i % 3).clamp(0, 1) as u8
		}

		for i in 0..100 {
			bitvec.push(value(i));
		}

		for i in 0..100 {
			assert_eq!(value(i), bitvec.get(i));
		}
	}

	#[test]
	fn unset() {
		let mut bitvec = BitVec::new();
		for _ in 0..100 {
			bitvec.push(1);
		}
		bitvec.unset(10);
		assert_eq!(0, bitvec.get(10));
	}
}
