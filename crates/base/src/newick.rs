#[derive(Clone, Debug, Default)]
pub struct Node {
	name: Option<String>,
	distance: Option<f64>,
	parent: Option<usize>,
	// There are several different encoding schemes for attributes, which
	// depend on the processing software.  Therefore, they are stored
	// verbatim to be processed on per-method basis.
	attributes: String,
}

impl Node {
	pub fn new(
		name: Option<String>,
		distance: Option<f64>,
		parent: Option<usize>,
		attributes: String,
	) -> Self {
		Self {
			name,
			distance,
			parent,
			attributes,
		}
	}
}

#[derive(Clone, Debug, Default)]
pub struct Tree {
	nodes: Vec<Node>,
}

impl Tree {
	pub fn empty() -> Self {
		Self { nodes: vec![] }
	}

	pub fn push(&mut self, node: Node) -> usize {
		self.nodes.push(node);

		self.nodes.len() - 1
	}

	pub fn get(&self, idx: usize) -> &Node {
		&self.nodes[idx]
	}

	pub fn get_mut(&mut self, idx: usize) -> &mut Node {
		&mut self.nodes[idx]
	}

	pub fn set(&mut self, node: Node, idx: usize) {
		self.nodes[idx] = node;
	}
}
