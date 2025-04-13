use anyhow::Result;
use pyo3::prelude::*;
use pyo3::{conversion::FromPyObject, exceptions::PyTypeError};

use crate::{py_bail, state::PyState};

pub struct PyPrior {
	/// INVARIANT: the type has a `probability` method
	inner: PyObject,
}

impl PyPrior {
	pub fn probability(&self, py: Python, state: &PyState) -> Result<f64> {
		let args = (state.clone(),);
		let out = self
			.inner
			.bind(py)
			.call_method1("probability", args)?
			.extract::<f64>()?;
		Ok(out)
	}
}

impl<'py> FromPyObject<'py> for PyPrior {
	fn extract_bound(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
		if !obj.getattr("probability")?.is_callable() {
			py_bail!(
				PyTypeError,
				"Prior objects must have a `probability` method, \
				which takes `State` and returns a real number",
			);
		}

		Ok(Self {
			inner: obj.clone().unbind(),
		})
	}
}
