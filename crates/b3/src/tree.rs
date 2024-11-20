use rand::{
	distributions::{Distribution, Uniform},
	Rng,
};
use serde_json::{json, Value as Json};

use std::collections::{HashSet, VecDeque};

use crate::{likelihood::Likelihood, operator::Proposal};
use base::{seq::DnaSeq, substitution::dna::Dna4Substitution};
use shchurvec::ShchurVec;

const ROOT: usize = usize::MAX;

pub struct Tree {
	children: ShchurVec<usize>,
	parents: ShchurVec<usize>,
	weights: ShchurVec<f64>,

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
		let mut c1 = vec![];
		let mut c2 = vec![];
		let mut c3 = vec![];
		let mut c4 = vec![];

		for i in 0..sequences[0].len() {
			let mut column = Vec::new();
			for sequence in sequences {
				column.push(sequence[i]);
			}

			if i % 4 == 0 {
				c1.push(column);
			} else if i % 4 == 1 {
				c2.push(column);
			} else if i % 4 == 2 {
				c3.push(column);
			} else {
				c4.push(column);
			}
		}

		let parents = ShchurVec::repeat(ROOT, weights.len());

		let likelihoods = vec![
			Likelihood::new(c1, Dna4Substitution::jukes_cantor()),
			Likelihood::new(c2, Dna4Substitution::jukes_cantor()),
			Likelihood::new(c3, Dna4Substitution::jukes_cantor()),
			Likelihood::new(c4, Dna4Substitution::jukes_cantor()),
		];

		let mut out = Self {
			children: children.into(),
			parents,
			weights: weights.into(),

			likelihoods,
		};

		out.update_all_parents();
		out.update_all_likelihoods();
		out.accept();

		out
	}

	pub fn likelihood(&self) -> f64 {
		self.likelihoods.iter().map(|l| l.likelihood()).sum()
	}

	pub fn propose(&mut self, proposal: Proposal) {
		let (mut edges, mut nodes) = self.update_edges(&proposal.edges);

		// TODO: should probably be a separate function
		for (node, weight) in proposal.weights {
			self.weights.set(node.0, weight);

			nodes.push(node);
			if self.parent_of(node).is_some() {
				edges.push(self.edge_index(node));
			}
			if let Some(node) = self.as_internal(node) {
				let (left, right) = self.children_of(node);
				edges.push(self.edge_index(left));
				edges.push(self.edge_index(right));
			}
		}

		self.verify();

		self.update_substitutions(&edges);
		self.update_probabilities(&nodes);
	}

	pub fn accept(&mut self) {
		self.children.accept();
		self.parents.accept();
		self.weights.accept();

		for likelihood in &mut self.likelihoods {
			likelihood.accept();
		}
	}

	pub fn reject(&mut self) {
		self.children.reject();
		self.parents.reject();
		self.weights.reject();

		for likelihood in &mut self.likelihoods {
			likelihood.reject();
		}
	}

	fn update_probabilities(&mut self, nodes: &[Node]) {
		let mut deq = VecDeque::<usize>::new();
		let mut set = HashSet::<usize>::new();

		for node in nodes {
			let mut curr = node.0;
			let mut chain = Vec::new();

			// Walk up from the starting nodes until the root, stop
			// when we encounter a node we have already walked.
			while !set.contains(&curr) && curr != ROOT {
				set.insert(curr);

				// If the node is internal, add it to the
				// current chain.
				if curr >= self.num_leaves() {
					chain.push(curr);
				}

				curr = self.parents[curr];
			}

			// Prepend the chain to the deque.  The first chain will
			// insert the root node and walk backwards.  All of the
			// rest will also go in the front, ensuring that
			// children always go befor their parents.
			while let Some(val) = chain.pop() {
				deq.push_front(val);
			}
		}

		let nodes = deq.make_contiguous();
		let children: Vec<_> = nodes
			.iter()
			.map(|n| n - self.num_leaves())
			.map(|i| {
				(self.children[i * 2], self.children[i * 2 + 1])
			})
			.collect();

		let num_leaves = self.num_leaves();

		use rayon::prelude::*;
		self.likelihoods.par_iter_mut().for_each(|likelihood| {
			likelihood.update_probabilities(
				num_leaves, nodes, &children,
			);
		});
	}

	fn update_edges(
		&mut self,
		edges: &[(usize, Node)],
	) -> (Vec<usize>, Vec<Node>) {
		let mut e = vec![];
		let mut n = vec![];

		for (edge, new_child) in edges.iter().copied() {
			let (_, parent) = self.edge_nodes(edge);

			self.children.set(edge, new_child.0);
			self.parents.set(new_child.0, parent.0);

			e.push(edge);

			// `parent` is now the parent of `new_child`, so it'll
			// be updated.  The old child must be handled separately
			// by the operator.
			n.push(new_child);
		}

		(e, n)
	}

	fn update_all_parents(&mut self) {
		let num_leaves = self.num_leaves();

		let mut iter = self.children.into_iter();
		let mut i = 0;
		while let (Some(left), Some(right)) = (iter.next(), iter.next())
		{
			self.parents.set(*left, i + num_leaves);
			self.parents.set(*right, i + num_leaves);
			i += 1;
		}
	}

	fn update_all_likelihoods(&mut self) {
		let edges: Vec<usize> = (0..self.children.len()).collect();
		self.update_substitutions(&edges);

		let nodes: Vec<Node> = self.nodes().collect();
		self.update_probabilities(&nodes);
	}

	fn update_substitutions(&mut self, edges: &[usize]) {
		let distances: Vec<f64> = edges
			.iter()
			.copied()
			.map(|e| {
				let child = self.children[e];
				let parent = e / 2 + self.num_leaves();

				self.weights[parent] - self.weights[child]
			})
			.collect();

		for likelihood in &mut self.likelihoods {
			likelihood.update_substitutions(edges, &distances);
		}
	}

	fn verify(&self) {
		for (i, parent) in self.parents.iter().enumerate() {
			assert!(
				*parent >= self.num_leaves(),
				"Leaf {} became a parent of {}",
				parent,
				i
			)
		}

		for node in self.internals() {
			let (left, right) = self.children_of(node);

			assert!(
				self.weight_of(node) > self.weight_of(left),
				"Node {} is lower than it's left child {}",
				node.0,
				left.0
			);
			assert!(
				self.weight_of(node) > self.weight_of(right),
				"Node {} is lower than it's right child {}",
				node.0,
				left.0
			);
		}
	}

	pub fn num_nodes(&self) -> usize {
		self.weights.len()
	}

	pub fn num_internals(&self) -> usize {
		(self.num_nodes() - 1) / 2
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

	/// Index of the edge between `child` and its parent.
	pub fn edge_index(&self, child: Node) -> usize {
		let parent = self.parent_of(child).unwrap();

		if self.children_of(parent).0 == child {
			(parent.0 - self.num_leaves()) * 2
		} else {
			(parent.0 - self.num_leaves()) * 2 + 1
		}
	}

	fn edge_nodes(&self, edge: usize) -> (Node, Internal) {
		let parent = edge / 2 + self.num_leaves();
		let child = self.children[edge];

		(Node(child), Internal(parent))
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
			"children": "TODO",
			"weights": "TODO",
		})
	}
}
