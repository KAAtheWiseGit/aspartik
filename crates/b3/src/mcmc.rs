use anyhow::Result;
use rand::Rng;

use crate::{
	likelihood::{Likelihood, Row},
	log,
	model::Model,
	operator::{scheduler::WeightedScheduler, Proposal},
	probability::Probability,
	State, Transitions,
};
use core::substitution::Substitution;

pub struct Config {
	pub burnin: usize,
	pub length: usize,

	pub save_state_every: usize,
}

pub type DynLikelihood<const N: usize> =
	Box<dyn Likelihood<Row = Row<N>, Substitution = Substitution<N>>>;
pub type DynModel<const N: usize> =
	Box<dyn Model<Substitution = Substitution<N>>>;

// TODO: interrupt handler, saving the state
pub fn run<const N: usize>(
	config: Config,
	state: &mut State,
	prior: Box<dyn Probability>,
	scheduler: &mut WeightedScheduler,
	mut likelihoods: Vec<DynLikelihood<N>>,
	mut transitions: Transitions<N>,
	mut model: DynModel<N>,
) -> Result<()> {
	// TODO: output configuration (maybe another logger type)
	let mut file = std::fs::File::create("start.trees")?;

	// TODO: burnin
	for i in 0..(config.burnin + config.length) {
		let operator = scheduler.get_operator(&mut state.rng);

		let hastings = match operator.propose(state)? {
			Proposal::Accept => {
				propose(
					state,
					&mut likelihoods,
					&mut transitions,
					&mut model,
				);

				accept(
					state,
					&mut likelihoods,
					&mut transitions,
				);

				continue;
			}
			Proposal::Reject => {
				continue;
			}
			Proposal::Hastings(ratio) => ratio,
		};

		propose(state, &mut likelihoods, &mut transitions, &mut model);

		let new_likelihood =
			likelihood(&likelihoods) + prior.probability(state);

		let ratio = new_likelihood - state.likelihood + hastings;

		if ratio > state.rng.random::<f64>().ln() {
			state.likelihood = new_likelihood;

			accept(state, &mut likelihoods, &mut transitions);
		} else {
			reject(state, &mut likelihoods, &mut transitions);
		}

		log::write(state, i)?;

		if i % config.save_state_every == 0 && i > config.burnin {
			use std::io::Write;
			file.write_fmt(format_args!(
				"{}",
				state.get_tree().into_newick()
			))?;
		}
	}

	Ok(())
}

fn propose<const N: usize>(
	state: &mut State,
	likelihoods: &mut [DynLikelihood<N>],
	transitions: &mut Transitions<N>,
	model: &mut DynModel<N>,
) {
	// Update the substitution matrix
	let substitution = model.get_matrix(state);
	// If the matrix has changed, `full` is true
	let full = transitions.update(substitution, state);

	let nodes = if full {
		// Full update, as matrices impact likelihoods
		state.tree.full_update()
	} else {
		state.tree.nodes_to_update()
	};

	let (nodes, edges, children) = state.tree.to_lists(&nodes);

	let transitions = transitions.matrices(&edges);

	for likelihood in likelihoods {
		likelihood.propose(&nodes, &transitions, &children);
	}
}

fn accept<const N: usize>(
	state: &mut State,
	likelihoods: &mut [DynLikelihood<N>],
	transitions: &mut Transitions<N>,
) {
	state.tree.verify();
	state.accept();
	transitions.accept();

	for likelihood in likelihoods {
		likelihood.accept();
	}
}

fn reject<const N: usize>(
	state: &mut State,
	likelihoods: &mut [DynLikelihood<N>],
	transitions: &mut Transitions<N>,
) {
	state.reject();
	transitions.reject();

	for likelihood in likelihoods {
		likelihood.reject();
	}
}

fn likelihood<const N: usize>(likelihoods: &[DynLikelihood<N>]) -> f64 {
	likelihoods
		.iter()
		.map(|likelihood| likelihood.likelihood())
		.sum()
}
