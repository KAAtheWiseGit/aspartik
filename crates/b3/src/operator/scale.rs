use rand::Rng;

use std::collections::HashMap;

use super::{Operator, Proposal, Rng as RngT, Status, TreeEdit};
use crate::state::State;

pub struct Scale();

impl Operator for Scale {
	fn propose(&self, state: &State, rng: &mut RngT) -> Proposal {
		let tree = state.get_tree();
		let factor = rng.gen_range(0.8..1.25);

		let mut out = Proposal {
			status: Status::Hastings(0.0),
			params: HashMap::new(),
			tree: TreeEdit::default(),
		};

		for node in tree.nodes() {
			let new_weight = tree.weight_of(node) * factor;
			out.tree.weights.push((node, new_weight));
		}

		out
	}
}
