use serde_json::{json, Value as Json};

use std::collections::HashMap;

use crate::{
	operator::Proposal,
	parameter::{BooleanParam, IntegerParam, Parameter, RealParam},
	tree::Tree,
};

pub struct State {
	/// Map of parameters by name.
	params: HashMap<String, Parameter>,
	/// Proposal parameters
	proposal_params: HashMap<String, Parameter>,
	/// The phylogenetic tree, which also contains the genetic data.
	tree: Tree,
}

impl State {
	pub fn new(tree: Tree) -> State {
		State {
			params: HashMap::new(),
			proposal_params: HashMap::new(),
			tree,
		}
	}

	pub fn likelihood(&self) -> f64 {
		self.tree.likelihood()
	}

	/// # Panics
	///
	/// Panics if `name` is not a valid parameter name.
	pub fn get_parameter<S: AsRef<str>>(&self, name: S) -> &Parameter {
		let name = name.as_ref();

		if let Some(param) = self.proposal_params.get(name) {
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
		self.proposal_params = proposal.params;

		self.tree.update_with(proposal.tree);
	}

	/// Accept the current proposal
	pub fn accept(&mut self) {
		for (name, param) in std::mem::take(&mut self.proposal_params) {
			self.params.insert(name, param);
		}
		self.tree.accept();
	}

	pub fn reject(&mut self) {
		self.proposal_params.clear();
		self.tree.reject();
	}

	pub fn serialize(&self) -> Json {
		json!({
			"tree": self.tree.serialize(),
			"parameters": self.params,
		})
	}
}
