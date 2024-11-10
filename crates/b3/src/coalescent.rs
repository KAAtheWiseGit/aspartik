use nalgebra::{Dyn, Matrix4, OMatrix, U4};

use base::{seq::DnaSeq, DnaNucleoBase};

type Substitution = Matrix4<f64>;

pub struct Coalescent {
	columns: Vec<DnaSeq>,
	children: Vec<(usize, usize)>,
	substitutions: Vec<(Substitution, Substitution)>,
}

impl Coalescent {
	pub fn new<S>(
		sequences: S,
		substitution_model: Substitution,
		nodes: &[f64],
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

		let mut substitutions = Vec::new();
		for (i, (left, right)) in edges
			.iter()
			.enumerate()
			.map(|(i, lr)| (i + sequences.len(), lr))
		{
			let left_distance = nodes[*left] - nodes[i];
			let right_distance = nodes[*right] - nodes[i];

			let left = (substitution_model * left_distance).exp();
			let right = (substitution_model * right_distance).exp();

			substitutions.push((left, right));
		}

		Self {
			columns,
			children: edges.into(),
			substitutions,
		}
	}

	pub fn likelihood(&self) -> f64 {
		let mut out = 0.0;

		type Table = OMatrix<f64, Dyn, U4>;
		let mut t =
			Table::zeros(self.columns.len() + self.children.len());

		for column in &self.columns {
			for (i, base) in column.iter().enumerate() {
				let j = match base {
					DnaNucleoBase::Adenine => 0,
					DnaNucleoBase::Cytosine => 1,
					DnaNucleoBase::Guanine => 2,
					DnaNucleoBase::Thymine => 3,
					_ => unreachable!(),
				};

				t[(i, j)] = 1.0;
			}

			for i in 0..self.children.len() {
				let left = t.row(self.children[i].0)
					* self.substitutions[i].0;
				let right = t.row(self.children[i].1)
					* self.substitutions[i].1;

				let parent = left.component_mul(&right);
				t.set_row(i + column.len(), &parent);
			}

			out += t.row(t.shape().0 - 1).sum().ln();

			t.fill(0.0);
		}

		out
	}
}
