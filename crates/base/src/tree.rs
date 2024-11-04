use petgraph::{
	graph::{DiGraph, NodeIndex},
	visit::EdgeRef,
	Incoming,
};

#[derive(Clone, Debug, Default)]
pub struct Tree<N> {
	graph: DiGraph<N, ()>,
}

impl<N> Tree<N> {
	/// Number of nodes in the tree.
	pub fn size(&self) -> usize {
		self.graph.edge_count()
	}

	/// Adds a node with a `value` and returns it's index.
	pub fn add_node(&mut self, value: N) -> usize {
		self.graph.add_node(value).index()
	}

	/// Returns a parent of a node.  If the node has several parents, only
	/// one is returned.
	pub fn parent(&self, node: usize) -> Option<usize> {
		let node = NodeIndex::new(node);

		self.graph
			.edges_directed(node, Incoming)
			.next()
			.map(|e| e.source().index())
	}

	/// Remove the parent edge for `node`.  If there is no parent, it's a
	/// no-op.  If there are several parents, only one is removed.
	pub fn remove_parent(&mut self, node: usize) {
		let node = NodeIndex::new(node);

		if let Some(incoming) = self
			.graph
			.edges_directed(node, Incoming)
			.next()
			.map(|e| e.id())
		{
			self.graph.remove_edge(incoming);
		}
	}

	/// Remove all parents of a node.
	pub fn remove_parent_all(&mut self, node: usize) {
		while self
			.graph
			.edges_directed(NodeIndex::new(node), Incoming)
			.next()
			.is_some()
		{
			self.remove_parent(node);
		}
	}

	/// Adds `parent` as a parent of `node`.  Doesn't check wherever `node`
	/// already has another parent.
	pub fn add_parent(&mut self, node: usize, parent: usize) {
		let node = NodeIndex::new(node);
		let parent = NodeIndex::new(parent);
		self.graph.add_edge(parent, node, ());
	}

	/// Replaces the old parent of `node` with the new one.  If `node` had
	/// several parents, only one is replaced.
	pub fn set_parent(&mut self, node: usize, parent: usize) {
		self.remove_parent(node);
		self.add_parent(node, parent);
	}

	/// Swaps parents of `a` and `b`.  If either has several parents, only
	/// one of them is chosen.  If either has no parents, the function is a
	/// no-op.
	pub fn swap_parents(&mut self, a: usize, b: usize) {
		if let (Some(a_parent), Some(b_parent)) =
			(self.parent(a), self.parent(b))
		{
			self.set_parent(a, b_parent);
			self.set_parent(b, a_parent);
		}
	}
}
