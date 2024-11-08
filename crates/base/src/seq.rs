use std::{
	fmt::Display,
	ops::{Index, IndexMut},
};

use crate::{bases::DnaNucleoBase, Error, Result};

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
pub struct Seq<T: Character> {
	value: Vec<T>,
}

impl<T: Character> Seq<T> {
	pub fn new() -> Self {
		Seq { value: Vec::new() }
	}

	pub fn reverse(&self) -> Self {
		let mut out = self.clone();
		out.value.reverse();
		out
	}

	pub fn append(&mut self, mut other: Self) {
		self.value.append(&mut other.value);
	}

	pub fn iter(&self) -> std::slice::Iter<'_, T> {
		self.value.iter()
	}

	pub fn length(&self) -> usize {
		self.value.len()
	}
}

impl<T: Character> Display for Seq<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut s = String::new();

		for item in &self.value {
			s.push((*item).into());
		}

		f.write_str(&s)
	}
}

impl<T: Character> TryFrom<&str> for Seq<T> {
	type Error = Error;

	fn try_from(value: &str) -> Result<Self> {
		let mut out = Seq { value: Vec::new() };

		for char in value.chars() {
			out.value.push(char.try_into()?);
		}

		Ok(out)
	}
}

impl<T: Character> Index<usize> for Seq<T> {
	type Output = T;

	fn index(&self, index: usize) -> &Self::Output {
		&self.value[index]
	}
}

impl<T: Character> IndexMut<usize> for Seq<T> {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		&mut self.value[index]
	}
}

impl DnaSeq {
	fn complement(&self) -> Self {
		let mut out = self.clone();
		for base in out.value.iter_mut() {
			*base = base.complement();
		}
		out
	}
}
