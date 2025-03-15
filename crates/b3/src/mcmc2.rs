use anyhow::Result;
use pyo3::prelude::*;
use rand::Rng as _;

use crate::{
	operator::{Proposal, PyOperator, WeightedScheduler},
	prior::PyPrior,
	state::PyState,
};

#[pyfunction]
pub fn run(
	length: usize,
	state: PyState,
	priors: Vec<PyPrior>,
	operators: Vec<PyOperator>,
) -> Result<()> {
	// We acquire the lock for the full duration of the program so that we
	// don't spend time locking and unlocking
	Python::with_gil(|py| -> Result<()> {
		let mut scheduler = WeightedScheduler::new(py, operators)?;

		for _ in 0..length {
			step(py, &state, &priors, &mut scheduler)?;
			// TODO: logging
		}
		Ok(())
	})
}

fn step(
	py: Python,
	state: &PyState,
	priors: &[PyPrior],
	scheduler: &mut WeightedScheduler,
) -> Result<()> {
	// TODO: select operator
	let operator =
		scheduler.select_operator(&mut state.inner().rng.inner());

	let hastings = match operator.propose(py, state)? {
		Proposal::Accept() => {
			// TODO: propose

			accept(state)?;
			return Ok(());
		}
		Proposal::Reject() => {
			return Ok(());
		}
		Proposal::Hastings(ratio) => ratio,
	};
	// TODO: apply proposal

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
