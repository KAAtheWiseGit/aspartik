use base::substitution::{self, Substitution};

use crate::state::StateRef;

pub trait Model {
	type Substitution;

	fn get_matrix(&self, state: &StateRef);
}

pub enum DnaModel {
	JukesCantor,
	K80,
	F81,
	Hky,
	Gtr,
}

impl Model for DnaModel {
	type Substitution = Substitution<4>;

	fn get_matrix(&self, state: &StateRef) {
		todo!()
	}
}
