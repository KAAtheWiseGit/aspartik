use rand::Rng;

use std::collections::HashMap;

use super::{Operator, Proposal, Rng as RngT, Status, TreeEdit};
use crate::state::State;

pub struct Slide {}

impl Operator for Slide {
	fn propose(&self, state: &State, rng: &mut RngT) -> Proposal {
		let tree = state.get_tree();

		let node = tree.sample_internal(rng);
		let Some(parent) = tree.parent_of(node) else {
			return Proposal::reject();
		};
		let (left, right) = tree.children_of(node);

		let high = tree.weight_of(parent);
		// maximum of two child weights
		let low = tree.weight_of(left).max(tree.weight_of(right));

		// TODO: what happens if `new_weight == low`?
		let new_weight = rng.gen_range(low..high);

		Proposal {
			status: Status::Hastings(0.0),
			params: HashMap::new(),
			tree: TreeEdit {
				weights: vec![(node.into(), new_weight)],
				spr: None,
			},
		}
	}
}
