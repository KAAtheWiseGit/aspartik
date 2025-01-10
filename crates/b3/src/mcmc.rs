use rand::Rng;

use crate::{
	likelihood::{Likelihood, Row},
	log, make_ref,
	model::Model,
	operator::Proposal,
	operator::{scheduler::WeightedScheduler, Status},
	probability::Probability,
	state::StateRef,
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

pub fn run<const N: usize>(
	config: Config,
	state: &mut State<N>,
	prior: Box<dyn Probability>,
	scheduler: &mut WeightedScheduler,
	mut likelihoods: Vec<DynLikelihood<N>>,
	mut transitions: Transitions<N>,
	mut model: DynModel<N>,
) {
	let mut file = std::fs::File::create("start.trees").unwrap();

	// TODO: burnin
	for i in 0..(config.burnin + config.length) {
		let operator = scheduler.get_operator(&mut state.rng);

		// TODO: mutable state ref
		use rand::SeedableRng;
		use rand_pcg::Pcg64;
		let proposal = operator
			.propose(state.as_ref(), &mut Pcg64::seed_from_u64(4));

		let hastings = match proposal.status {
			Status::Accept => {
				propose(
					state,
					proposal,
					&mut likelihoods,
					&mut transitions,
					&mut model,
				);

				state.accept();
				transitions.accept();

				for likelihood in &mut likelihoods {
					likelihood.accept();
				}
				continue;
			}
			Status::Reject => {
				continue;
			}
			Status::Hastings(ratio) => ratio,
		};

		propose(
			state,
			proposal,
			&mut likelihoods,
			&mut transitions,
			&mut model,
		);

		let new_likelihood = likelihood(&likelihoods)
			+ prior.probability(state.as_ref());

		let ratio = new_likelihood - state.likelihood + hastings;

		if ratio > state.rng.random::<f64>().ln() {
			state.likelihood = new_likelihood;
			state.accept();
			transitions.accept();

			for likelihood in &mut likelihoods {
				likelihood.accept();
			}
		} else {
			state.reject();
			transitions.reject();

			for likelihood in &mut likelihoods {
				likelihood.reject();
			}
		}

		log::write(state.as_ref(), i).unwrap();

		if i % config.save_state_every == 0 && i > config.burnin {
			use std::io::Write;
			file.write_fmt(format_args!(
				"{}",
				state.as_ref().get_tree().serialize()
			))
			.unwrap();
		}
	}
}

fn propose<const N: usize>(
	state: &mut State<N>,
	mut proposal: Proposal,
	likelihoods: &mut [DynLikelihood<N>],
	transitions: &mut Transitions<N>,
	model: &mut DynModel<N>,
) {
	state.proposal_params = std::mem::take(&mut proposal.params);

	state.tree.propose(proposal);

	// Update the substitution matrix
	let substitution = model.get_matrix(make_ref!(state));
	// If the matrix has changed, `full` is true
	let full = transitions.update(substitution, make_ref!(state));

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

fn likelihood<const N: usize>(likelihoods: &[DynLikelihood<N>]) -> f64 {
	likelihoods
		.iter()
		.map(|likelihood| likelihood.likelihood())
		.sum()
}
