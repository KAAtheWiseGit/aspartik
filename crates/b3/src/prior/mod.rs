use anyhow::Result;

use crate::State;

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
mod distribution;

pub use distribution::DistributionPrior;

pub type LogProb = f64;

pub trait Probability {
	fn probability(&self, state: &State) -> Result<LogProb>;
}
