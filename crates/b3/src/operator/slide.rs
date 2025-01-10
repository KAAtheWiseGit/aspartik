use super::{Operator, Proposal};
use crate::{distribution::Distribution, State};

pub struct Slide {
	distribution: Distribution,
}

impl Slide {
	pub fn new(distribution: Distribution) -> Box<dyn Operator> {
		Box::new(Self { distribution })
	}
}

impl Operator for Slide {
	fn propose(&self, state: &mut State) -> Proposal {
		let rng = &mut state.rng;
		let tree = &state.tree;

		let node = tree.sample_internal(rng);
		let Some(parent) = tree.parent_of(node) else {
			return Proposal::reject();
		};
		let (left, right) = tree.children_of(node);

		let weight = tree.weight_of(node);
		let high = tree.weight_of(parent);
		// maximum of two child weights
		let low = tree.weight_of(left).max(tree.weight_of(right));

		let new_weight = self
			.distribution
			.gen_range_from(low, high, weight, rng);

		Proposal::hastings(0.0)
			.with_weights(vec![(node.into(), new_weight)])
	}
}
