use anyhow::{anyhow, Result};
use rand::SeedableRng;
use rand_pcg::Pcg64;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use crate::{parameter::Parameter, tree::Tree};

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
	/// Proposal parameters
	#[serde(rename = "parameters")]
	old_params: HashMap<String, Parameter>,
	/// Current set of parameters by name.
	#[serde(skip)]
	params: HashMap<String, Parameter>,

	/// Name of each sequence
	names: Vec<String>,
	/// The phylogenetic tree, which also contains the genetic data.
	pub(crate) tree: Tree,

	/// Current likelihood, for caching purposes.
	pub(crate) likelihood: f64,

	pub(crate) rng: Pcg64,
}

impl State {
	pub fn new(names: Vec<String>, tree: Tree) -> Self {
		Self {
			old_params: HashMap::new(),
			params: HashMap::new(),
			names,
			tree,
			likelihood: f64::NEG_INFINITY,
			rng: Pcg64::seed_from_u64(4),
		}
	}

	/// Accept the current proposal
	pub(crate) fn accept(&mut self) {
		self.old_params = self.params.clone();

		self.tree.accept();
	}

	pub(crate) fn reject(&mut self) {
		self.params = self.old_params.clone();

		self.tree.reject();
	}

	pub fn param<S: AsRef<str>>(&self, name: S) -> Result<&Parameter> {
		let name = name.as_ref();

		self.params.get(name).ok_or_else(|| {
			anyhow!("Tried to get the parameter '{name}', which is not present in the state")
		})
	}

	pub fn mut_param<S: AsRef<str>>(
		&mut self,
		name: S,
	) -> Result<&mut Parameter> {
		let name = name.as_ref();

		self.params.get_mut(name).ok_or_else(|| {
			anyhow!("Tried to get the parameter '{name}', which is not present in the state")
		})
	}

	pub fn has_parameter<S: AsRef<str>>(&self, name: S) -> bool {
		self.params.contains_key(name.as_ref())
	}

	pub fn get_tree(&self) -> &Tree {
		&self.tree
	}
}
