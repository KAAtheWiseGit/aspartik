// Constructor operators return `Box<dyn Operator>`.
#![allow(clippy::new_ret_no_self)]

use anyhow::Result;
use rand_pcg::Pcg64;

use crate::State;

// Operators:
// - [ ] Parameter operators
//   - [/] Scaling with a distribution.
//   - [ ] "Bit flips" for boolean parameters
//   - [ ] Uniform selection in parameter bounds
//   - [ ] Swap operator for exchanging dimension values
//   - [ ] Regular and Bactrian versions:
//     - [ ] Delta exchange
//     - [ ] Random walk (+/- delta)
//     - [ ] "Up/down": scale two parameters using the same coefficient
// - [ ] Tree operators
//   - [x] Epoch scaling
//   - [x] Exchange: swap two branches, narrow and wide variations
//   - [ ] Random node height, reconstructs the tree
//   - [ ] Scaling
//   - [x] Uniform: move node height between parent and children
//   - [ ] Subtree sliding: same as above, but the node can be swapped with
//     parents
//   - [ ] Leaf node moving and scaling
//   - [ ] Wilson-Balding branch swapping move
//   - [ ] Bactrian versions for all distribution dependent operators from the
//     above
mod param;
mod tree;

pub mod scheduler;

pub use param::Scale as ParamScale;
pub use tree::Scale as TreeScale;
pub use tree::Slide as TreeSlide;
pub use tree::{
	NarrowExchange as TreeNarrowExchange, WideExchange as TreeWideExchange,
};

pub type Rng = Pcg64;

#[derive(Debug, Clone)]
pub enum Proposal {
	Accept,
	Reject,
	Hastings(f64),
}

pub trait Operator {
	fn propose(&self, state: &mut State) -> Result<Proposal>;
}
