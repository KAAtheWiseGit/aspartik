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

	pub fn push(
		&mut self,
		node: Node
	) -> usize {
		self.nodes.push(node);

		self.nodes.len() - 1
	}

	pub fn get_parent(&self, i: usize) -> Option<usize> {
		self.nodes[i].parent
	}

	pub fn get_distance(&self, i: usize) -> Option<f64> {
		self.nodes[i].distance
	}

	pub fn get_name(&self, i: usize) -> Option<&str> {
		self.nodes[i].name.as_deref()
	}

	pub fn get_attributes(&self, i: usize) -> &str {
		&self.nodes[i].attributes
	}

	pub fn set_parent(&mut self, i: usize, parent: Option<usize>) {
		self.nodes[i].parent = parent;
	}

	pub fn set_distance(&mut self, i: usize, distance: Option<f64>) {
		self.nodes[i].distance = distance;
	}

	pub fn set_name(&mut self, i: usize, name: Option<String>) {
		self.nodes[i].name = name;
	}

	pub fn set_attributes(&mut self, i: usize, attributes: String) {
		self.nodes[i].attributes = attributes;
	}
}
