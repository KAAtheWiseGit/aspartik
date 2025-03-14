use anyhow::Result;
use pyo3::prelude::*;
use pyo3::types::PyTuple;

use crate::state::PyState;

// TODO: name
pub struct PyPrior {
	inner: PyObject,
}

impl PyPrior {
	pub fn new(obj: PyObject) -> Self {
		Self { inner: obj }
	}
}

impl PyPrior {
	pub fn probability(&self, py: Python, state: &PyState) -> Result<f64> {
		// TODO: name logging
		let args = PyTuple::new(py, [state.clone()])?;
		let out = self
			.inner
			.bind(py)
			.call_method1("probability", args)?
			.extract::<f64>()?;
		let out: f64 = Ok::<f64, PyErr>(out)?;

		Ok(out)
	}
}
