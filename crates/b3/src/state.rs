use std::collections::HashMap;

use crate::{operator::Proposal, parameter::Parameter};

pub struct State {
	params: HashMap<String, Parameter>,
	proposal: Option<Proposal>,
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

	pub fn propose(&mut self, proposal: Proposal) {
		self.proposal = Some(proposal);
	}

	/// Accept the current proposal
	pub fn accept(&mut self) {
		let Some(mut proposal) = self.proposal.take() else {
			return;
		};

		for (name, param) in proposal.take_params() {
			self.params.insert(name, param);
		}

		// TODO: tree
	}
}
