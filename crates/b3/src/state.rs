use std::collections::HashMap;

use crate::{operator::Proposal, parameter::Parameter};

pub struct State {
	params: HashMap<String, Parameter>,
}

impl State {
	/// # Panics
	///
	/// Panics if `name` is not a valid parameter name.
	pub fn get_parameter<S: AsRef<str>>(&self, name: S) -> &Parameter {
		&self.params[name.as_ref()]
	}

	pub fn has_parameter<S: AsRef<str>>(&self, name: S) -> bool {
		self.params.contains_key(name.as_ref())
	}

	// XXX: Distinct?
	// pub fn get_tree() -> Tree

	/// Updates the state by accepting a proposal.
	pub fn update(&mut self, mut proposal: Proposal) {
		for (name, param) in proposal.take_params() {
			self.params.insert(name, param);
		}

		// TODO: tree
	}
}
