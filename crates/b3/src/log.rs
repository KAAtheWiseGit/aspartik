use anyhow::Result;
use pyo3::prelude::*;
use pyo3::types::PyTuple;

use std::{fs::File, io::Write, path::PathBuf};

use crate::state::PyState;

/// The trait defining the logging behaviour.
///
/// The loggers registered with `b3` are interfaced with using this trait.
pub trait Logger: Send {
	/// Calls the logger at step `index`, passing the current state.
	///
	/// This function is called every step, excluding burnin (TODO: index?).
	/// Thus, the logger itself is responsible for implementing sampling and
	/// flushing the output.
	fn log(&mut self, state: PyState, index: usize) -> Result<()>;
}

pub struct PyLogger {
	inner: PyObject,
}

impl Logger for PyLogger {
	fn log(&mut self, state: PyState, index: usize) -> Result<()> {
		Python::with_gil(|py| -> Result<()> {
			let args = (state.clone(), index).into_pyobject(py)?;
			let args = PyTuple::new(py, args)?;
			self.inner.bind(py).call_method1("log", args)?;

			Ok(())
		})?;
		Ok(())
	}
}

/// Serializes the simulation state to allow pausing inference.
#[pyclass]
pub struct StateLogger {
	every: usize,
	#[expect(unused)]
	file: File,
}

#[pymethods]
impl StateLogger {
	/// Serializes the state to `file` every `every`-th step.
	#[new]
	pub fn new(file: PathBuf, every: usize) -> Result<Self> {
		let file = File::create(file)?;
		Ok(StateLogger { every, file })
	}
}

impl Logger for StateLogger {
	fn log(&mut self, _state: PyState, index: usize) -> Result<()> {
		if index % self.every != 0 {
			return Ok(());
		}

		todo!()
	}
}

/// Records the trees in Newick format, delimited by newlines.
// XXX: tree selection
#[pyclass]
pub struct TreeLogger {
	every: usize,
	file: File,
}

#[pymethods]
impl TreeLogger {
	/// `file` is the file path to which trees will be written `every`
	/// steps.
	#[new]
	pub fn new(file: PathBuf, every: usize) -> Result<Self> {
		let file = File::create(&file)?;
		Ok(TreeLogger { every, file })
	}
}

impl Logger for TreeLogger {
	fn log(&mut self, state: PyState, index: usize) -> Result<()> {
		if index % self.every != 0 {
			return Ok(());
		}

		let tree = state.inner().tree.inner().to_newick();
		self.file.write_all(tree.as_bytes())?;
		self.file.write_all(b"\n")?;
		self.file.flush()?;

		Ok(())
	}
}

pub fn submodule(py: Python<'_>) -> PyResult<Bound<'_, PyModule>> {
	let m = PyModule::new(py, "log")?;

	m.add_class::<StateLogger>()?;
	m.add_class::<TreeLogger>()?;

	Ok(m)
}
