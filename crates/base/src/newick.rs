use std::collections::HashMap;

// There isn't a particular rhyme or reason to attributes, they are interpreted
// on per-software basis.  So, the hash map stores the source string
// representation to be interpreted by individual methods.
pub type Attributes = HashMap<String, String>;

// XXX: maybe get rid of it in favor of struct of vectors
#[derive(Clone, Debug, Default)]
struct Node {
	parent: Option<usize>,
	distance: Option<f64>,
	name: Option<String>,
	attributes: Attributes,
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
		parent: Option<usize>,
		distance: Option<f64>,
		name: Option<String>,
		attributes: Attributes,
	) {
		self.nodes.push(Node {
			parent,
			distance,
			name,
			attributes,
		})
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

	pub fn get_attributes(&self, i: usize) -> &Attributes {
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

	pub fn set_attributes(&mut self, i: usize, attributes: Attributes) {
		self.nodes[i].attributes = attributes;
	}
}
