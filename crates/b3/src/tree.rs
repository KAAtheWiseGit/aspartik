use nalgebra::Matrix4;
use wide::f64x4;

use base::{seq::DnaSeq, DnaNucleoBase};

type Row = f64x4;
type Substitution = [f64x4; 4];

pub struct Tree {
	/// Leaf node DNA sequences.
	columns: Vec<DnaSeq>,
	/// Tuples of left and right children indices of the internal nodes.
	children: Vec<(usize, usize)>,
	probabilities: Vec<Vec<Row>>,
	weights: Vec<f64>,

	/// Substitution matrices for edges, specified in `children`.
	substitutions: Vec<(Substitution, Substitution)>,
	model: Matrix4<f64>,
}

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

		let substitutions = vec![
			(
				[f64x4::new([0.0, 0.0, 0.0, 0.0]); 4],
				[f64x4::new([0.0, 0.0, 0.0, 0.0]); 4],
			);
			edges.len()
		];

		let probabilities = vec![
			vec![
				f64x4::new([0.0, 0.0, 0.0, 0.0]);
				columns.len() + edges.len()
			];
			columns[0].len()
		];

		let mut out = Self {
			columns,
			children: edges.into(),
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

	pub fn likelihood(&self) -> f64 {
		self.probabilities
			.iter()
			.map(|p| p.last().unwrap().reduce_add().ln())
			.sum()
	}

	pub fn update_substitutions<I>(&mut self, nodes: I)
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

	pub fn update_leaf_probabilites(&mut self) {
		for (column, probability) in
			self.columns.iter().zip(&mut self.probabilities)
		{
			for (i, base) in column.iter().enumerate() {
				probability[i] = to_row(base);
			}
		}
	}

	pub fn update_internal_probabilities<I>(&mut self, nodes: I)
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
