use anyhow::Result;
use pyo3::exceptions::PyKeyError;
use pyo3::prelude::*;

use std::{
	collections::HashMap,
	sync::{Arc, Mutex, MutexGuard},
};

use crate::{rng::PyRng, tree::PyTree, PyParameter};

#[derive(Debug)]
pub struct State {
	/// TODO: parameter serialization
	backup_params: HashMap<String, PyParameter>,
	/// Current set of parameters by name.
	params: HashMap<String, PyParameter>,

	pub(crate) tree: PyTree,

	/// Current likelihood, for caching purposes.
	pub(crate) likelihood: f64,

	pub(crate) rng: PyRng,
}

impl State {
	pub fn new(tree: PyTree, params: HashMap<String, PyParameter>) -> Self {
		Self {
			backup_params: HashMap::new(),
			params,
			tree,
			likelihood: f64::NEG_INFINITY,
			rng: PyRng::new(4),
		}
	}

	/// Accept the current proposal
	pub fn accept(&mut self) {
		self.backup_params = self.params.clone();

		self.tree.inner().accept();
	}

	pub fn reject(&mut self) {
		// deep copy
		self.params.clear();
		for (key, value) in &self.backup_params {
			self.params.insert(key.to_owned(), value.deep_copy());
		}

		self.tree.inner().reject();
	}
}

#[derive(Debug, Clone)]
#[pyclass(name = "State")]
pub struct PyState {
	inner: Arc<Mutex<State>>,
}

impl PyState {
	pub fn inner(&self) -> MutexGuard<State> {
		self.inner.lock().unwrap()
	}
}

#[pymethods]
impl PyState {
	#[new]
	fn new(tree: PyTree, params: HashMap<String, PyParameter>) -> Self {
		let state = State::new(tree, params);

		Self {
			inner: Arc::new(Mutex::new(state)),
		}
	}

	fn __getitem__(&self, name: &str) -> Result<PyParameter> {
		let inner = self.inner();
		let param = inner
			.params
			.get(name)
			.ok_or_else(|| {
				let msg = format!(
					"No parameter with the name {name}"
				);
				PyKeyError::new_err(msg)
			})?
			.clone();
		Ok(param)
	}

	#[getter]
	fn tree(&self) -> PyTree {
		self.inner().tree.clone()
	}

	#[getter]
	fn rng(&self) -> PyRng {
		self.inner().rng.clone()
	}
}
