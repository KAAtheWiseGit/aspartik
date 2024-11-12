use crate::state::State;

// modules:
//
// - [x] compound: combine several distributions
//
// - priors:
//
//   - [ ] Beta
//   - [ ] Chi^2
//   - [ ] Dirichlet
//   - [ ] Exponential
//   - [ ] Gamma
//   - [ ] Inverse Gamma
//   - [ ] Laplace
//   - [ ] Log normal
//   - [ ] Normal
//   - [ ] One on X
//   - [x] Poisson
//   - [x] Uniform
//
// - Tree taxa groupings, which check the tree and return -inf if the conditions
//   aren't met.
mod compound;
mod poisson;
mod uniform;

pub use compound::Compound;
pub use poisson::Poisson;
pub use uniform::Uniform;

pub type LogProb = f64;

pub trait Probability {
	// Because we pass `State` here, this can be implemented for both
	// parameter priors and for the tree likelihood.
	fn probability(&self, state: &State) -> LogProb;
}
