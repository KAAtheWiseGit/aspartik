use anyhow::Result;
use pyo3::prelude::*;
use rand::Rng as _;

use crate::{
	operator::{Proposal, PyOperator, WeightedScheduler},
	prior::PyPrior,
	state::PyState,
	substitution::PySubstitution,
	transitions::Transitions,
};

#[pyfunction]
pub fn run(
	length: usize,
	state: PyState,
	priors: Vec<PyPrior>,
	operators: Vec<PyOperator>,
	substitution: PySubstitution<4>,
) -> Result<()> {
	let num_edges = state.inner().tree.inner().num_internals() * 2;
	let mut transitions = Transitions::<4>::new(num_edges);

	// We acquire the lock for the full duration of the program so that we
	// don't spend time locking and unlocking
	Python::with_gil(|py| -> Result<()> {
		let mut scheduler = WeightedScheduler::new(py, operators)?;

		for _ in 0..length {
			step(
				py,
				&state,
				&priors,
				&substitution,
				&mut transitions,
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
	substitution: &PySubstitution<4>,
	transitions: &mut Transitions<4>,
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

	// update the transitions
	let substitution_matrix = substitution.get_matrix(py)?;
	let inner_state = state.inner();
	let tree = &*inner_state.tree.inner();
	let full_update = transitions.update(substitution_matrix, tree);
	let nodes = if full_update {
		tree.full_update()
	} else {
		tree.nodes_to_update()
	};

	// calculate tree likelihood
	let likelihood = 0.0;

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
