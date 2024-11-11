use std::collections::HashMap;

use crate::{
	operator::Proposal,
	parameter::{BooleanParam, IntegerParam, Parameter, RealParam},
	tree::Tree,
};

pub struct State {
	params: HashMap<String, Parameter>,
	tree: Tree,
	proposal: Option<Proposal>,
}

impl State {
	/// # Panics
	///
	/// Panics if `name` is not a valid parameter name.
	pub fn get_parameter<S: AsRef<str>>(&self, name: S) -> &Parameter {
		let name = name.as_ref();

		if let Some(proposal) = &self.proposal {
			if let Some(param) = proposal.params.get(name) {
				return param;
			}
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
		self.proposal = Some(proposal);
	}

	/// Accept the current proposal
	pub fn accept(&mut self) {
		let Some(mut proposal) = self.proposal.take() else {
			return;
		};

		for (name, param) in std::mem::take(&mut proposal.params) {
			self.params.insert(name, param);
		}

		// TODO: tree
	}
}
