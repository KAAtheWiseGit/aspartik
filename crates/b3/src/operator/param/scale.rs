use rand::distr::{Distribution as _, Uniform};

use crate::operator::{Operator, Proposal};
use crate::{distribution::Distribution, State};

// TODO: scaling all dimensions
pub struct Scale {
	parameter: String,
	factor: f64,
	distribution: Distribution,
}

impl Scale {
	pub fn new(
		parameter: String,
		factor: f64,
		distribution: Distribution,
	) -> Box<dyn Operator> {
		assert!(0.0 < factor && factor < 1.0);

		Box::new(Self {
			parameter,
			factor,
			distribution,
		})
	}
}

impl Operator for Scale {
	fn propose(&self, state: &mut State) -> Proposal {
		let scale = self.distribution.gen_range(
			self.factor,
			1.0 / self.factor,
			&mut state.rng,
		);

		// TODO: ugliness
		let len = state.param(&self.parameter).unwrap().len();
		let index =
			Uniform::new(0, len).unwrap().sample(&mut state.rng);

		let param = state
			.mut_param(&self.parameter)
			.unwrap()
			.as_mut_real()
			.unwrap();
		param[index] *= scale;

		Proposal::Reject
	}
}
