use rand_xoshiro::Xoshiro256StarStar;

use std::collections::HashMap;

use crate::{parameter::Parameter, state::State};

// Operators:
// - [ ] Parameter operators
//   - [ ] "Bit flips" for boolean parameters
//   - [ ] Uniform selection in parameter bounds
//   - [ ] Swap operator for exchanging dimension values
//   - [ ] Regular and Bactrian versions:
//     - [ ] Delta exchange
//     - [ ] Random walk (+/- delta)
//     - [ ] "Up/down": scale two parameters using the same coefficient
// - [ ] Tree operators
//   - [ ] Epoch scaling
//   - [ ] Exchange: swap two branches, narrow and wide variations
//   - [ ] Random node height, reconstructs the tree
//   - [ ] Scaling
//   - [ ] Uniform: move node height between parent and children
//   - [ ] Subtree sliding: same as above, but the node can be swapped with
//     parents
//   - [ ] Leaf node moving and scaling
//   - [ ] Wilson-Balding branch swapping move
//   - [ ] Bactrian versions for all distribution dependent operators from the
//     above

#[derive(Debug, Clone, Copy)]
pub enum Status {
	Accept,
	Reject,
	Hastings(f64),
}

type NodeWeight = f64;
type NodeIndex = usize;

pub type Rng = Xoshiro256StarStar;

#[derive(Default, Debug, Clone)]
pub struct TreeEdit {
	/// Update the weight of nodes on the left to values on the right.
	pub weights: Vec<(NodeIndex, NodeWeight)>,
	/// Swap the parents of nodes on the left and on the right.
	pub parents: Vec<(NodeIndex, NodeIndex)>,
}

#[derive(Debug, Clone)]
pub struct Proposal {
	pub status: Status,
	/// A hash map of parameters updated by the operator.
	pub params: HashMap<String, Parameter>,
	/// A proposed edit of the tree.
	pub tree: TreeEdit,
}

pub trait Operator {
	fn propose(state: &State, rng: &mut Rng) -> Proposal;
}
