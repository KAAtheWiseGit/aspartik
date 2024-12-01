use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256StarStar;

use crate::{
	operator::{scheduler::WeightedScheduler, Status},
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
	scheduler: &mut WeightedScheduler,
) {
	let mut rng = Xoshiro256StarStar::seed_from_u64(4);
	let mut old_likelihood = f64::NEG_INFINITY;

	let mut file = std::fs::File::create("start.trees").unwrap();

	// TODO: burnin
	for i in 0..(config.burnin + config.length) {
		let operator = scheduler.get_operator(&mut rng);
		let proposal = operator.propose(state, &mut rng);

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

		let likelihood = state.likelihood() + prior.probability(state);

		let ratio = likelihood - old_likelihood + hastings;

		if ratio > rng.gen::<f64>().ln() {
			old_likelihood = likelihood;
			state.accept();
		} else {
			state.reject();
		}

		if i % config.state == 0 && i > config.burnin {}

		if i % config.trees == 0 && i > config.burnin {
			let root = state.get_tree().root();
			println!("root: {}", state.get_tree().weight_of(root));
			println!("likelihood: {old_likelihood}");

			use std::io::Write;
			file.write_fmt(format_args!(
				"{}",
				state.get_tree().serialize()
			))
			.unwrap();
		}
	}
}
