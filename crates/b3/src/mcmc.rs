use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256StarStar;

use crate::{
	operator::{scheduler::TurnScheduler, Status},
	probability::Probability,
	state::State,
};

pub struct Config {
	pub burnin: usize,
	pub length: usize,

	pub state: usize,
	pub trees: usize,
	// TODO: logger
}

pub fn run(
	config: Config,
	state: &mut State,
	prior: Box<dyn Probability>,
	scheduler: &mut TurnScheduler,
) {
	let mut rng = Xoshiro256StarStar::seed_from_u64(4);
	let mut old_likelihood = f64::NEG_INFINITY;

	// TODO: burnin
	for i in 0..(config.burnin + config.length) {
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

		if i % config.state == 0 && i > config.burnin {
			// TODO: save the state.
			// XXX: perhaps put the logger here, too.  One
			// conditional away from the tight loop.
		}
	}
}
