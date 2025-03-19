use anyhow::Result;
use linalg::RowMatrix;
use pyo3::prelude::*;
use pyo3::{conversion::FromPyObject, exceptions::PyTypeError};

use crate::state::PyState;

pub struct PyClock {
	inner: PyObject,
}

pub type Substitution = RowMatrix<f64, 4, 4>;

impl<'py> FromPyObject<'py> for PyClock {
	fn extract_bound(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
		if !obj.getattr("update")?.is_callable() {
			return Err(PyTypeError::new_err("Substitution model objects must have an `update` method, which takes state, a list of edges and returns clock rates on these edges"));
		}

		Ok(Self {
			inner: obj.clone().unbind(),
		})
	}
}

impl PyClock {
	pub fn update(
		&self,
		py: Python,
		state: PyState,
		edges: Vec<usize>,
	) -> Result<Vec<f64>> {
		let args = (state, edges).into_pyobject(py)?;
		let rates = self.inner.call_method1(py, "update", args)?;
		let rates = rates.extract::<Vec<f64>>(py)?;

		Ok(rates)
	}
}
