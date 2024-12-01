use rand::Rng;

use super::{Operator, Proposal, Rng as RngT};
use crate::{
	state::State,
	tree::{Internal, Tree},
};

pub struct NarrowExchange {}

impl NarrowExchange {
	pub fn new() -> Box<dyn Operator> {
		Box::new(Self {})
	}
}

impl Operator for NarrowExchange {
	fn propose(&self, state: &State, rng: &mut RngT) -> Proposal {
		let tree = state.get_tree();

		if tree.num_internals() < 2 {
			return Proposal::reject();
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
			if tree.weight_of(left) < tree.weight_of(right) {
				(right, left)
			} else {
				(left, right)
			};

		// If the lower child isn't internal, abort.
		let Some(parent) = tree.as_internal(parent) else {
			return Proposal::reject();
		};

		// TODO: proper Hastings ratio
		let _num_grandparents_before: usize = tree
			.internals()
			.map(|node| is_grandparent(tree, node))
			.map(|is_gp| is_gp as usize)
			.sum();

		let child = if rng.gen_bool(0.5) {
			tree.children_of(parent).0
		} else {
			tree.children_of(parent).1
		};

		Proposal::hastings(0.0)
			.with_replacement(tree, grandparent, uncle, child)
			.with_replacement(tree, parent, child, uncle)
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
	fn propose(&self, state: &State, rng: &mut RngT) -> Proposal {
		let tree = state.get_tree();

		let i = tree.sample_node(rng);
		let j = loop {
			let out = tree.sample_node(rng);
			if out != i {
				break out;
			}
		};

		let Some(i_parent) = tree.parent_of(i) else {
			return Proposal::reject();
		};
		let Some(j_parent) = tree.parent_of(j) else {
			return Proposal::reject();
		};

		if j != i_parent.into()
			&& tree.weight_of(j) < tree.weight_of(i_parent)
			&& tree.weight_of(i) < tree.weight_of(j_parent)
		{
			Proposal::hastings(0.0)
				.with_replacement(tree, i_parent, i, j)
				.with_replacement(tree, j_parent, j, i)
		} else {
			Proposal::reject()
		}
	}
}
