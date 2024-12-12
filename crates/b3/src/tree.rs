use rand::{
	distributions::{Distribution, Uniform},
	Rng,
};

use std::collections::{HashSet, VecDeque};

use crate::operator::Proposal;
use io::newick::{
	Node as NewickNode, NodeIndex as NewickNodeIndex, Tree as NewickTree,
};
use shchurvec::ShchurVec;

const ROOT: usize = usize::MAX;

pub struct Tree {
	children: ShchurVec<usize>,
	parents: ShchurVec<usize>,
	weights: ShchurVec<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone)]
pub struct Update {
	pub edges: Vec<usize>,
	pub lengths: Vec<f64>,

	pub nodes: Vec<usize>,
	pub children: Vec<usize>,
}

impl Tree {
	pub fn new(weights: &[f64], children: &[usize]) -> Self {
		let mut out = Self {
			children: children.into(),
			parents: ShchurVec::repeat(ROOT, weights.len()),
			weights: weights.into(),
		};

		out.update_all_parents();
		out.accept();

		out
	}

	pub fn propose(&mut self, proposal: Proposal) -> Update {
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

		let (edges, lengths) = self.update_substitutions(&edges);
		let (nodes, children) = self.update_probabilities(&nodes);

		Update {
			edges,
			lengths,
			nodes,
			children,
		}
	}

	pub fn accept(&mut self) {
		self.children.accept();
		self.parents.accept();
		self.weights.accept();
	}

	pub fn reject(&mut self) {
		self.children.reject();
		self.parents.reject();
		self.weights.reject();
	}

	fn update_probabilities(
		&self,
		nodes: &[Node],
	) -> (Vec<usize>, Vec<usize>) {
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

		let nodes: Vec<_> = deq.into();

		let mut children = Vec::with_capacity(nodes.len() * 2);
		for node in &nodes {
			let n = node - self.num_leaves();
			children.push(n * 2);
			children.push(n * 2 + 1);
		}

		(nodes, children)
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

	pub fn update_all_likelihoods(&self) -> Update {
		let edges: Vec<usize> = (0..self.children.len()).collect();
		let (edges, lengths) = self.update_substitutions(&edges);

		let nodes: Vec<Node> = self.nodes().collect();
		let (nodes, children) = self.update_probabilities(&nodes);

		Update {
			edges,
			lengths,
			nodes,
			children,
		}
	}

	fn update_substitutions(
		&self,
		edges: &[usize],
	) -> (Vec<usize>, Vec<f64>) {
		let distances: Vec<f64> = edges
			.iter()
			.copied()
			.map(|e| {
				let child = self.children[e];
				let parent = e / 2 + self.num_leaves();

				self.weights[parent] - self.weights[child]
			})
			.collect();

		(edges.to_vec(), distances)
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
				"Node {} ({}) is lower than it's left child {} ({})",
				node.0,
				self.weight_of(node),
				left.0,
				self.weight_of(left),
			);
			assert!(
				self.weight_of(node) > self.weight_of(right),
				"Node {} ({}) is lower than it's right child {} ({})",
				node.0,
				self.weight_of(node),
				left.0,
				self.weight_of(right),
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

	pub fn serialize(&self) -> String {
		let mut tree = NewickTree::new();

		use std::collections::HashMap;
		let mut map: HashMap<Node, NewickNodeIndex> = HashMap::new();

		for node in self.nodes() {
			let newick_node = tree.add_node(NewickNode::new(
				node.0.to_string(),
				"".to_owned(),
				self.weight_of(node),
			));

			map.insert(node, newick_node);
		}

		for parent in self.internals() {
			let (left, right) = self.children_of(parent);

			tree.add_edge(map[&parent.into()], map[&left]);
			tree.add_edge(map[&parent.into()], map[&right]);

			// set root
			if self.parent_of(parent).is_none() {
				tree.set_root(map[&parent.into()]);
			}
		}

		tree.serialize()
	}
}
