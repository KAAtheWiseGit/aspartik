use std::fmt::Display;

use crate::{bases::DnaNucleoBase, Error, Result};

pub(crate) trait Sealed:
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

impl Sealed for DnaNucleoBase {}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Seq<T: Sealed> {
	value: Vec<T>,
}

impl<T: Sealed> Seq<T> {
	pub fn reverse(&self) -> Self {
		let mut out = self.clone();
		out.value.reverse();
		out
	}
}

impl<T: Sealed> Display for Seq<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut s = String::new();

		for item in &self.value {
			s.push((*item).into());
		}

		f.write_str(&s)
	}
}

impl<T: Sealed> TryFrom<&str> for Seq<T> {
	type Error = Error;

	fn try_from(value: &str) -> Result<Self> {
		let mut out = Seq { value: Vec::new() };

		for char in value.chars() {
			out.value.push(char.try_into()?);
		}

		Ok(out)
	}
}

impl Seq<DnaNucleoBase> {
	fn complement(&self) -> Self {
		let mut out = self.clone();
		for base in out.value.iter_mut() {
			*base = base.complement();
		}
		out
	}
}
