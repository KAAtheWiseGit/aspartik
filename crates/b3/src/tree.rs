use rand::{
	distr::{Distribution, Uniform},
	Rng,
};
use serde::{Deserialize, Serialize};

use std::collections::{HashSet, VecDeque};

use io::newick::{
	Node as NewickNode, NodeIndex as NewickNodeIndex, Tree as NewickTree,
};
use shchurvec::ShchurVec;

const ROOT: usize = usize::MAX;

#[derive(Debug)]
pub struct Tree {
	#[allow(dead_code)]
	names: Vec<String>,

	children: ShchurVec<usize>,
	parents: ShchurVec<usize>,
	weights: ShchurVec<f64>,

	updated_edges: Vec<usize>,
	updated_nodes: Vec<Node>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Node(usize);

impl From<Internal> for Node {
	fn from(internal: Internal) -> Node {
		Self(internal.0)
	}
}

impl Internal {
	pub fn to_index(self) -> usize {
		self.0
	}
}

impl From<Leaf> for Node {
	fn from(leaf: Leaf) -> Node {
		Node(leaf.0)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Internal(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Leaf(usize);

impl Tree {
	pub fn new(
		names: Vec<String>,
		weights: &[f64],
		children: &[usize],
	) -> Self {
		let mut out = Self {
			names,

			children: children.into(),
			parents: ShchurVec::repeat(ROOT, weights.len()),
			weights: weights.into(),

			updated_edges: Vec::new(),
			updated_nodes: Vec::new(),
		};

		out.set_all_parents();

		out
	}

	pub fn accept(&mut self) {
		self.children.accept();
		self.parents.accept();
		self.weights.accept();
		self.clear_updated();
	}

	pub fn reject(&mut self) {
		self.children.reject();
		self.parents.reject();
		self.weights.reject();
		self.clear_updated();
	}

	fn clear_updated(&mut self) {
		self.updated_edges.clear();
		self.updated_nodes.clear();
	}

	pub fn edges_to_update(&self) -> Vec<usize> {
		self.updated_edges.clone()
	}

	// should retrun `Internal`
	pub fn nodes_to_update(&self) -> Vec<Internal> {
		self.walk_nodes(&self.updated_nodes)
	}

	pub fn full_update(&self) -> Vec<Internal> {
		let internals: Vec<Node> =
			self.internals().map(|n| n.into()).collect();
		self.walk_nodes(&internals)
	}

	pub fn to_lists(
		&self,
		nodes: &[Internal],
	) -> (Vec<usize>, Vec<usize>, Vec<usize>) {
		let mut out_nodes = Vec::new();
		let mut edges = Vec::new();
		let mut children = Vec::new();

		for node in nodes {
			out_nodes.push(node.0);
			let (left, right) = self.children_of(*node);
			children.push(left.0);
			children.push(right.0);

			edges.push(self.edge_index(left));
			edges.push(self.edge_index(right));
		}

		(out_nodes, edges, children)
	}

	fn walk_nodes(&self, nodes: &[Node]) -> Vec<Internal> {
		let mut deq = VecDeque::<Internal>::new();
		let mut set = HashSet::<Internal>::new();

		for node in nodes {
			let mut chain = Vec::new();
			let mut curr =
				self.as_internal(*node).unwrap_or_else(|| {
					self.parent_of(*node).unwrap()
				});

			// Walk up from the starting nodes until the root, stop
			// when we encounter a node we have already walked.
			loop {
				if set.contains(&curr) {
					break;
				}

				set.insert(curr);
				chain.push(curr);

				if let Some(parent) = self.parent_of(curr) {
					curr = parent;
				} else {
					break;
				}
			}

			// Prepend the chain to the deque.  The first chain will
			// insert the root node and walk backwards.  All of the
			// rest will also go in the front, ensuring that
			// children always go befor their parents.
			while let Some(node) = chain.pop() {
				deq.push_front(node);
			}
		}

		deq.into()
	}

	/// Set the child of `edge` to `node`.
	///
	/// Doesn't do any validation.
	pub fn update_edge(&mut self, edge: usize, new_child: Node) {
		let (_, parent) = self.edge_nodes(edge);

		self.children.set(edge, new_child.0);
		self.parents.set(new_child.0, parent.0);

		self.updated_edges.push(edge);

		// `parent` is now the parent of `new_child`, so it'll
		// be updated.  The operator must handle the old node
		// separately.
		self.updated_nodes.push(new_child);
	}

	/// Set the weight of `node` to `weight`.
	///
	/// Takes care of book-keeping the parent and child edge updates.
	pub fn update_weight(&mut self, node: Node, weight: f64) {
		self.weights.set(node.0, weight);
		self.updated_nodes.push(node);

		if self.parent_of(node).is_some() {
			self.updated_edges.push(self.edge_index(node));
		}
		if let Some(node) = self.as_internal(node) {
			let (left, right) = self.children_of(node);
			self.updated_edges.push(self.edge_index(left));
			self.updated_edges.push(self.edge_index(right));
		}
	}

	/// Make `node` the root of the tree.
	///
	/// The old root must be updated in a separate `update_edge` call.
	pub fn update_root(&mut self, node: Node) {
		self.parents.set(node.0, ROOT);
	}

	/// Replaces `child` with `replacement` in respect to `child`'s parent.
	pub fn update_replacement(&mut self, child: Node, replacement: Node) {
		let edge = self.edge_index(child);
		self.update_edge(edge, replacement);
	}

	/// Swaps the parents of nodes `a` and `b`.
	///
	/// The parent of `a` becomes the parent of `b` and visa versa.  If `a`
	/// and `b` share the same parent, they switch polarity (left <->
	/// right).
	// TODO: invariants (a can't be a parent of b)
	pub fn swap_parents(&mut self, a: Node, b: Node) {
		assert!(self.parent_of(a).is_some(), "a must not be root");
		assert!(self.parent_of(b).is_some(), "b must not be root");

		let edge_a = self.edge_index(a);
		let edge_b = self.edge_index(b);

		self.update_edge(edge_a, b);
		self.update_edge(edge_b, a);
	}

	fn set_all_parents(&mut self) {
		let num_leaves = self.num_leaves();

		let mut iter = self.children.into_iter();
		let mut i = 0;
		while let (Some(left), Some(right)) = (iter.next(), iter.next())
		{
			self.parents.set(*left, i + num_leaves);
			self.parents.set(*right, i + num_leaves);
			i += 1;
		}

		self.parents.accept();
	}

	pub fn verify(&self) {
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
				self.weight_of(node) < self.weight_of(left),
				"Node {} ({}) is lower than it's left child {} ({})",
				node.0,
				self.weight_of(node),
				left.0,
				self.weight_of(left),
			);
			assert!(
				self.weight_of(node) < self.weight_of(right),
				"Node {} ({}) is lower than it's right child {} ({})",
				node.0,
				self.weight_of(node),
				left.0,
				self.weight_of(right),
			);
		}

		use std::collections::HashSet;
		let mut children = HashSet::new();
		for node in self.internals() {
			let (left, right) = self.children_of(node);
			children.insert(left);
			children.insert(right);
		}
		assert_eq!(children.len(), self.num_nodes() - 1);
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

	pub fn edge_distance(&self, edge: usize) -> f64 {
		let (child, parent) = self.edge_nodes(edge);

		self.weight_of(child) - self.weight_of(parent)
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
		let range = Uniform::new(0, self.num_nodes()).unwrap();
		let i = range.sample(rng);
		Node(i)
	}

	pub fn sample_internal<R: Rng + ?Sized>(
		&self,
		rng: &mut R,
	) -> Internal {
		let range = Uniform::new(self.num_leaves(), self.num_nodes())
			.unwrap();
		let i = range.sample(rng);
		Internal(i)
	}

	pub fn sample_leaf<R: Rng + ?Sized>(&self, rng: &mut R) -> Leaf {
		let range = Uniform::new(0, self.num_leaves()).unwrap();
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

	pub fn into_newick(&self) -> String {
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

impl Serialize for Tree {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.serialize_str(&self.into_newick())
	}
}

impl<'de> Deserialize<'de> for Tree {
	fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		// Parse Newick tree from string
		todo!()
	}
}
