#![allow(clippy::new_ret_no_self)]

use anyhow::Result;
use pyo3::prelude::*;
use serde_json::{json, to_string, to_value, Value as Json};

use std::{
	collections::HashMap, fs::File, io::Write, path::PathBuf, sync::Mutex,
};

use crate::State;

/// The trait defining the logging behaviour.
///
/// The loggers registered with `b3` are interfaced with using this trait.
pub trait Logger: Send {
	/// Calls the logger at step `index`, passing the current state.
	///
	/// This function is called every step, excluding burnin (TODO: index?).
	/// Thus, the logger itself is responsible for implementing sampling and
	/// flushing the output.
	fn log(&mut self, state: &State, index: usize) -> Result<()>;
}

static STATE: Mutex<Option<GlobalLoggerStore>> = Mutex::new(None);

/// Unwraps the Mutex holding `LogState`.  Must be called once per the scope of
/// the returned value, otherwise the function will deadlock.
macro_rules! mut_state {
	() => {
		STATE.lock().unwrap().as_mut().unwrap()
	};
}

/// The internal logging object, which holds the cache and all of the registered
/// loggers.
struct GlobalLoggerStore {
	/// The loggers to be executed.
	loggers: Vec<Box<dyn Logger>>,

	/// Cached prior probability values.  They must be set by priors using
	/// `record_prior`.
	priors: HashMap<String, f64>,
}

/// Initialize the given loggers.
///
/// They will be called by the main MCMC loop.
pub fn init(loggers: Vec<Box<dyn Logger>>) {
	let mut state = STATE.lock().unwrap();

	*state = Some(GlobalLoggerStore {
		loggers,
		priors: HashMap::new(),
	});
}

/// Execute the loggers.
pub(crate) fn write(state: &State, index: usize) -> Result<()> {
	for logger in &mut mut_state!().loggers {
		logger.log(state, index)?;
	}

	Ok(())
}

/// Cache a value of a prior for the current iteration.
pub(crate) fn record_prior(name: &str, value: f64) {
	mut_state!().priors.insert(name.to_owned(), value);
}

/// Serializes the simulation state to allow pausing inference.
#[pyclass]
pub struct StateLogger {
	every: usize,
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
	fn log(&mut self, state: &State, index: usize) -> Result<()> {
		if index % self.every != 0 {
			return Ok(());
		}

		// Truncate.  Only fails if the file is not writable, which is
		// not the case here.
		self.file.set_len(0)?;
		serde_json::to_writer(&self.file, state)?;

		Ok(())
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
	fn log(&mut self, state: &State, index: usize) -> Result<()> {
		if index % self.every != 0 {
			return Ok(());
		}

		use std::io::Write;
		let tree = state.tree.into_newick();
		self.file.write_all(tree.as_bytes())?;
		self.file.write_all(b"\n")?;
		self.file.flush()?;

		Ok(())
	}
}

#[pyclass]
pub struct JsonLogger {
	log_every: usize,
	dst: File,

	probabilities: Vec<String>,
	parameters: Vec<String>,
}

#[pymethods]
impl JsonLogger {
	#[new]
	pub fn new(
		log_every: usize,
		file: PathBuf,
		probabilities: Vec<String>,
		parameters: Vec<String>,
	) -> Result<Self> {
		let dst = File::create(file)?;

		Ok(JsonLogger {
			log_every,
			dst,
			probabilities,
			parameters,
		})
	}
}

impl Logger for JsonLogger {
	fn log(&mut self, state: &State, index: usize) -> Result<()> {
		if index % self.log_every != 0 {
			return Ok(());
		}

		let parameters = self
			.parameters
			.iter()
			.map(|parameter| -> Result<Json> {
				let param = state.param(parameter)?;

				Ok(to_value(param)?)
			})
			.collect::<Result<Vec<Json>>>()?;

		let mut distributions = HashMap::new();
		for distribution in &self.probabilities {
			distributions.insert(
				distribution.to_owned(),
				mut_state!().priors[distribution],
			);
		}

		let out = to_string(&json![{
			"parameters": parameters,
			"distributions": distributions,
		}])? + "\n";

		self.dst.write_all(out.as_bytes())?;

		Ok(())
	}
}

pub fn submodule(py: Python<'_>) -> PyResult<Bound<'_, PyModule>> {
	let m = PyModule::new(py, "log")?;

	m.add_class::<StateLogger>()?;
	m.add_class::<TreeLogger>()?;
	m.add_class::<JsonLogger>()?;

	Ok(m)
}
