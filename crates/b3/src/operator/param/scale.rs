use anyhow::{anyhow, Result};
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
	fn propose(&self, state: &mut State) -> Result<Proposal> {
		let scale = self.distribution.random_range(
			self.factor,
			1.0 / self.factor,
			state,
		)?;

		// TODO: ugliness
		let len = state.param(&self.parameter)?.len();
		let index = Uniform::new(0, len)?.sample(&mut state.rng);

		let param = state
			.mut_param(&self.parameter)?
			.as_mut_real()
			.ok_or_else(|| {
				anyhow!("ParamScale can't edit a non-real parameter '{}'", &self.parameter)
			})?;
		param[index] *= scale;

		Ok(Proposal::Reject)
	}
}
