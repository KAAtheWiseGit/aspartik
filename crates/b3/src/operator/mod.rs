use std::collections::HashMap;

use crate::{parameter::Parameter, state::State};

#[derive(Debug, Clone, Copy)]
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

impl Proposal {
	pub fn status(&self) -> Status {
		self.status
	}

	pub fn params(&self) -> &HashMap<String, Parameter> {
		&self.params
	}

	pub fn take_params(&mut self) -> HashMap<String, Parameter> {
		std::mem::take(&mut self.params)
	}
}

pub trait Operator {
	// TODO: rng
	fn propose(state: &State) -> Proposal;
}
