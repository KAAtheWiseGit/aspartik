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
	Gtr {
		exchanges: String,
		probabilities: String,
	},
}

impl Model for DnaModel {
	type Substitution = Substitution<4>;

	fn get_matrix(&self, state: &StateRef) -> Substitution<4> {
		match self {
			DnaModel::JukesCantor => substitution::jukes_cantor(),
			DnaModel::K80 { kappa } => {
				let kappa = state
					.get_parameter(kappa)
					.unwrap()
					.as_real()
					.unwrap()
					.first();

				substitution::k80(kappa)
			}
			DnaModel::F81 { probabilities } => {
				let probabilities = state
					.get_parameter(probabilities)
					.unwrap()
					.as_real()
					.unwrap();

				substitution::f81(
					probabilities.values[0],
					probabilities.values[1],
					probabilities.values[2],
					probabilities.values[3],
				)
			}
			DnaModel::Hky {
				kappa,
				probabilities,
			} => {
				let kappa = state
					.get_parameter(kappa)
					.unwrap()
					.as_real()
					.unwrap()
					.first();

				let probabilities = state
					.get_parameter(probabilities)
					.unwrap()
					.as_real()
					.unwrap();

				substitution::hky(
					kappa,
					probabilities.values[0],
					probabilities.values[1],
					probabilities.values[2],
					probabilities.values[3],
				)
			}
			DnaModel::Gtr {
				exchanges,
				probabilities,
			} => {
				let exchanges = state
					.get_parameter(exchanges)
					.unwrap()
					.as_real()
					.unwrap();

				let probabilities = state
					.get_parameter(probabilities)
					.unwrap()
					.as_real()
					.unwrap();
				substitution::gtr(
					exchanges.values[0],
					exchanges.values[1],
					exchanges.values[2],
					exchanges.values[3],
					exchanges.values[4],
					exchanges.values[5],
					probabilities.values[0],
					probabilities.values[1],
					probabilities.values[2],
					probabilities.values[3],
				)
			}
		}
	}
}
