use anyhow::Result;

use crate::{log::log_distribution, State};

mod distribution;

pub use distribution::DistributionPrior;

pub type LogProb = f64;

pub trait Probability {
	fn probability(&self, state: &State) -> Result<LogProb>;
}

pub struct Prior {
	name: String,
	prior: Box<dyn Probability>,
}

impl Prior {
	pub fn new<S, P>(name: S, prior: P) -> Self
	where
		S: AsRef<str>,
		P: Probability + 'static,
	{
		Prior {
			name: name.as_ref().to_owned(),
			prior: Box::new(prior),
		}
	}
}

impl Probability for Prior {
	fn probability(&self, state: &State) -> Result<LogProb> {
		let probability = self.prior.probability(state)?;
		log_distribution(&self.name, probability);
		Ok(probability)
	}
}
