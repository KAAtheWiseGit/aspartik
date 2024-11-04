use crate::bases::DnaNucleoBase;

pub(crate) trait Sealed:
	TryFrom<u8>
	+ TryFrom<char>
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

impl Seq<DnaNucleoBase> {
	fn complement(&self) -> Self {
		let mut out = self.clone();
		for base in out.value.iter_mut() {
			*base = base.complement();
		}
		out
	}
}
