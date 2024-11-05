use std::collections::HashMap;

use crate::{parameter::Parameter, state::State};

pub enum Status {
	Accept,
	Reject,
	// TODO: decide if it should be log or not
	Hastings(f64),
}

pub struct Proposal {
	status: Status,
	/// A hash map of parameters updated by the operator.
	params: HashMap<String, Parameter>,
	// Might require additional optimisations, so it's separate
	// tree: Option<Tree>,
}

pub trait Operator {
	// TODO: rng
	fn propose(state: &State) -> Proposal;
}
