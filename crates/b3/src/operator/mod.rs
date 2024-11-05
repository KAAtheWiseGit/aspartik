use crate::state::State;

pub struct Proposal {
	status: Status,
	// TODO: return a set of updated parameters
}

pub enum Status {
	Accept,
	Reject,
	// TODO: decide if it should be log or not
	Hastings(f64),
}

pub trait Operator {
	// TODO: rng
	fn propose(state: &State) -> Proposal;
}
