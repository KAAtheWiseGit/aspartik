use anyhow::{anyhow, Result};
use rand::SeedableRng;
use rand_pcg::Pcg64;
use serde_json::{json, Value as Json};

use std::collections::HashMap;

use crate::{
	likelihood::{Likelihood, Row},
	model::Model,
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
	pub(crate) params: HashMap<String, Parameter>,
	/// Proposal parameters
	pub(crate) proposal_params: HashMap<String, Parameter>,
	/// The phylogenetic tree, which also contains the genetic data.
	pub(crate) tree: Tree,

	pub(crate) model: DynModel<N>,
	pub(crate) transitions: Transitions<N>,

	/// Current likelihood, for caching purposes.
	pub(crate) likelihood: f64,

	pub(crate) rng: Pcg64,
}

/// A workaround because the `as_ref` method requires a full `state` borrow,
/// blocking partial mutable borrows.
#[macro_export]
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
	pub fn new(tree: Tree, model: DynModel<N>) -> Self {
		// let num_edges = (&sites[0].len() - 1) * 2;
		let num_edges = 0;

		Self {
			params: HashMap::new(),
			proposal_params: HashMap::new(),
			tree,
			model,
			transitions: Transitions::new(num_edges),
			likelihood: f64::NEG_INFINITY,
			rng: Pcg64::seed_from_u64(4),
		}
	}

	pub fn as_ref(&self) -> StateRef {
		StateRef {
			params: &self.params,
			proposal_params: &self.proposal_params,
			tree: &self.tree,
		}
	}

	pub fn as_mut(&mut self) -> StateRef {
		todo!()
	}

	/// Accept the current proposal
	pub(crate) fn accept(&mut self) {
		for (name, param) in std::mem::take(&mut self.proposal_params) {
			self.params.insert(name, param);
		}

		self.tree.accept();
	}

	pub(crate) fn reject(&mut self) {
		self.proposal_params.clear();

		self.tree.reject();
	}

	#[allow(unused)]
	pub(crate) fn serialize(&self) -> Json {
		json!({
			"tree": self.tree.serialize(),
			"parameters": self.params,
			"rng": self.rng,
		})
	}
}

#[derive(Clone, Copy)]
pub struct StateRef<'a> {
	pub params: &'a HashMap<String, Parameter>,
	pub proposal_params: &'a HashMap<String, Parameter>,
	pub tree: &'a Tree,
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
