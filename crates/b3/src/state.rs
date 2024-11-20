use serde_json::{json, Value as Json};

use std::collections::HashMap;

use crate::{
	likelihood::Likelihood,
	operator::Proposal,
	parameter::{BooleanParam, IntegerParam, Parameter, RealParam},
	tree::Tree,
};
use base::{seq::DnaSeq, substitution::dna::Dna4Substitution};

pub struct State {
	/// Map of parameters by name.
	params: HashMap<String, Parameter>,
	/// Proposal parameters
	proposal_params: HashMap<String, Parameter>,
	/// The phylogenetic tree, which also contains the genetic data.
	tree: Tree,

	likelihoods: Vec<Likelihood<Dna4Substitution>>,
}

impl State {
	pub fn new(tree: Tree, sequences: &[DnaSeq]) -> State {
		let mut c1 = vec![];
		let mut c2 = vec![];
		let mut c3 = vec![];
		let mut c4 = vec![];

		for i in 0..sequences[0].len() {
			let mut column = Vec::new();
			for sequence in sequences {
				column.push(sequence[i]);
			}

			if i % 4 == 0 {
				c1.push(column);
			} else if i % 4 == 1 {
				c2.push(column);
			} else if i % 4 == 2 {
				c3.push(column);
			} else {
				c4.push(column);
			}
		}

		let mut likelihoods = vec![
			Likelihood::new(c1, Dna4Substitution::jukes_cantor()),
			Likelihood::new(c2, Dna4Substitution::jukes_cantor()),
			Likelihood::new(c3, Dna4Substitution::jukes_cantor()),
			Likelihood::new(c4, Dna4Substitution::jukes_cantor()),
		];

		let update = tree.update_all_likelihoods();
		for likelihood in &mut likelihoods {
			likelihood.update(&update);
		}

		State {
			params: HashMap::new(),
			proposal_params: HashMap::new(),
			tree,
			likelihoods,
		}
	}

	pub fn likelihood(&self) -> f64 {
		self.likelihoods.iter().map(|l| l.likelihood()).sum()
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

	pub fn propose(&mut self, mut proposal: Proposal) {
		self.proposal_params = std::mem::take(&mut proposal.params);

		let update = self.tree.propose(proposal);

		use rayon::prelude::*;
		self.likelihoods.par_iter_mut().for_each(|likelihood| {
			likelihood.update(&update);
		});
	}

	/// Accept the current proposal
	pub fn accept(&mut self) {
		for (name, param) in std::mem::take(&mut self.proposal_params) {
			self.params.insert(name, param);
		}

		self.tree.accept();

		for likelihood in &mut self.likelihoods {
			likelihood.accept();
		}
	}

	pub fn reject(&mut self) {
		self.proposal_params.clear();
		self.tree.reject();

		for likelihood in &mut self.likelihoods {
			likelihood.reject();
		}
	}

	pub fn serialize(&self) -> Json {
		json!({
			"tree": self.tree.serialize(),
			"parameters": self.params,
		})
	}
}
