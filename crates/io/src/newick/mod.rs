#![allow(dead_code)]

use petgraph::graph::DiGraph;

mod parse;

struct Node {
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

pub struct Tree {
	graph: DiGraph<Node, ()>,
}
