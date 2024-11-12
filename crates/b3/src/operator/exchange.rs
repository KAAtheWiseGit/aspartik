use rand::{
	distributions::{Distribution, Uniform},
	Rng,
};

use std::collections::HashMap;

use super::{Operator, Proposal, Rng as RngT, Status, TreeEdit};
use crate::{state::State, tree::Tree};

pub struct NarrowExchange();

impl Operator for NarrowExchange {
	fn propose(&self, state: &State, rng: &mut RngT) -> Proposal {
		let tree = state.get_tree();

		if tree.num_internals() < 2 {
			return Proposal::reject();
		}

		let range = Uniform::from(tree.num_leaves()..tree.num_nodes());

		// An internal node which has at least one internal node child.
		let grandparent = loop {
			let node = range.sample(rng);
			if is_grandparent(tree, node) {
				break node;
			}
		};

		let (left, right) = tree.children_of(grandparent).unwrap();

		let (parent, uncle) =
			if tree.weight_of(left) < tree.weight_of(right) {
				(right, left)
			} else {
				(left, right)
			};

		// If the lower child isn't internal, abort.
		if tree.is_leaf(parent) {
			return Proposal::reject();
		}

		// TODO: proper Hastings ratio
		let _num_grandparents_before: usize = (tree.num_leaves()
			..tree.num_nodes())
			.map(|node| is_grandparent(tree, node))
			.map(|is_gp| is_gp as usize)
			.sum();

		let child = if rng.gen_bool(0.5) {
			tree.children_of(parent).unwrap().0
		} else {
			tree.children_of(parent).unwrap().1
		};

		Proposal {
			status: Status::Hastings(0.0),
			params: HashMap::new(),
			tree: TreeEdit {
				parents: vec![(child, uncle)],
				weights: vec![],
			},
		}
	}
}

fn is_grandparent(tree: &Tree, node: usize) -> bool {
	let Some((left, right)) = tree.children_of(node) else {
		return false;
	};

	tree.is_internal(left) || tree.is_internal(right)
}
