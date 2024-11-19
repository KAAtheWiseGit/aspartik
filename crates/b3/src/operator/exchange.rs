use rand::Rng;

use super::{Operator, Proposal, Rng as RngT};
use crate::{
	state::State,
	tree::{Internal, Tree},
};

pub struct NarrowExchange();

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

		let grandparent_to_uncle = tree.edge_index(uncle);
		let parent_to_child = tree.edge_index(child);

		Proposal::hastings(0.0).with_edges(vec![
			// Redirect the edge coming out from grandparent from
			// uncle to the child
			(grandparent_to_uncle, child),
			// Redirect the edge from parent to the uncle
			(parent_to_child, uncle),
		])
	}
}

fn is_grandparent(tree: &Tree, node: Internal) -> bool {
	let (left, right) = tree.children_of(node);
	tree.is_internal(left) || tree.is_internal(right)
}
