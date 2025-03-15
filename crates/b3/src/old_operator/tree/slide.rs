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
		let tree = &mut state.tree;

		let node = tree.random_internal(&mut state.rng);
		let Some(parent) = tree.parent_of(node.into()) else {
			// If the node is root, abort the proposal
			return Ok(Proposal::Reject);
		};
		let (left, right) = tree.children_of(node);

		let weight = tree.weight_of(node.into());
		let low = tree.weight_of(parent.into());
		// maximum of two child weights
		let high = tree.weight_of(left).min(tree.weight_of(right));

		let new_weight = self
			.distribution
			.random_range_with(low, high, weight, state)?;

		let tree = &mut state.tree;

		tree.update_weight(node.into(), new_weight);
		Ok(Proposal::Hastings(0.0))
	}
}
