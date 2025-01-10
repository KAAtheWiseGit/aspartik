use anyhow::Result;
use serde_json::{json, to_string, to_value, Value as Json};

use std::{collections::HashMap, fs::File, io::Write, path::Path, sync::Mutex};

use crate::State;

static STATE: Mutex<Option<LogState>> = Mutex::new(None);

macro_rules! mut_state {
	() => {
		STATE.lock().unwrap().as_mut().unwrap()
	};
}

struct LogState {
	loggers: Vec<Logger>,

	probabilities: HashMap<String, f64>,
}

pub fn init(loggers: Vec<Logger>) {
	let mut r = STATE.lock().unwrap();

	*r = Some(LogState {
		loggers,
		probabilities: HashMap::new(),
	});
}

pub fn write(state: &State, index: usize) -> Result<()> {
	for logger in &mut mut_state!().loggers {
		logger.log(state, index)?;
	}

	Ok(())
}

pub fn log_distribution(name: &str, value: f64) {
	mut_state!().probabilities.insert(name.to_owned(), value);
}

pub struct Logger {
	log_every: usize,
	dst: Box<dyn Write + Sync + Send>,

	probabilities: Vec<String>,
	parameters: Vec<String>,
}

impl Logger {
	pub fn new(
		log_every: usize,
		file: Option<&Path>,
		probabilities: Vec<String>,
		parameters: Vec<String>,
	) -> Self {
		let dst = if let Some(path) = file {
			Box::new(File::create(path).unwrap())
				as Box<dyn Write + Sync + Send>
		} else {
			Box::new(std::io::stdout())
		};

		Logger {
			log_every,
			dst,
			probabilities,
			parameters,
		}
	}

	fn log(&mut self, state: &State, index: usize) -> Result<()> {
		if index % self.log_every != 0 {
			return Ok(());
		}

		let parameters = self
			.parameters
			.iter()
			.map(|parameter| -> Result<Json> {
				let param = state.get_parameter(parameter)?;

				Ok(to_value(param)?)
			})
			.collect::<Result<Vec<Json>>>()?;

		let mut distributions = HashMap::new();
		for distribution in &self.probabilities {
			distributions.insert(
				distribution.to_owned(),
				mut_state!().probabilities[distribution],
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
