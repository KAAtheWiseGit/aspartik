use anyhow::Result;

use std::{cell::RefCell, fs::File, io::Write, path::Path};

use crate::State;

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

	pub(crate) fn log(&self, state: &State, index: usize) -> Result<()> {
		if index % self.log_every != 0 {
			return Ok(());
		}

		let out = String::new();

		for parameter in &self.parameters {
			let _param = state.get_parameter(parameter)?;

			// TODO: parameter serialization
		}

		self.file.borrow_mut().as_mut().map_or_else(
			|| {
				println!("{out}");
			},
			|file| {
				file.write_all(out.as_bytes()).unwrap();
			},
		);

		Ok(())
	}
}
