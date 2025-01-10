use anyhow::{anyhow, Result};
use serde_json::{json, Value as Json};

use std::collections::HashMap;

use crate::{
	likelihood::{CpuLikelihood, Likelihood, Row},
	model::Model,
	operator::Proposal,
	parameter::Parameter,
	tree::Tree,
	Transitions,
};
use core::substitution::Substitution;

type DynLikelihood<const N: usize> =
	Box<dyn Likelihood<Row = Row<N>, Substitution = Substitution<N>>>;
type DynModel<const N: usize> = Box<dyn Model<Substitution = Substitution<N>>>;

pub struct State<const N: usize> {
	/// Map of parameters by name.
	params: HashMap<String, Parameter>,
	/// Proposal parameters
	proposal_params: HashMap<String, Parameter>,
	/// The phylogenetic tree, which also contains the genetic data.
	tree: Tree,

	model: DynModel<N>,
	transitions: Transitions<N>,

	likelihoods: Vec<DynLikelihood<N>>,

	/// Current likelihood, for caching purposes.
	pub(crate) likelihood: f64,
}

/// A workaround because the `as_ref` method requires a full `state` borrow,
/// blocking partial mutable borrows.
macro_rules! make_ref {
	($state:ident) => {
		&StateRef {
			params: &$state.params,
			proposal_params: &$state.proposal_params,
			tree: &$state.tree,
		}
	};
}

impl<const N: usize> State<N> {
	pub fn new(
		tree: Tree,
		sites: Vec<Vec<Row<N>>>,
		model: DynModel<N>,
	) -> Self {
		let num_edges = (&sites[0].len() - 1) * 2;

		let likelihood = Box::new(CpuLikelihood::new(sites));
		let likelihoods: Vec<DynLikelihood<N>> = vec![likelihood];

		Self {
			params: HashMap::new(),
			proposal_params: HashMap::new(),
			tree,
			model,
			transitions: Transitions::new(num_edges),
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

		self.tree.propose(proposal);

		// Update the substitution matrix
		let substitution = self.model.get_matrix(make_ref!(self));
		// If the matrix has changed, `full` is true
		let full =
			self.transitions.update(substitution, make_ref!(self));

		let nodes = if full {
			// Full update, as matrices impact likelihoods
			self.tree.full_update()
		} else {
			self.tree.nodes_to_update()
		};

		let (nodes, edges, children) = self.tree.to_lists(&nodes);

		let transitions = self.transitions.matrices(&edges);

		for likelihood in &mut self.likelihoods {
			likelihood.propose(&nodes, &transitions, &children);
		}
	}

	/// Accept the current proposal
	pub(crate) fn accept(&mut self) {
		for (name, param) in std::mem::take(&mut self.proposal_params) {
			self.params.insert(name, param);
		}

		self.tree.accept();
		self.transitions.accept();

		for likelihood in &mut self.likelihoods {
			likelihood.accept();
		}
	}

	pub(crate) fn reject(&mut self) {
		self.proposal_params.clear();

		self.tree.reject();
		self.transitions.reject();

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

	pub fn has_parameter<S: AsRef<str>>(&self, name: S) -> bool {
		// Proposal can't have parameters not already in state.
		self.params.contains_key(name.as_ref())
	}

	pub fn get_tree(&self) -> &Tree {
		self.tree
	}
}
