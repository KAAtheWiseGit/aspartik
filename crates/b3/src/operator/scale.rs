use rand::Rng;

use super::{Operator, Proposal, Rng as RngT};
use crate::state::State;

pub struct Scale();

impl Operator for Scale {
	fn propose(&self, state: &State, rng: &mut RngT) -> Proposal {
		let tree = state.get_tree();
		let factor = rng.gen_range(0.8..1.25);

		let mut out = Proposal::accept();

		for node in tree.nodes() {
			let new_weight = tree.weight_of(node) * factor;
			out.tree.weights.push((node, new_weight));
		}

		out
	}
}
