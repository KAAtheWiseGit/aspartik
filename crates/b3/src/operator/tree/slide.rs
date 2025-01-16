use anyhow::Result;

use crate::operator::{Operator, Proposal};
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
	fn propose(&self, state: &mut State) -> Result<Proposal> {
		let rng = &mut state.rng;
		let tree = &mut state.tree;

		let node = tree.sample_internal(rng);
		let Some(parent) = tree.parent_of(node) else {
			return Ok(Proposal::Reject);
		};
		let (left, right) = tree.children_of(node);

		let weight = tree.weight_of(node);
		let low = tree.weight_of(parent);
		// maximum of two child weights
		let high = tree.weight_of(left).max(tree.weight_of(right));

		let new_weight = self
			.distribution
			.gen_range_from(low, high, weight, rng);

		tree.update_weight(node.into(), new_weight);
		Ok(Proposal::Hastings(0.0))
	}
}
