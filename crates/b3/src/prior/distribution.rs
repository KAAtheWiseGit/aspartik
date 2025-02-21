#![allow(unused)]

use anyhow::{bail, Result};

use super::Probability;
use crate::{parameter::Parameter, Distribution, State};

pub struct DistributionPrior {
	param: String,
	dist: Distribution,
}

impl DistributionPrior {
	pub fn new(param: String, dist: Distribution) -> Self {
		DistributionPrior { param, dist }
	}
}

impl Probability for DistributionPrior {
	fn probability(&self, state: &State) -> Result<f64> {
		match state.param(&self.param)? {
			Parameter::Real(p) => {
				if !self.dist.is_continious() {
					// TODO: distribution name
					bail!("The distribution is not continuous and doesn't support real parameters")
				}

				let mut out = 0.0;
				for i in 0..p.len() {
					let density =
						self.dist.pdf(p[i], state)?;
					out += density.ln();
				}

				Ok(out)
			}
			Parameter::Integer(p) => {
				if !self.dist.is_discrete() {
					// TODO: distribution name
					bail!("The distribution is not discrete and doesn't support integer parameters")
				}

				let mut out = 0.0;
				for i in 0..p.len() {
					let density =
						self.dist.pmf(p[i], state)?;
					out += density.ln();
				}

				Ok(out)
			}
			Parameter::Boolean(..) => {
				bail!("DistributionPrior does not support boolean paramters");
			}
		}
	}
}
