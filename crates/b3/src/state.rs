use anyhow::{anyhow, Result};
use serde_json::{json, Value as Json};

use std::collections::HashMap;

use crate::{
	likelihood::{CpuLikelihood, Likelihood},
	operator::Proposal,
	parameter::{BooleanParam, IntegerParam, Parameter, RealParam},
	substitution::Substitution,
	tree::Tree,
};
use base::{
	seq::{Character, Seq},
	substitution::Model,
};
use linalg::{RowMatrix, Vector};

type DynLikelihood<const N: usize> = Box<
	dyn Likelihood<
		Row = Vector<f64, N>,
		Substitution = RowMatrix<f64, N, N>,
	>,
>;

pub struct State<C: Character, const N: usize> {
	/// Map of parameters by name.
	params: HashMap<String, Parameter>,
	/// Proposal parameters
	proposal_params: HashMap<String, Parameter>,
	/// The phylogenetic tree, which also contains the genetic data.
	tree: Tree,

	model: Substitution<C, N>,

	likelihoods: Vec<DynLikelihood<N>>,

	/// Current likelihood, for caching purposes.
	pub(crate) likelihood: f64,
}

impl<C: Character, const N: usize> State<C, N> {
	pub fn new(
		tree: Tree,
		sites: Vec<Vec<Vector<f64, N>>>,
		mut model: Substitution<C, N>,
	) -> Self {
		let likelihood = Box::new(CpuLikelihood::new(sites));
		let mut likelihoods: Vec<DynLikelihood<N>> = vec![likelihood];

		let update = tree.update_all_likelihoods();

		model.propose(&update.edges, &update.lengths);
		model.accept();
		let substitutions = model.substitutions();

		for likelihood in &mut likelihoods {
			likelihood.propose(
				&substitutions,
				&update.nodes,
				&update.children,
			);
			likelihood.accept();
		}

		Self {
			params: HashMap::new(),
			proposal_params: HashMap::new(),
			tree,
			model,
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

	pub fn as_ref(&self) -> StateRef {
		StateRef {
			params: &self.params,
			proposal_params: &self.proposal_params,
			tree: &self.tree,
		}
	}

	pub(crate) fn propose(&mut self, mut proposal: Proposal) {
		self.proposal_params = std::mem::take(&mut proposal.params);

		let update = self.tree.propose(proposal);

		self.model.propose(&update.edges, &update.lengths);
		let substitutions = self.model.substitutions();

		for likelihood in &mut self.likelihoods {
			likelihood.propose(
				&substitutions,
				&update.nodes,
				&update.children,
			);
		}
	}

	/// Accept the current proposal
	pub(crate) fn accept(&mut self) {
		for (name, param) in std::mem::take(&mut self.proposal_params) {
			self.params.insert(name, param);
		}

		self.tree.accept();
		self.model.accept();

		for likelihood in &mut self.likelihoods {
			likelihood.accept();
		}
	}

	pub(crate) fn reject(&mut self) {
		self.proposal_params.clear();

		self.tree.reject();
		self.model.reject();

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

#[derive(Clone, Copy)]
pub struct StateRef<'a> {
	params: &'a HashMap<String, Parameter>,
	proposal_params: &'a HashMap<String, Parameter>,
	tree: &'a Tree,
}

impl StateRef<'_> {
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
		self.tree
	}
}
