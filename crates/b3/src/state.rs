use std::collections::HashMap;

use crate::parameter::Parameter;

pub struct State {
	params: HashMap<String, Parameter>,
}

impl State {
	/// # Panics
	///
	/// Panics if `name` is not a valid parameter name.
	pub fn get_parameter<S: AsRef<str>>(&self, name: S) -> &Parameter {
		&self.params[name.as_ref()]
	}

	pub fn has_parameter<S: AsRef<str>>(&self, name: S) -> bool {
		self.params.contains_key(name.as_ref())
	}

	// XXX: Distinct?
	// pub fn get_tree() -> Tree
}
