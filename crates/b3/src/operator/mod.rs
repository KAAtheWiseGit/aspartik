use std::collections::HashMap;

use crate::{
	parameter::Parameter,
	state::{Index, State},
};

pub enum Status {
	Accept,
	Reject,
	// TODO: decide if it should be log or not
	Hastings(f64),
}

pub struct Proposal {
	status: Status,
	// XXX: index type
	/// A hash map of updated parameters with their indexes as keys.
	params: HashMap<Index, Parameter>,
	// Might require additional optimisations, so it's separate
	// tree: Option<Tree>,
}

pub trait Operator {
	// TODO: rng
	fn propose(state: &State) -> Proposal;
}
