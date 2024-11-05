use std::collections::HashMap;

use crate::parameter::Parameter;

pub struct State {
	params: HashMap<String, Parameter>,
}

impl State {
	pub fn get_parameter(name: String) -> Parameter {
		todo!()
	}

	// XXX: Distinct?
	// pub fn get_tree() -> Tree
}
