use super::Probability;
use crate::state::State;

struct Compound {
	probabilities: Vec<Box<dyn Probability>>,
}

impl Compound {
	pub fn new<I>(probabilities: I) -> Self
	where
		I: IntoIterator<Item = Box<dyn Probability>>,
	{
		Self {
			probabilities: probabilities.into_iter().collect(),
		}
	}
}

impl Probability for Compound {
	fn probability(&self, state: &State) -> f64 {
		self.probabilities
			.iter()
			.map(|p| p.probability(state))
			// This returns 1 if the iterator is empty
			// https://doc.rust-lang.org/src/core/iter/traits/accum.rs.html#114-123
			.product()
	}
}
