use std::collections::HashMap;

use crate::{
	operator::Proposal,
	parameter::{BooleanParam, IntegerParam, Parameter, RealParam},
	tree::Tree,
};

pub struct State {
	params: HashMap<String, Parameter>,
	tree: Tree,
	proposal: Proposal,
}

impl State {
	/// # Panics
	///
	/// Panics if `name` is not a valid parameter name.
	pub fn get_parameter<S: AsRef<str>>(&self, name: S) -> &Parameter {
		let name = name.as_ref();

		if let Some(param) = self.proposal.params.get(name) {
			return param;
		}

		&self.params[name]
	}

	pub fn get_real_parameter<S: AsRef<str>>(
		&self,
		name: S,
	) -> Option<&RealParam> {
		match self.get_parameter(name) {
			Parameter::Real(p) => Some(p),
			_ => None,
		}
	}

	pub fn get_integer_parameter<S: AsRef<str>>(
		&self,
		name: S,
	) -> Option<&IntegerParam> {
		match self.get_parameter(name) {
			Parameter::Integer(p) => Some(p),
			_ => None,
		}
	}

	pub fn get_boolean_parameter<S: AsRef<str>>(
		&self,
		name: S,
	) -> Option<&BooleanParam> {
		match self.get_parameter(name) {
			Parameter::Boolean(p) => Some(p),
			_ => None,
		}
	}

	pub fn has_parameter<S: AsRef<str>>(&self, name: S) -> bool {
		// Proposal can't have parameters not already in state.
		self.params.contains_key(name.as_ref())
	}

	pub fn get_tree(&self) -> &Tree {
		&self.tree
	}

	pub fn propose(&mut self, proposal: Proposal) {
		self.proposal = proposal;

		let reverse = self
			.tree
			.update_with(self.proposal.tree.take().unwrap());
		self.proposal.tree = Some(reverse);
	}

	/// Accept the current proposal
	pub fn accept(&mut self) {
		for (name, param) in std::mem::take(&mut self.proposal.params) {
			self.params.insert(name, param);
		}
	}

	pub fn reject(&mut self) {
		self.proposal.params.clear();

		// Roll the tree back
		self.tree.update_with(self.proposal.tree.take().unwrap());
	}
}
