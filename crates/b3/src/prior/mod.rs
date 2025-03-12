use anyhow::Result;
use pyo3::prelude::*;

use crate::{log::record_prior, State};

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
		record_prior(&self.name, probability);
		Ok(probability)
	}
}

pub struct PyPrior {
	inner: PyObject,
}

impl PyPrior {
	pub fn new(obj: PyObject) -> Self {
		Self { inner: obj }
	}
}

impl Probability for PyPrior {
	fn probability(&self, _state: &State) -> Result<LogProb> {
		let out: f64 = Python::with_gil(|py| {
			let out = self
				.inner
				.bind(py)
				.call_method1(
					"probability",
					(/* TODO: state */),
				)?
				.extract::<f64>()?;
			Ok::<f64, PyErr>(out)
		})?;

		Ok(out)
	}
}
