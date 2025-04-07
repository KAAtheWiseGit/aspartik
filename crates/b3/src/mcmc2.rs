use anyhow::Result;
use pyo3::prelude::*;
use rand::Rng as _;

use crate::{
	likelihood::{Likelihood, PyLikelihood},
	operator::{Proposal, PyOperator, WeightedScheduler},
	state::PyState,
	transitions::Transitions,
	PyPrior,
};

#[pyfunction]
pub fn run(
	length: usize,
	state: PyState,
	priors: Vec<PyPrior>,
	operators: Vec<PyOperator>,
	likelihood: PyLikelihood,
) -> Result<()> {
	let num_edges = state.inner().tree.inner().num_internals() * 2;
	let mut transitions = Transitions::<4>::new(num_edges);

	let likelihood = &mut *likelihood.inner();

	// We acquire the lock for the full duration of the program so that we
	// don't spend time locking and unlocking
	Python::with_gil(|py| -> Result<()> {
		let mut scheduler = WeightedScheduler::new(py, operators)?;

		for _ in 0..length {
			step(
				py,
				&state,
				&priors,
				&mut transitions,
				likelihood,
				&mut scheduler,
			)?;
			// TODO: logging
		}
		Ok(())
	})
}

fn step(
	py: Python,
	state: &PyState,
	priors: &[PyPrior],
	transitions: &mut Transitions<4>,
	likelihood: &mut Likelihood,
	scheduler: &mut WeightedScheduler,
) -> Result<()> {
	let operator =
		scheduler.select_operator(&mut state.inner().rng.inner());

	let hastings = match operator.propose(py, state)? {
		Proposal::Accept() => {
			accept(state)?;
			return Ok(());
		}
		Proposal::Reject() => {
			return Ok(());
		}
		Proposal::Hastings(ratio) => ratio,
	};

	let mut prior: f64 = 0.0;
	for py_prior in priors {
		prior += py_prior.probability(py, state)?;

		// short-circuit on a rejection by any prior
		if prior == f64::NEG_INFINITY {
			reject(state)?;
			return Ok(());
		}
	}

	// calculate tree likelihood
	let likelihood = likelihood.propose(py, state, transitions)?;

	let posterior = likelihood + prior;

	let ratio = posterior - state.inner().likelihood + hastings;

	let random_0_1 = state.inner().rng.inner().random::<f64>();
	if ratio > random_0_1.ln() {
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
