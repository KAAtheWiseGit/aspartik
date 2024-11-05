use crate::state::State;

// modules:
//
// - compound: combine several distributions
//
// - priors:
//
//   - Beta
//   - Chi^2
//   - Dirichlet
//   - Exponential
//   - Gamma
//   - Inverse Gamma
//   - Laplace
//   - Log normal
//   - Normal
//   - One on X
//   - Poisson
//   - Uniform
//
// - Tree likelihood.  This one might need a separate module or an
//   implementation in another crate.

pub trait Probability {
	// Because we pass `State` here, this can be implemented for both
	// parameter priors and for the tree likelihood.
	// TODO: log or not?
	fn probability(&self, state: &State) -> f64;
}
