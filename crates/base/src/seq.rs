use crate::bases::{Base, DnaNucleoBase};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Seq<T: Base> {
	value: Vec<T>,
}

impl<T: Base> Seq<T> {
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
