use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256StarStar;

use crate::{
	operator::{scheduler::WeightedScheduler, Status},
	probability::Probability,
	Logger, State,
};
use base::seq::Character;

pub struct Config {
	pub burnin: usize,
	pub length: usize,

	pub save_state_every: usize,

	pub loggers: Vec<Logger>,
}

pub fn run<C: Character, const N: usize>(
	config: Config,
	state: &mut State<C, N>,
	prior: Box<dyn Probability>,
	scheduler: &mut WeightedScheduler,
) {
	let mut rng = Xoshiro256StarStar::seed_from_u64(4);

	let mut file = std::fs::File::create("start.trees").unwrap();

	// TODO: burnin
	for i in 0..(config.burnin + config.length) {
		let operator = scheduler.get_operator(&mut rng);
		let proposal = operator.propose(state.as_ref(), &mut rng);

		let hastings = match proposal.status {
			Status::Accept => {
				state.propose(proposal);
				state.accept();
				continue;
			}
			Status::Reject => {
				continue;
			}
			Status::Hastings(ratio) => ratio,
		};

		state.propose(proposal);

		let new_likelihood =
			state.likelihood() + prior.probability(state.as_ref());

		let ratio = new_likelihood - state.likelihood + hastings;

		if ratio > rng.gen::<f64>().ln() {
			state.likelihood = new_likelihood;
			state.accept();
		} else {
			state.reject();
		}

		for logger in &config.loggers {
			logger.log(state.as_ref(), i).unwrap();
		}

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
