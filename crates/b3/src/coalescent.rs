use nalgebra::{Dyn, Matrix4, OMatrix, RowVector4, U4};

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
				const ADENINE: RowVector4<f64> =
					RowVector4::new(1.0, 0.0, 0.0, 0.0);
				const CYTOSINE: RowVector4<f64> =
					RowVector4::new(0.0, 1.0, 0.0, 0.0);
				const GUANINE: RowVector4<f64> =
					RowVector4::new(0.0, 0.0, 1.0, 0.0);
				const THYMINE: RowVector4<f64> =
					RowVector4::new(0.0, 0.0, 0.0, 1.0);
				const ANY: RowVector4<f64> =
					RowVector4::new(0.25, 0.25, 0.25, 0.25);

				match base {
					DnaNucleoBase::Adenine => {
						t.set_row(i, &ADENINE)
					}
					DnaNucleoBase::Cytosine => {
						t.set_row(i, &CYTOSINE)
					}
					DnaNucleoBase::Guanine => {
						t.set_row(i, &GUANINE)
					}
					DnaNucleoBase::Thymine => {
						t.set_row(i, &THYMINE)
					}
					_ => t.set_row(i, &ANY),
				}
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
		}

		out
	}
}
