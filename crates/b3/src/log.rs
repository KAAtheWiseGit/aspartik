#![allow(clippy::new_ret_no_self)]

use anyhow::Result;
use serde_json::{json, to_string, to_value, Value as Json};

use std::{collections::HashMap, fs::File, io::Write, path::Path, sync::Mutex};

use crate::State;

pub trait Logger: Send {
	fn log(&mut self, state: &State, index: usize) -> Result<()>;
}

static STATE: Mutex<Option<LogState>> = Mutex::new(None);

macro_rules! mut_state {
	() => {
		STATE.lock().unwrap().as_mut().unwrap()
	};
}

struct LogState {
	loggers: Vec<Box<dyn Logger>>,

	priors: HashMap<String, f64>,
}

pub fn init(loggers: Vec<Box<dyn Logger>>) {
	let mut state = STATE.lock().unwrap();

	*state = Some(LogState {
		loggers,
		priors: HashMap::new(),
	});
}

pub fn write(state: &State, index: usize) -> Result<()> {
	for logger in &mut mut_state!().loggers {
		logger.log(state, index)?;
	}

	Ok(())
}

pub fn record_prior(name: &str, value: f64) {
	mut_state!().priors.insert(name.to_owned(), value);
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
