use nalgebra::Matrix4;
use rand::{
	distributions::{Distribution, Uniform},
	Rng,
};
use serde_json::{json, Value as Json};
use wide::f64x4;

use std::collections::{HashSet, VecDeque};

use crate::operator::TreeEdit;
use base::{seq::DnaSeq, DnaNucleoBase};

type Row = f64x4;
type Substitution = [f64x4; 4];

const ROOT: usize = usize::MAX;

pub struct Tree {
	/// Leaf node DNA sequences.
	columns: Vec<DnaSeq>,
	/// Tuples of left and right children indices of the internal nodes.
	children: Vec<(usize, usize)>,
	parents: Vec<usize>,
	probabilities: Vec<Vec<Row>>,
	weights: Vec<f64>,

	/// Substitution matrices for edges, specified in `children`.
	substitutions: Vec<(Substitution, Substitution)>,
	model: Matrix4<f64>,
}

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

pub struct Internal(usize);

pub struct Leaf(usize);

impl Tree {
	pub fn new<S>(
		sequences: S,
		substitution_model: Matrix4<f64>,
		weights: &[f64],
		edges: &[(usize, usize)],
	) -> Self
	where
		S: IntoIterator<Item = DnaSeq>,
	{
		let sequences: Vec<DnaSeq> = sequences.into_iter().collect();
		let mut columns = Vec::new();
		for i in 0..sequences[0].len() {
			let mut column = DnaSeq::new();
			for sequence in &sequences {
				column.push(sequence[i]);
			}
			columns.push(column);
		}

		let num_leaves = columns[0].len();
		let num_internals = edges.len();
		let num_nodes = weights.len();

		let mut parents = vec![ROOT; num_nodes];
		for (i, (left, right)) in edges.iter().enumerate() {
			parents[*left] = i + num_leaves;
			parents[*right] = i + num_leaves;
		}

		let zero_row = f64x4::new([0.0, 0.0, 0.0, 0.0]);

		let substitutions =
			vec![([zero_row; 4], [zero_row; 4]); num_internals];

		let probabilities = vec![vec![zero_row; num_nodes]; num_leaves];

		let mut out = Self {
			columns,
			children: edges.into(),
			parents,
			probabilities,
			weights: weights.into(),
			substitutions,
			model: substitution_model,
		};

		out.update_substitutions(out.num_leaves()..out.num_nodes());
		out.update_leaf_probabilites();
		out.update_internal_probabilities(
			out.num_leaves()..out.num_nodes(),
		);

		out
	}

	pub fn num_leaves(&self) -> usize {
		self.columns[0].len()
	}

	pub fn num_internals(&self) -> usize {
		self.children.len()
	}

	pub fn num_nodes(&self) -> usize {
		self.num_leaves() + self.num_internals()
	}

	pub fn is_leaf<N: Into<Node>>(&self, node: N) -> bool {
		node.into().0 < self.num_leaves()
	}

	pub fn is_internal<N: Into<Node>>(&self, node: N) -> bool {
		node.into().0 >= self.num_leaves()
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
		let (left, right) = self.children[node.0 - self.num_leaves()];

		(Node(left), Node(right))
	}

	/// Returns the parent of `node`, or `None` if the node is the root of
	/// the tree.
	pub fn parent_of<N: Into<Node>>(&self, node: N) -> Option<Internal> {
		Some(self.parents[node.into().0])
			.take_if(|p| *p == ROOT)
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

	pub fn likelihood(&self) -> f64 {
		self.probabilities
			.iter()
			.map(|p| p.last().unwrap().reduce_add().ln())
			.sum()
	}

	fn update_substitutions<I>(&mut self, nodes: I)
	where
		I: IntoIterator<Item = usize>,
	{
		let num_leaves = self.num_leaves();

		for i in nodes.into_iter().map(|i| i - num_leaves) {
			let (left, right) = self.children[i];

			let left_distance =
				self.weights[left] - self.weights[i];
			let right_distance =
				self.weights[right] - self.weights[i];

			let left = (self.model * left_distance).exp();
			let right = (self.model * right_distance).exp();

			let left = to_sub(left);
			let right = to_sub(right);

			self.substitutions[i] = (left, right);
		}
	}

	fn update_leaf_probabilites(&mut self) {
		for (column, probability) in
			self.columns.iter().zip(&mut self.probabilities)
		{
			for (i, base) in column.iter().enumerate() {
				probability[i] = to_row(base);
			}
		}
	}

	fn update_internal_probabilities<I>(&mut self, nodes: I)
	where
		I: IntoIterator<Item = usize> + Clone,
	{
		let num_leaves = self.num_leaves();

		for probability in &mut self.probabilities {
			// This should be fast for ranges and a cheap copy for
			// slice iterators.
			for i in nodes
				.clone()
				.into_iter()
				.map(|i| i - num_leaves)
			{
				let left = multiply(
					probability[self.children[i].0],
					self.substitutions[i].0,
				);
				let right = multiply(
					probability[self.children[i].1],
					self.substitutions[i].1,
				);
				probability[i + num_leaves] = left * right;
			}
		}
	}

	pub fn update_with(&mut self, edit: TreeEdit) -> TreeEdit {
		let mut old_weights = Vec::with_capacity(edit.weights.len());
		for (node, weight) in edit.weights.iter().copied() {
			old_weights.push((node, self.weights[node]));
			self.weights[node] = weight;
		}

		/// Swap the children of parent of `x`
		macro_rules! swap {
			($parent_x:ident, $x:ident, $y:ident) => {
				let num_leaves = self.num_leaves();
				if $parent_x != ROOT {
					if self.children[$parent_x - num_leaves]
						.0 == $y
					{
						self.children[$parent_x
							- num_leaves]
							.0 = $y;
					} else {
						self.children[$parent_x
							- num_leaves]
							.1 = $y;
					}
				}
			};
		}

		for (a, b) in edit.parents.iter().copied() {
			let parent_a = self.parents[a];
			let parent_b = self.parents[b];

			swap!(parent_a, a, b);
			swap!(parent_b, b, a);

			self.parents[a] = parent_b;
			self.parents[b] = parent_a;
		}

		self.update_affected(edit.weights.iter().map(|(n, _)| n));
		self.update_affected(edit.parents.iter().map(|(n, _)| n));

		TreeEdit {
			weights: old_weights,
			parents: edit.parents,
		}
	}

	fn update_affected<'a, I>(&mut self, nodes: I)
	where
		I: IntoIterator<Item = &'a usize> + 'a,
	{
		let mut deq = VecDeque::<usize>::new();
		let mut set = HashSet::<usize>::new();

		for node in nodes {
			let mut curr = *node;
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

		let slice = deq.make_contiguous();
		self.update_substitutions(slice.iter().copied());
		self.update_internal_probabilities(slice.iter().copied());
	}

	pub fn serialize(&self) -> Json {
		json!({
			"children": self.children,
			"weights": self.weights,
		})
	}
}

fn to_row(base: &DnaNucleoBase) -> Row {
	match base {
		DnaNucleoBase::Adenine => [1.0, 0.0, 0.0, 0.0],
		DnaNucleoBase::Cytosine => [0.0, 0.0, 1.0, 0.0],
		DnaNucleoBase::Guanine => [0.0, 0.0, 1.0, 0.0],
		DnaNucleoBase::Thymine => [0.0, 0.0, 0.0, 1.0],
		// TODO: other types
		_ => [0.25, 0.25, 0.25, 0.25],
	}
	.into()
}

#[inline(always)]
fn multiply(row: Row, sub: Substitution) -> f64x4 {
	let a = sub[0] * row.as_array_ref()[0];
	let c = sub[1] * row.as_array_ref()[1];
	let g = sub[2] * row.as_array_ref()[2];
	let t = sub[3] * row.as_array_ref()[3];

	/// The reason we need this function is because the `wide` `reduce_add`
	/// method is marked `inline` and, unlike the `glam` methods, it seems
	/// that rustc doesn't inline it.  So, this naive implementation
	/// compiles to a faster binary.
	#[inline(always)]
	fn reduce(vec: f64x4) -> f64 {
		let vec = vec.as_array_ref();

		vec[0] + vec[1] + vec[2] + vec[3]
	}

	f64x4::new([reduce(a), reduce(c), reduce(g), reduce(t)])
}

fn to_sub(sub: Matrix4<f64>) -> Substitution {
	let [a, c, g, t]: &[[f64; 4]; 4] = sub.as_ref();
	[
		f64x4::new(*a),
		f64x4::new(*c),
		f64x4::new(*g),
		f64x4::new(*t),
	]
}
