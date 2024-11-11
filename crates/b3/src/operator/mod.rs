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

#[derive(Default, Debug, Clone)]
pub struct TreeEdit {
	/// Update the weight of nodes on the left to values on the right.
	weights: Vec<(NodeIndex, NodeWeight)>,
	/// Update the parents of nodes on the left to nodes specified on the
	/// right.
	parents: Vec<(NodeIndex, NodeIndex)>,
}

impl TreeEdit {
	pub fn weights(&self) -> &[(NodeIndex, NodeWeight)] {
		&self.weights
	}

	pub fn parents(&self) -> &[(NodeIndex, NodeIndex)] {
		&self.parents
	}
}

#[derive(Debug, Clone)]
pub struct Proposal {
	status: Status,
	/// A hash map of parameters updated by the operator.
	params: HashMap<String, Parameter>,
	// Might require additional optimisations, so it's separate
	tree: Option<TreeEdit>,
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

	pub fn take_tree_edit(&mut self) -> Option<TreeEdit> {
		std::mem::take(&mut self.tree)
	}
}

pub trait Operator {
	// TODO: rng
	fn propose(state: &State) -> Proposal;
}
