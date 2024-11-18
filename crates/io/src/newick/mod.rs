#![allow(dead_code)]

use petgraph::stable_graph::{NodeIndex, StableDiGraph};

mod parse;

#[derive(Debug, Clone)]
pub struct Node {
	name: String,
	attributes: String,
	length: f64,
}

impl Node {
	pub fn name(&self) -> &str {
		&self.name
	}

	pub fn length(&self) -> f64 {
		self.length
	}
}

#[derive(Debug, Clone, Default)]
pub struct Tree {
	graph: StableDiGraph<Node, ()>,
	root: Option<NodeIndex>,
}

impl Tree {
	pub fn root(&self) -> Option<&NodeIndex> {
		self.root.as_ref()
	}

	pub fn children_of(&self, node: NodeIndex) -> Vec<NodeIndex> {
		self.graph.neighbors(node).collect()
	}

	pub fn get_node(&self, idx: NodeIndex) -> &Node {
		&self.graph[idx]
	}
}
