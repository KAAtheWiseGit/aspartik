use rand::distributions::{Distribution, WeightedIndex};

use super::{Operator, Rng as RngT};

pub struct WeightedScheduler {
	operators: Vec<Box<dyn Operator>>,
	weights: Vec<f64>,
}

impl WeightedScheduler {
	pub fn new<I, J>(operators: I, weights: J) -> Self
	where
		I: IntoIterator<Item = Box<dyn Operator>>,
		J: IntoIterator<Item = f64>,
	{
		Self {
			operators: operators.into_iter().collect(),
			weights: weights.into_iter().collect(),
		}
	}

	pub fn get_operator(&mut self, rng: &mut RngT) -> &dyn Operator {
		let dist = WeightedIndex::new(&self.weights)
			.expect("Failed to pick a random index");

		let index = dist.sample(rng);

		Box::as_ref(&self.operators[index])
	}
}
