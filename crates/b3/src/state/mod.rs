use crate::parameter::Parameter;

pub type Index = usize;

pub struct State {
	params: Vec<Parameter>,
}

impl State {
	// TODO: how do we index parameters?
	pub fn get_parameter(id: Index) -> Parameter {
		todo!()
	}

	// XXX: Distinct?
	// pub fn get_tree() -> Tree
}
