#![allow(clippy::new_ret_no_self)]

use anyhow::Result;
use serde_json::{json, to_string, to_value, Value as Json};

use std::{collections::HashMap, fs::File, io::Write, path::Path, sync::Mutex};

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
pub struct StateLogger {
	every: usize,
	file: File,
}

impl StateLogger {
	/// Serializes the state to `file` every `every`-th step.
	pub fn new<P>(file: P, every: usize) -> Result<Box<dyn Logger>>
	where
		P: AsRef<Path>,
	{
		let file = File::create(file.as_ref())?;
		Ok(Box::new(TreeLogger { every, file }))
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
pub struct TreeLogger {
	every: usize,
	file: File,
}

impl TreeLogger {
	/// `file` is the file path to which trees will be written `every`
	/// steps.
	pub fn new<P>(file: P, every: usize) -> Result<Box<dyn Logger>>
	where
		P: AsRef<Path>,
	{
		let file = File::create(file.as_ref())?;
		Ok(Box::new(TreeLogger { every, file }))
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

pub struct JsonLogger {
	log_every: usize,
	dst: Box<dyn Write + Sync + Send>,

	probabilities: Vec<String>,
	parameters: Vec<String>,
}

impl JsonLogger {
	pub fn new(
		log_every: usize,
		file: Option<&Path>,
		probabilities: Vec<String>,
		parameters: Vec<String>,
	) -> Box<dyn Logger> {
		let dst = if let Some(path) = file {
			Box::new(File::create(path).unwrap())
				as Box<dyn Write + Sync + Send>
		} else {
			Box::new(std::io::stdout())
		};

		Box::new(JsonLogger {
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
