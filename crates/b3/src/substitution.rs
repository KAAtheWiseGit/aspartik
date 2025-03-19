use anyhow::Result;
use pyo3::prelude::*;
use pyo3::{conversion::FromPyObject, exceptions::PyTypeError};

pub struct PySubstitution {
	inner: PyObject,
}

impl<'py> FromPyObject<'py> for PySubstitution {
	fn extract_bound(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
		if !obj.getattr("update")?.is_callable() {
			return Err(PyTypeError::new_err("Substitution model objects must have an `update` method, which takes a list of edges and returns a substitution matrix"));
		}

		Ok(Self {
			inner: obj.clone().unbind(),
		})
	}
}

impl PySubstitution {
	pub fn update(&self, py: Python, edges: Vec<usize>) -> Result<()> {
		let args = (edges,).into_pyobject(py)?;
		let _matrix = self.inner.call_method1(py, "update", args)?;
		// let matrix = matrix.extract::<?>(py)?;

		Ok(())
	}
}
