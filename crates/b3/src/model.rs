use base::substitution::{self, Substitution};

use crate::state::StateRef;

pub trait Model {
	type Substitution;

	fn get_matrix(&self, state: &StateRef) -> Self::Substitution;
}

pub enum DnaModel {
	JukesCantor,
	K80 {
		kappa: String,
	},
	F81 {
		probabilities: String,
	},
	Hky {
		kappa: String,
		probabilities: String,
	},
	Gtr {},
}

impl Model for DnaModel {
	type Substitution = Substitution<4>;

	fn get_matrix(&self, state: &StateRef) -> Substitution<4> {
		match self {
			DnaModel::JukesCantor => substitution::jukes_cantor(),
			DnaModel::K80 { kappa } => {
				// TODO: a more ergonomic parameter API
				let kappa = state
					.get_real_parameter(kappa)
					.unwrap()
					.unwrap()
					.values[0];

				substitution::k80(kappa)
			}
			_ => todo!(),
		}
	}
}
