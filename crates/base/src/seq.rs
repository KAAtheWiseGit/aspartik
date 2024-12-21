use anyhow::{Context, Error, Result};

use std::{
	fmt::Display,
	ops::{Index, IndexMut},
};

use crate::bases::DnaNucleoBase;

/// A character in a sequence alphabet.
///
/// # Safety
///
/// Must be identical to a `u8`.  `Into<u8>` must be a simple type cast, with no
/// layout changes.
pub trait Character:
	TryFrom<u8, Error = Error>
	+ TryFrom<char, Error = Error>
	+ Into<u8>
	+ Into<char>
	+ Copy
	+ std::fmt::Debug
	+ Eq
	+ std::hash::Hash
{
}

impl Character for DnaNucleoBase {}
pub type DnaSeq = Seq<DnaNucleoBase>;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct Seq<C: Character> {
	inner: Vec<C>,
}

impl<C: Character> Display for Seq<C> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut s = String::new();

		for item in &self.inner {
			s.push((*item).into());
		}

		f.write_str(&s)
	}
}

impl<C: Character> TryFrom<&str> for Seq<C> {
	type Error = Error;

	fn try_from(value: &str) -> Result<Self> {
		let mut out = Seq { inner: Vec::new() };

		for char in value.chars() {
			out.inner.push(char.try_into().with_context(|| {
				let width = out.len();
				format!("\n\tAn illegal character encountered in the sequence:\n> {}\n> {:width$}^", value, "")
			})?);
		}

		Ok(out)
	}
}

impl<C: Character> Index<usize> for Seq<C> {
	type Output = C;

	fn index(&self, index: usize) -> &Self::Output {
		&self.inner[index]
	}
}

impl<C: Character> IndexMut<usize> for Seq<C> {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		&mut self.inner[index]
	}
}

impl<'a, C: Character> IntoIterator for &'a Seq<C> {
	type Item = &'a C;
	type IntoIter = std::slice::Iter<'a, C>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

// Character-agnostic methods
impl<C: Character> Seq<C> {
	pub fn new() -> Self {
		Seq { inner: Vec::new() }
	}

	/// Reverses the characters in-place.
	pub fn reverse(&mut self) {
		self.inner.reverse();
	}

	pub fn append(&mut self, mut other: Self) {
		self.inner.append(&mut other.inner);
	}

	pub fn push(&mut self, character: C) {
		self.inner.push(character);
	}

	pub fn iter(&self) -> std::slice::Iter<'_, C> {
		self.inner.iter()
	}

	pub fn len(&self) -> usize {
		self.inner.len()
	}

	pub fn is_empty(&self) -> bool {
		self.inner.is_empty()
	}

	/// Counts how many times the character `c` occurs in the sequence.
	pub fn count(&self, c: C) -> usize {
		let mut out = 0;

		for current in self.iter().copied() {
			if current == c {
				out += 1
			}
		}

		out
	}
}

// DNA-specific methods
impl DnaSeq {
	/// Returns the sequence complement of `self`.  Note that this function
	/// doesn't reverse the direction of the sequence, use
	/// [`reverse_complement`][`DnaSeq::reverse_complement`] for that.
	pub fn complement(&self) -> Self {
		let mut out = self.clone();
		for base in out.inner.iter_mut() {
			*base = base.complement();
		}
		out
	}

	pub fn reverse_complement(&self) -> Self {
		let mut out = self.complement();
		out.reverse();

		out
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn decode() {
		let s = "ACTGxACTG";
		let seq: Result<Seq<DnaNucleoBase>> = s.try_into();
		assert!(seq.is_err());
	}

	#[test]
	fn count() {
		let s: DnaSeq = "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC".try_into().unwrap();

		assert_eq!(s.count(DnaNucleoBase::Adenine), 20);
		assert_eq!(s.count(DnaNucleoBase::Cytosine), 12);
		assert_eq!(s.count(DnaNucleoBase::Guanine), 17);
		assert_eq!(s.count(DnaNucleoBase::Thymine), 21);
	}

	#[test]
	fn dna_complement() {
		let s: DnaSeq = "AAAACCCGGT".try_into().unwrap();

		assert_eq!(s.reverse_complement().to_string(), "ACCGGGTTTT");
	}
}
