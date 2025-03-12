#![allow(dead_code)]

use anyhow::{bail, Result};
use pyo3::prelude::*;
use pyo3::{
	conversion::FromPyObjectBound, exceptions::PyIndexError, types::PyTuple,
};

#[derive(Debug)]
enum Param {
	Real(Vec<f64>),
	Integer(Vec<i64>),
	Boolean(Vec<bool>),
}

impl Param {
	fn len(&self) -> usize {
		match self {
			Param::Real(p) => p.len(),
			Param::Integer(p) => p.len(),
			Param::Boolean(p) => p.len(),
		}
	}
}

impl ToString for Param {
	fn to_string(&self) -> String {
		let mut out = String::new();

		macro_rules! seq {
			($p: expr) => {
				for (i, value) in $p.iter().enumerate() {
					out += &value.to_string();
					if i < $p.len() - 1 {
						out += ", ";
					}
				}
			};
		}
		match self {
			Param::Real(p) => seq!(p),
			Param::Integer(p) => seq!(p),
			Param::Boolean(p) => seq!(p),
		}

		out
	}
}

#[derive(Debug)]
#[pyclass(sequence)]
pub struct PyParameter {
	inner: Param,
}

impl PyParameter {
	fn check_index(&self, i: usize) -> PyResult<()> {
		if i >= self.inner.len() {
			Err(PyIndexError::new_err(
				format!("Parameter has {} dimensions, index {i} is out of bounds", self.inner.len()))
			)
		} else {
			Ok(())
		}
	}
}

#[pymethods]
#[allow(non_snake_case)]
impl PyParameter {
	#[staticmethod]
	#[pyo3(signature = (*values))]
	pub fn Real(values: &Bound<PyTuple>) -> Result<Self> {
		if values.is_empty() {
			bail!("A parameter must have at least one value")
		}

		let values: Vec<f64> = extract(values)?;
		Ok(Self {
			inner: Param::Real(values),
		})
	}

	#[staticmethod]
	#[pyo3(signature = (*values))]
	pub fn Integer(values: &Bound<PyTuple>) -> Result<Self> {
		if values.is_empty() {
			bail!("A parameter must have at least one value")
		}

		let values: Vec<i64> = extract(values)?;
		Ok(Self {
			inner: Param::Integer(values),
		})
	}

	#[staticmethod]
	#[pyo3(signature = (*values))]
	pub fn Boolean(values: &Bound<PyTuple>) -> Result<Self> {
		if values.is_empty() {
			bail!("A parameter must have at least one value")
		}

		let values: Vec<bool> = extract(values)?;
		Ok(Self {
			inner: Param::Boolean(values),
		})
	}

	pub fn __len__(&self) -> usize {
		self.inner.len()
	}

	pub fn __getitem__(&self, py: Python, i: usize) -> PyResult<PyObject> {
		self.check_index(i)?;

		Ok(match &self.inner {
			Param::Real(p) => p[i].into_pyobject(py)?.into(),
			Param::Integer(p) => p[i].into_pyobject(py)?.into(),
			Param::Boolean(p) => {
				p[i].into_pyobject(py)?.to_owned().into()
			}
		})
	}

	pub fn __setitem__(
		&mut self,
		i: usize,
		value: Bound<PyAny>,
	) -> PyResult<()> {
		self.check_index(i)?;

		match &mut self.inner {
			Param::Real(p) => {
				let value = value.extract::<f64>()?;
				p[i] = value;
			}
			Param::Integer(p) => {
				let value = value.extract::<i64>()?;
				p[i] = value;
			}
			Param::Boolean(p) => {
				let value = value.extract::<bool>()?;
				p[i] = value;
			}
		}

		Ok(())
	}

	pub fn __repr__(&self) -> String {
		let mut out = String::from("PyParameter.");
		match &self.inner {
			Param::Real(..) => out += "Real(",
			Param::Integer(..) => out += "Integer(",
			Param::Boolean(..) => out += "Boolean(",
		}
		out += &self.inner.to_string();
		out += ")";
		out
	}

	pub fn __str__(&self) -> String {
		format!("[{}]", self.inner.to_string())
	}
}

fn extract<T: for<'a> FromPyObjectBound<'a, 'a>>(
	tuble: &Bound<PyTuple>,
) -> Result<Vec<T>> {
	Ok(tuble.into_iter()
		.map(|v| v.extract::<T>())
		.collect::<PyResult<Vec<T>>>()?)
}
