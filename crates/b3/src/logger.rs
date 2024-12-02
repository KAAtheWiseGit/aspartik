use anyhow::Result;
use serde_json::{json, to_string, to_value, Value as Json};

use std::{cell::RefCell, fs::File, io::Write, path::Path};

use crate::state::StateRef;
use base::seq::Character;

pub struct Logger {
	log_every: usize,
	file: RefCell<Option<File>>,

	distributions: Vec<String>,
	parameters: Vec<String>,
}

impl Logger {
	pub fn new(
		log_every: usize,
		file: Option<&Path>,
		distributions: Vec<String>,
		parameters: Vec<String>,
	) -> Self {
		let file = file.map(|path| File::create(path).unwrap()).into();

		Logger {
			log_every,
			file,
			distributions,
			parameters,
		}
	}

	pub(crate) fn log(&self, state: StateRef, index: usize) -> Result<()> {
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

		let out = to_string(&json![{
			"parameters": parameters,
			"distributions": "TODO",
		}])? + "\n";

		self.file.borrow_mut().as_mut().map_or_else(
			|| {
				println!("{}", out);
			},
			|file| {
				file.write_all(out.as_bytes()).unwrap();
			},
		);

		Ok(())
	}
}
