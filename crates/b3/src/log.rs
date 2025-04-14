use anyhow::Result;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;

use crate::{py_bail, py_call_method, state::PyState};

pub struct PyLogger {
	inner: PyObject,
	every: Option<usize>,
}

impl<'py> FromPyObject<'py> for PyLogger {
	fn extract_bound(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
		if !obj.getattr("log")?.is_callable() {
			py_bail!(
				PyTypeError,
				"Loggers must have a callable `log` method"
			);
		}

		let every = obj.getattr("every")?.extract::<usize>().ok();

		Ok(PyLogger {
			inner: obj.clone().unbind(),
			every,
		})
	}
}

impl PyLogger {
	pub fn log(&mut self, state: PyState, index: usize) -> Result<()> {
		if self.every.is_some_and(|every| index % every != 0) {
			return Ok(());
		}

		Python::with_gil(|py| -> Result<()> {
			let args = (state.clone(), index);
			py_call_method!(py, self.inner, "log", args)?;

			Ok(())
		})?;

		Ok(())
	}
}
