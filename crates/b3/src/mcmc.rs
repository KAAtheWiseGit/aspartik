use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256StarStar;

use crate::{
	operator::{scheduler::TurnScheduler, Status},
	probability::Probability,
	state::State,
};

pub fn run(
	state: &mut State,
	prior: Box<dyn Probability>,
	scheduler: &mut TurnScheduler,
) {
	let mut rng = Xoshiro256StarStar::seed_from_u64(4);
	let mut old_likelihood = f64::NEG_INFINITY;

	// TODO: burnin
	loop {
		let operator = scheduler.get_operator();
		let proposal = operator.propose(state, &mut rng);
		state.propose(proposal);

		let hastings = match state.get_proposal_status() {
			Status::Accept => {
				state.accept();
				continue;
			}
			Status::Reject => {
				state.reject();
				continue;
			}
			Status::Hastings(ratio) => ratio,
		};

		let likelihood = state.likelihood() + prior.probability(state);

		let ratio = likelihood - old_likelihood + hastings;

		if ratio > rng.gen::<f64>().ln() {
			state.accept();
		} else {
			state.reject();
		}

		old_likelihood = likelihood;
	}
}
