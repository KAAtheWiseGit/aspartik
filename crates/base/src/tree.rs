use std::collections::HashMap;

struct Node {
	parent: Option<usize>,
	distance: Option<f64>,
	name: Option<String>,
	// There isn't a particular rhyme or reason to attributes, they are
	// interpreted on per-software basis.  So, the hash map stores the
	// source string representation to be interpreted by individual methods.
	attributes: HashMap<String, String>,
}

pub struct NewickTree {
	nodes: Vec<Node>,
}
