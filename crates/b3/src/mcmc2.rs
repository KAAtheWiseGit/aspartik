use anyhow::Result;
use pyo3::prelude::*;
use rand::Rng as _;

use crate::{prior::PyPrior, state::PyState};

#[pyfunction]
pub fn run(length: usize, state: PyState, priors: Vec<PyPrior>) -> Result<()> {
	for i in 0..length {
		// XXX: should we just acquire the lock for the entire duration
		// of `run`?
		Python::with_gil(|py| -> Result<()> {
			step(py, &state, &priors)
		})?;
		// TODO: logging
	}

	Ok(())
}

fn step(py: Python, state: &PyState, priors: &[PyPrior]) -> Result<()> {
	// TODO: select operator

	// TODO: propose
	let hastings = 0.0;

	// TODO: get tree likelihood
	let likelihood = 0.0;

	let mut prior: f64 = 0.0;
	for py_prior in priors {
		prior += py_prior.probability(py, state)?;

		// short-circuit on a rejection by any prior
		if prior.is_infinite() {
			break;
		}
	}

	let posterior = likelihood + prior;

	let ratio = posterior - state.inner().likelihood + hastings;

	let random_0_1 = state.inner().rng.inner().random::<f64>().ln();
	if ratio > random_0_1 {
		state.inner().likelihood = posterior;

		accept(state)?;
	} else {
		reject(state)?;
	}

	Ok(())
}

fn accept(state: &PyState) -> Result<()> {
	state.inner().accept()?;

	Ok(())
}

fn reject(state: &PyState) -> Result<()> {
	state.inner().reject()?;

	Ok(())
}
