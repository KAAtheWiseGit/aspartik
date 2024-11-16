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

		let parents = vec![ROOT; weights.len()];

		let likelihoods = vec![Likelihood::new(
			columns,
			Dna4Substitution::jukes_cantor(),
		)];

		let mut out = Self {
			children: children.to_vec(),
			parents,
			weights: weights.to_vec(),

			likelihoods,
		};

		out.update_all_parents();
		out.update_all_likelihoods();

		out
	}

	pub fn likelihood(&self) -> f64 {
		self.likelihoods.iter().map(|l| l.likelihood()).sum()
	}

	pub fn update_with(&mut self, edit: TreeEdit) -> TreeEdit {
		let mut edges = vec![];
		let mut distances = vec![];
		let mut nodes = vec![];

		if let Some(spr) = edit.spr {
			let (mut e, mut d, mut n) =
				self.update_spr(spr.0, spr.1);
			edges.append(&mut e);
			distances.append(&mut d);
			nodes.append(&mut n);
		}

		let (nodes, children) = self.get_children_tuples(nodes);
		let num_leaves = self.num_leaves();

		for likelihood in &mut self.likelihoods {
			likelihood.update_substitutions(&edges, &distances);
			likelihood.update_probabilities(
				num_leaves, &nodes, &children,
			);
		}

		// TODO
		TreeEdit::default()
	}

	fn get_children_tuples(
		&self,
		nodes: Vec<Node>,
	) -> (Vec<usize>, Vec<(usize, usize)>) {
		let nodes_iter =
			nodes.into_iter().filter_map(|n| self.as_internal(n));

		let mut nodes = vec![];
		let mut children = vec![];
		for node in nodes_iter {
			nodes.push(node.0);
			let (left, right) = self.children_of(node);
			children.push((left.0, right.0))
		}

		(nodes, children)
	}

	pub fn update_spr(
		&mut self,
		s: Node,
		r_c: Node,
	) -> (Vec<usize>, Vec<f64>, Vec<Node>) {
		let mut edges = vec![];
		let mut distances = vec![];
		let mut nodes = vec![];

		let r_p = self.parent_of(r_c);
		let p = self.parent_of(s);

		if let Some(p) = p {
			// p: x, s -> p: r_c, s
			let x = self.other_child_of(p, s);
			let p_to_x = self.edge_index(p, x);
			self.children[p_to_x] = r_c.0;
			self.parents[r_c.0] = p.0;

			edges.push(p_to_x);
			distances.push(self.weight_of(p) - self.weight_of(r_c));
			nodes.push(p.into());

			if let Some(p_p) = self.parent_of(p) {
				// p_p: p, z -> p_p: x, z
				let p_p_to_p = self.edge_index(p_p, p.into());
				self.children[p_p_to_p] = x.0;
				self.parents[x.0] = p_p.0;

				edges.push(p_p_to_p);
				distances
					.push(self.weight_of(p_p)
						- self.weight_of(x));
				nodes.push(p_p.into());
			}
		}

		if let Some(r_p) = r_p {
			let r_p_to_r_c = self.edge_index(r_p, r_c);
			// TODO: figure out what to do if s is rooted.  It
			// should probably be forbidden.
			let p = p.unwrap();
			self.children[r_p_to_r_c] = p.0;
			self.parents[p.0] = r_p.0;

			edges.push(r_p_to_r_c);
			distances.push(self.weight_of(r_p) - self.weight_of(p));
			nodes.push(r_p.into());
		}

		(edges, distances, nodes)
	}

	fn update_all_parents(&mut self) {
		let num_leaves = self.num_leaves();
		let mut iter = self.children.chunks(2).enumerate();
		while let Some((i, [left, right])) = iter.next() {
			self.parents[*left] = i + num_leaves;
			self.parents[*right] = i + num_leaves;
		}
	}

	fn update_all_likelihoods(&mut self) {
		// TODO: deduplicate
		let mut nodes = vec![];
		let mut children = vec![];
		for node in self.internals() {
			nodes.push(node.0);
			let (left, right) = self.children_of(node);
			children.push((left.0, right.0))
		}

		// TODO: deduplicate
		let mut edges = vec![];
		let mut distances = vec![];
		for node in self.internals() {
			let (left, right) = self.children_of(node);

			let i = node.0 - self.num_leaves();

			edges.push(i * 2);
			distances
				.push(self.weight_of(node)
					- self.weight_of(left));
			edges.push(i * 2 + 1);
			distances
				.push(self.weight_of(node)
					- self.weight_of(right));
		}

		let num_leaves = self.num_leaves();
		for likelihood in &mut self.likelihoods {
			likelihood.update_substitutions(&edges, &distances);
			likelihood.update_probabilities(
				num_leaves, &nodes, &children,
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

	fn other_child_of(&self, node: Internal, child: Node) -> Node {
		if self.children_of(node).0 != child {
			self.children_of(node).0
		} else {
			self.children_of(node).1
		}
	}

	fn edge_index(&self, parent: Internal, child: Node) -> usize {
		if self.children_of(parent).0 == child {
			(parent.0 - self.num_leaves()) * 2
		} else {
			(parent.0 - self.num_leaves()) * 2 + 1
		}
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
