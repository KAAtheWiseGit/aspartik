use anyhow::{anyhow, Result};
use rand::SeedableRng;
use rand_pcg::Pcg64;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use crate::{parameter::Parameter, tree::Tree};

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
	/// Map of parameters by name.
	pub(crate) params: HashMap<String, Parameter>,
	/// Proposal parameters
	#[serde(skip)]
	pub(crate) proposal_params: HashMap<String, Parameter>,
	/// The phylogenetic tree, which also contains the genetic data.
	pub(crate) tree: Tree,

	/// Current likelihood, for caching purposes.
	pub(crate) likelihood: f64,

	pub(crate) rng: Pcg64,
}

impl State {
	pub fn new(tree: Tree) -> Self {
		Self {
			params: HashMap::new(),
			proposal_params: HashMap::new(),
			tree,
			likelihood: f64::NEG_INFINITY,
			rng: Pcg64::seed_from_u64(4),
		}
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
		&self.tree
	}
}
