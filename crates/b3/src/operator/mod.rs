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
	// TODO: decide if it should be log or not
	Hastings(f64),
}

// TODO: node height type, tree nodo type
type NodeHeight = ();
type NodeIndex = ();

pub struct TreeEdit {
	heights: HashMap<NodeIndex, NodeHeight>,
	// TODO: verify this is enough for arbitrary edits
	/// The key is the old child.  The value is the new child.
	children: HashMap<NodeIndex, NodeIndex>,
}

pub struct Proposal {
	status: Status,
	/// A hash map of parameters updated by the operator.
	params: HashMap<String, Parameter>,
	// Might require additional optimisations, so it's separate
	wree: Option<TreeEdit>,
}

impl Proposal {
	pub fn status(&self) -> Status {
		self.status
	}

	pub fn params(&self) -> &HashMap<String, Parameter> {
		&self.params
	}

	pub fn take_params(&mut self) -> HashMap<String, Parameter> {
		std::mem::take(&mut self.params)
	}
}

pub trait Operator {
	// TODO: rng
	fn propose(state: &State) -> Proposal;
}
