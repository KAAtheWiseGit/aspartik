use anyhow::{anyhow, bail, Result};
use rand::SeedableRng;
use rand_pcg::Pcg64;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use crate::{
	parameter::{BooleanParam, IntegerParam, Parameter, RealParam},
	tree::Tree,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
	/// Proposal parameters
	#[serde(rename = "parameters")]
	old_params: HashMap<String, Parameter>,
	/// Current set of parameters by name.
	#[serde(skip)]
	params: HashMap<String, Parameter>,

	/// The phylogenetic tree, which also contains the genetic data.
	pub(crate) tree: Tree,

	/// Current likelihood, for caching purposes.
	pub(crate) likelihood: f64,

	pub(crate) rng: Pcg64,
}

macro_rules! cast {
	($value:expr, $func:ident, $name:expr, $typ:expr) => {{
		let param = $value;
		let t = param.type_name();
		param.$func().ok_or_else(|| {
			anyhow!("Expected parameter '{}' to be {}, got {} instead", $name, $typ, t)
		})
	}};
}

macro_rules! expect_one {
	($cast:expr, $name:expr) => {{
		let p = $cast;
		if p.len() == 1 {
			Ok(p[0])
		} else {
			bail!(
				"Expected parameter {} to have one dimension",
				$name
			)
		}
	}};
}

impl State {
	pub fn new(tree: Tree) -> Self {
		Self {
			old_params: HashMap::new(),
			params: HashMap::new(),
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

	// Parameter API

	/// Returns true if the parameter `name` is present in the state.
	pub fn has_parameter(&self, name: &str) -> bool {
		self.params.contains_key(name)
	}

	pub fn param(&self, name: &str) -> Result<&Parameter> {
		self.params.get(name).ok_or_else(|| {
			anyhow!("Tried to get the parameter '{name}', which is not present in the state")
		})
	}

	pub fn real_param(&self, name: &str) -> Result<&RealParam> {
		cast!(self.param(name)?, as_real, name, "real")
	}

	pub fn integer_param(&self, name: &str) -> Result<&IntegerParam> {
		cast!(self.param(name)?, as_integer, name, "integer")
	}

	pub fn boolean_param(&self, name: &str) -> Result<&BooleanParam> {
		cast!(self.param(name)?, as_boolean, name, "boolean")
	}

	pub fn mut_param(&mut self, name: &str) -> Result<&mut Parameter> {
		self.params.get_mut(name).ok_or_else(|| {
			anyhow!("Tried to get the parameter '{name}', which is not present in the state")
		})
	}

	pub fn mut_real_param(&mut self, name: &str) -> Result<&mut RealParam> {
		cast!(self.mut_param(name)?, as_mut_real, name, "real")
	}

	pub fn mut_integer_param(
		&mut self,
		name: &str,
	) -> Result<&mut IntegerParam> {
		cast!(self.mut_param(name)?, as_mut_integer, name, "integer")
	}

	pub fn mut_boolean_param(
		&mut self,
		name: &str,
	) -> Result<&mut BooleanParam> {
		cast!(self.mut_param(name)?, as_mut_boolean, name, "boolean")
	}

	pub fn one_real_param(&self, name: &str) -> Result<f64> {
		expect_one!(self.real_param(name)?, name)
	}

	pub fn one_integer_param(&self, name: &str) -> Result<i64> {
		expect_one!(self.integer_param(name)?, name)
	}

	pub fn one_boolean_param(&self, name: &str) -> Result<bool> {
		expect_one!(self.boolean_param(name)?, name)
	}
}
