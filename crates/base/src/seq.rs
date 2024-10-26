use crate::bases::Base;

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
