#![allow(dead_code)]

use anyhow::Result;
use pyo3::prelude::*;
use pyo3::{
	conversion::FromPyObjectBound,
	exceptions::{PyIndexError, PyTypeError},
	types::PyTuple,
};

use std::fmt::{self, Display};

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

impl Display for Param {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Param::Real(p) => {
				for (i, value) in p.iter().enumerate() {
					value.fmt(f)?;
					if i < p.len() - 1 {
						f.write_str(", ")?;
					}
				}
			}
			Param::Integer(p) => {
				for (i, value) in p.iter().enumerate() {
					value.fmt(f)?;
					if i < p.len() - 1 {
						f.write_str(", ")?;
					}
				}
			}
			Param::Boolean(p) => {
				for (i, value) in p.iter().enumerate() {
					if *value {
						f.write_str("True")?;
					} else {
						f.write_str("False")?;
					}
					if i < p.len() - 1 {
						f.write_str(", ")?;
					}
				}
			}
		}

		Ok(())
	}
}

#[derive(Debug)]
#[pyclass(sequence)]
pub struct PyParameter {
	inner: Param,
}

impl PyParameter {
	fn check_index(&self, i: usize) -> Result<()> {
		if i >= self.inner.len() {
			let dimension = if self.inner.len() % 10 == 1 {
				"dimension"
			} else {
				"dimensions"
			};
			Err(PyIndexError::new_err(
				format!("Parameter has {} {}, index {} is out of bounds", self.inner.len(), dimension, i)
			).into())
		} else {
			Ok(())
		}
	}
}

fn check_empty(values: &Bound<PyTuple>) -> Result<()> {
	if values.is_empty() {
		Err(PyTypeError::new_err(
			"A parameter must have at least one value",
		)
		.into())
	} else {
		Ok(())
	}
}

#[pymethods]
#[allow(non_snake_case)]
impl PyParameter {
	#[staticmethod]
	#[pyo3(signature = (*values))]
	pub fn Real(values: &Bound<PyTuple>) -> Result<Self> {
		check_empty(values)?;

		let values: Vec<f64> = extract(values)?;
		Ok(Self {
			inner: Param::Real(values),
		})
	}

	#[staticmethod]
	#[pyo3(signature = (*values))]
	pub fn Integer(values: &Bound<PyTuple>) -> Result<Self> {
		check_empty(values)?;

		let values: Vec<i64> = extract(values)?;
		Ok(Self {
			inner: Param::Integer(values),
		})
	}

	#[staticmethod]
	#[pyo3(signature = (*values))]
	pub fn Boolean(values: &Bound<PyTuple>) -> Result<Self> {
		check_empty(values)?;

		let values: Vec<bool> = extract(values)?;
		Ok(Self {
			inner: Param::Boolean(values),
		})
	}

	pub fn __len__(&self) -> usize {
		self.inner.len()
	}

	pub fn __getitem__(&self, py: Python, i: usize) -> Result<PyObject> {
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
		let subtype = match &self.inner {
			Param::Real(..) => "Real",
			Param::Integer(..) => "Integer",
			Param::Boolean(..) => "Boolean",
		};

		format!("PyParameter.{}({})", subtype, self.inner)
	}

	pub fn __str__(&self) -> String {
		format!("[{}]", self.inner)
	}
}

fn extract<T: for<'a> FromPyObjectBound<'a, 'a>>(
	tuple: &Bound<PyTuple>,
) -> Result<Vec<T>> {
	Ok(tuple.into_iter()
		.map(|v| v.extract::<T>())
		.collect::<PyResult<Vec<T>>>()?)
}
