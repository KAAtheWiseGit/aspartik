use anyhow::Result;
use rand::Rng;

use crate::operator::{Operator, Proposal};
use crate::{
	tree::{Internal, Tree},
	State,
};

pub struct NarrowExchange {}

impl NarrowExchange {
	pub fn new() -> Box<dyn Operator> {
		Box::new(Self {})
	}
}

impl Operator for NarrowExchange {
	fn propose(&self, state: &mut State) -> Result<Proposal> {
		let rng = &mut state.rng;
		let tree = &mut state.tree;

		if tree.num_internals() < 2 {
			return Ok(Proposal::Reject);
		}

		// An internal node which has at least one internal node child.
		let grandparent = loop {
			let internal = tree.sample_internal(rng);
			if is_grandparent(tree, internal) {
				break internal;
			}
		};

		let (left, right) = tree.children_of(grandparent);

		let (parent, uncle) =
			if tree.weight_of(left) > tree.weight_of(right) {
				(right, left)
			} else {
				(left, right)
			};

		if tree.weight_of(parent) == tree.weight_of(uncle) {
			return Ok(Proposal::Reject);
		}
		// Uncle must be lower than the parent
		assert!(tree.weight_of(uncle) > tree.weight_of(parent));

		// If the lower child isn't internal, abort.
		let Some(parent) = tree.as_internal(parent) else {
			return Ok(Proposal::Reject);
		};
		let Some(uncle) = tree.as_internal(uncle) else {
			return Ok(Proposal::Reject);
		};

		let num_grandparents_before: usize = tree
			.internals()
			.map(|node| is_grandparent(tree, node))
			.map(|is_gp| is_gp as usize)
			.sum();
		let before = is_grandparent(tree, parent) as usize
			+ is_grandparent(tree, uncle) as usize;

		let child = if rng.random_bool(0.5) {
			tree.children_of(parent).0
		} else {
			tree.children_of(parent).1
		};

		tree.swap_parents(uncle.into(), child);

		let after = is_grandparent(tree, parent) as usize
			+ is_grandparent(tree, uncle) as usize;

		let num_grandparents_after =
			num_grandparents_before - before + after;
		let ratio = (num_grandparents_before as f64
			/ num_grandparents_after as f64)
			.ln();

		Ok(Proposal::Hastings(ratio))
	}
}

fn is_grandparent(tree: &Tree, node: Internal) -> bool {
	let (left, right) = tree.children_of(node);
	tree.is_internal(left) || tree.is_internal(right)
}

pub struct WideExchange {}

impl WideExchange {
	pub fn new() -> Box<dyn Operator> {
		Box::new(Self {})
	}
}

impl Operator for WideExchange {
	fn propose(&self, state: &mut State) -> Result<Proposal> {
		let rng = &mut state.rng;
		let tree = &mut state.tree;

		let i = tree.sample_node(rng);
		let j = loop {
			let out = tree.sample_node(rng);
			if out != i {
				break out;
			}
		};

		let Some(i_parent) = tree.parent_of(i) else {
			return Ok(Proposal::Reject);
		};
		let Some(j_parent) = tree.parent_of(j) else {
			return Ok(Proposal::Reject);
		};

		if j != i_parent.into()
			&& tree.weight_of(j) < tree.weight_of(i_parent.into())
			&& tree.weight_of(i) < tree.weight_of(j_parent.into())
		{
			tree.swap_parents(i, j);

			Ok(Proposal::Hastings(0.0))
		} else {
			Ok(Proposal::Reject)
		}
	}
}
