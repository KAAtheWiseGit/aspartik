use anyhow::{anyhow, Result};
use rand::distr::{Distribution as _, Uniform};

use crate::operator::{Operator, Proposal};
use crate::{distribution::Distribution, State};

/// Scales a single dimension of a parameter.
pub struct ScaleOne {
	parameter: String,
	factor: f64,
	distribution: Distribution,
}

impl ScaleOne {
	/// `factor` defines how large the proposals will be.  The parameter
	/// values will be scaled between `factor` and `1 / factor`.
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

impl Operator for ScaleOne {
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

/// An operator which scales all dimensions of a parameter.
pub struct ScaleAll {
	parameter: String,
	factor: f64,
	distribution: Distribution,
	independent: bool,
}

impl ScaleAll {
	/// Set `independent` to true to scale all of the parameter dimensions
	/// independently.
	pub fn new(
		parameter: String,
		factor: f64,
		distribution: Distribution,
		independent: bool,
	) -> Box<dyn Operator> {
		assert!(0.0 < factor && factor < 1.0);

		Box::new(Self {
			parameter,
			factor,
			distribution,
			independent,
		})
	}
}

impl Operator for ScaleAll {
	fn propose(&self, state: &mut State) -> Result<Proposal> {
		let len = state.param(&self.parameter)?.len();

		let scales = if self.independent {
			vec![
				self.distribution.random_range(
					self.factor,
					1.0 / self.factor,
					state,
				)?;
				len
			]
		} else {
			(0..len).map(|_| {
				self.distribution.random_range(
					self.factor,
					1.0 / self.factor,
					state,
				)
			})
			.collect::<Result<Vec<f64>>>()?
		};

		let param = state
			.mut_param(&self.parameter)?
			.as_mut_real()
			.ok_or_else(|| {
				anyhow!("ParamScale can't edit a non-real parameter '{}'", &self.parameter)
			})?;
		for i in 0..len {
			param[i] *= scales[i];
		}

		Ok(Proposal::Reject)
	}
}
