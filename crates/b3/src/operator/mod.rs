use rand_xoshiro::Xoshiro256StarStar;

use std::collections::HashMap;

use crate::{parameter::Parameter, state::State, tree::Node};

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
mod exchange;
mod scale;
mod slide;

pub mod scheduler;

pub use exchange::NarrowExchange;
pub use scale::Scale;
pub use slide::Slide;

#[derive(Debug, Clone, Copy)]
pub enum Status {
	Accept,
	Reject,
	Hastings(f64),
}

type NodeWeight = f64;

pub type Rng = Xoshiro256StarStar;

#[derive(Debug, Clone, Default)]
pub struct TreeEdit {
	/// Update the weight of nodes on the left to values on the right.
	pub weights: Vec<(Node, NodeWeight)>,
	/// Subtree pruning and regrafting.  The parent of the left node is
	/// removed and regrafted between the right node and its parent.
	pub spr: Option<(Node, Node)>,
}

#[derive(Debug, Clone)]
pub struct Proposal {
	pub status: Status,
	/// A hash map of parameters updated by the operator.
	pub params: HashMap<String, Parameter>,
	/// A proposed edit of the tree.
	pub tree: TreeEdit,
}

impl Proposal {
	pub fn reject() -> Self {
		Self {
			status: Status::Reject,
			params: HashMap::new(),
			tree: TreeEdit::default(),
		}
	}

	pub fn accept() -> Self {
		Self {
			status: Status::Reject,
			..Proposal::reject()
		}
	}
}

pub trait Operator {
	fn propose(&self, state: &State, rng: &mut Rng) -> Proposal;
}
