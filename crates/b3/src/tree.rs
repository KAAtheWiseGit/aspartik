use rand::{
	distributions::{Distribution, Uniform},
	Rng,
};
use serde_json::{json, Value as Json};

use crate::{likelihood::Likelihood, operator::TreeEdit};
use base::{seq::DnaSeq, substitution::dna::Dna4Substitution};

const ROOT: usize = usize::MAX;

pub struct Tree {
	children: Vec<usize>,
	parents: Vec<usize>,
	weights: Vec<f64>,

	likelihoods: Vec<Likelihood<Dna4Substitution>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Node(usize);

impl From<Internal> for Node {
	fn from(internal: Internal) -> Node {
		Self(internal.0)
	}
}

impl From<Leaf> for Node {
	fn from(leaf: Leaf) -> Node {
		Node(leaf.0)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Internal(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Leaf(usize);

impl Tree {
	pub fn new(
		// TODO: per-site models
		sequences: &[DnaSeq],
		weights: &[f64],
		children: &[usize],
	) -> Self {
		let mut columns = Vec::new();
		for i in 0..sequences[0].len() {
			let mut column = Vec::new();
			for sequence in sequences {
				column.push(sequence[i]);
			}
			columns.push(column);
		}

		let num_leaves = columns[0].len();
		let num_nodes = weights.len();

		let mut parents = vec![ROOT; num_nodes];
		while let Some((i, [left, right])) =
			children.chunks(2).enumerate().next()
		{
			parents[*left] = i + num_leaves;
			parents[*right] = i + num_leaves;
		}

		let likelihoods = vec![Likelihood::new(
			columns,
			Dna4Substitution::jukes_cantor(),
		)];

		Self {
			children: children.to_vec(),
			parents,
			weights: weights.to_vec(),

			likelihoods,
		}
	}

	pub fn likelihood(&self) -> f64 {
		self.likelihoods.iter().map(|l| l.likelihood()).sum()
	}

	pub fn update_with(&mut self, _edit: TreeEdit) -> TreeEdit {
		todo!()
	}


	pub fn num_nodes(&self) -> usize {
		self.weights.len()
	}

	pub fn num_internals(&self) -> usize {
		self.children.len()
	}

	pub fn num_leaves(&self) -> usize {
		self.num_internals() + 1
	}

	pub fn is_internal<N: Into<Node>>(&self, node: N) -> bool {
		node.into().0 >= self.num_leaves()
	}

	pub fn is_leaf<N: Into<Node>>(&self, node: N) -> bool {
		node.into().0 < self.num_leaves()
	}

	pub fn as_internal<N: Into<Node>>(&self, node: N) -> Option<Internal> {
		let node = node.into();
		if self.is_internal(node) {
			Some(Internal(node.0))
		} else {
			None
		}
	}

	pub fn as_leaf<N: Into<Node>>(&self, node: N) -> Option<Leaf> {
		let node = node.into();
		if self.is_leaf(node) {
			Some(Leaf(node.0))
		} else {
			None
		}
	}

	/// Returns the index of the root node.
	pub fn root(&self) -> Internal {
		// There must always be a rooted element in the tree.
		let i = self.parents.iter().position(|p| *p == ROOT).unwrap();
		Internal(i)
	}

	pub fn weight_of<N: Into<Node>>(&self, node: N) -> f64 {
		self.weights[node.into().0]
	}

	pub fn children_of(&self, node: Internal) -> (Node, Node) {
		let index = node.0 - self.num_leaves();
		let left = self.children[index * 2];
		let right = self.children[index * 2 + 1];

		(Node(left), Node(right))
	}

	/// Returns the parent of `node`, or `None` if the node is the root of
	/// the tree.
	pub fn parent_of<N: Into<Node>>(&self, node: N) -> Option<Internal> {
		Some(self.parents[node.into().0])
			.take_if(|p| *p != ROOT)
			.map(Internal)
	}

	pub fn sample_node<R: Rng + ?Sized>(&self, rng: &mut R) -> Node {
		let range = Uniform::from(0..self.num_nodes());
		let i = range.sample(rng);
		Node(i)
	}

	pub fn sample_internal<R: Rng + ?Sized>(
		&self,
		rng: &mut R,
	) -> Internal {
		let range = Uniform::from(self.num_leaves()..self.num_nodes());
		let i = range.sample(rng);
		Internal(i)
	}

	pub fn sample_leaf<R: Rng + ?Sized>(&self, rng: &mut R) -> Leaf {
		let range = Uniform::from(0..self.num_leaves());
		let i = range.sample(rng);
		Leaf(i)
	}

	pub fn nodes(&self) -> impl Iterator<Item = Node> {
		(0..self.num_nodes()).map(Node)
	}

	pub fn internals(&self) -> impl Iterator<Item = Internal> {
		(self.num_leaves()..self.num_nodes()).map(Internal)
	}

	pub fn leaves(&self) -> impl Iterator<Item = Leaf> {
		(0..self.num_leaves()).map(Leaf)
	}

	pub fn serialize(&self) -> Json {
		json!({
			"children": self.children,
			"weights": self.weights,
		})
	}
}
