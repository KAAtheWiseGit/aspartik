use anyhow::{anyhow, Result};
use serde_json::{json, Value as Json};

use std::collections::HashMap;

use crate::{
	likelihood::{Likelihood, ThreadedLikelihood},
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

	// TODO: generic
	likelihoods: Vec<ThreadedLikelihood<Dna4Substitution>>,

	/// Current likelihood, for caching purposes.
	pub(crate) likelihood: f64,
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
			ThreadedLikelihood::new(
				c1,
				Dna4Substitution::jukes_cantor(),
			),
			ThreadedLikelihood::new(
				c2,
				Dna4Substitution::jukes_cantor(),
			),
			ThreadedLikelihood::new(
				c3,
				Dna4Substitution::jukes_cantor(),
			),
			ThreadedLikelihood::new(
				c4,
				Dna4Substitution::jukes_cantor(),
			),
		];

		let update = tree.update_all_likelihoods();
		for likelihood in &mut likelihoods {
			likelihood.propose(update.clone());
		}

		State {
			params: HashMap::new(),
			proposal_params: HashMap::new(),
			tree,
			likelihoods,
			likelihood: f64::NEG_INFINITY,
		}
	}

	pub(crate) fn likelihood(&self) -> f64 {
		self.likelihoods
			.iter()
			.map(|likelihood| likelihood.likelihood())
			.sum()
	}

	pub fn get_parameter<S: AsRef<str>>(
		&self,
		name: S,
	) -> Result<&Parameter> {
		let name = name.as_ref();

		if let Some(param) = self.proposal_params.get(name) {
			return Ok(param);
		}

		self.params.get(name).ok_or_else(|| {
			anyhow!("Tried to get the parameter '{name}', which is not present in the state")
		})
	}

	pub fn get_real_parameter<S: AsRef<str>>(
		&self,
		name: S,
	) -> Result<Option<&RealParam>> {
		match self.get_parameter(name)? {
			Parameter::Real(p) => Ok(Some(p)),
			_ => Ok(None),
		}
	}

	pub fn get_integer_parameter<S: AsRef<str>>(
		&self,
		name: S,
	) -> Result<Option<&IntegerParam>> {
		match self.get_parameter(name)? {
			Parameter::Integer(p) => Ok(Some(p)),
			_ => Ok(None),
		}
	}

	pub fn get_boolean_parameter<S: AsRef<str>>(
		&self,
		name: S,
	) -> Result<Option<&BooleanParam>> {
		match self.get_parameter(name)? {
			Parameter::Boolean(p) => Ok(Some(p)),
			_ => Ok(None),
		}
	}

	pub fn has_parameter<S: AsRef<str>>(&self, name: S) -> bool {
		// Proposal can't have parameters not already in state.
		self.params.contains_key(name.as_ref())
	}

	pub fn get_tree(&self) -> &Tree {
		&self.tree
	}

	pub(crate) fn propose(&mut self, mut proposal: Proposal) {
		self.proposal_params = std::mem::take(&mut proposal.params);

		let update = self.tree.propose(proposal);

		for likelihood in &mut self.likelihoods {
			likelihood.propose(update.clone());
		}
	}

	/// Accept the current proposal
	pub(crate) fn accept(&mut self) {
		for (name, param) in std::mem::take(&mut self.proposal_params) {
			self.params.insert(name, param);
		}

		self.tree.accept();

		for likelihood in &mut self.likelihoods {
			likelihood.accept();
		}
	}

	pub(crate) fn reject(&mut self) {
		self.proposal_params.clear();

		self.tree.reject();

		for likelihood in &mut self.likelihoods {
			likelihood.reject();
		}
	}

	#[allow(unused)]
	pub(crate) fn serialize(&self) -> Json {
		json!({
			"tree": self.tree.serialize(),
			"parameters": self.params,
		})
	}
}
