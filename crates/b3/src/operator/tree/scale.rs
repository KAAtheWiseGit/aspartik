use anyhow::Result;

use crate::operator::{Operator, Proposal};
use crate::{distribution::Distribution, State};

pub struct Scale {
	factor: f64,
	distribution: Distribution,
}

impl Scale {
	pub fn new(
		factor: f64,
		distribution: Distribution,
	) -> Box<dyn Operator> {
		assert!(0.0 < factor && factor < 1.0);

		Box::new(Self {
			factor,
			distribution,
		})
	}
}

impl Operator for Scale {
	fn propose(&self, state: &mut State) -> Result<Proposal> {
		let rng = &mut state.rng;
		let tree = &mut state.tree;

		let scale = self.distribution.gen_range(
			self.factor,
			1.0 / self.factor,
			rng,
		);

		for node in tree.nodes() {
			let new_weight = tree.weight_of(node) * scale;
			tree.update_weight(node, new_weight);
		}

		// TODO: ratio
		Ok(Proposal::Hastings(0.0))
	}
}
