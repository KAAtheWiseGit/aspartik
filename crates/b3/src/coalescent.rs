use glam::{DMat4, DVec4};
use nalgebra::Matrix4;

use base::{seq::DnaSeq, DnaNucleoBase};

type Substitution = Matrix4<f64>;

pub struct Coalescent {
	columns: Vec<DnaSeq>,
	children: Vec<(usize, usize)>,
	substitutions: Vec<(DMat4, DMat4)>,
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

			substitutions.push((left.into(), right.into()));
		}

		Self {
			columns,
			children: edges.into(),
			substitutions,
		}
	}

	pub fn likelihood(&self) -> f64 {
		let mut out = 0.0;

		let any = DVec4::new(0.25, 0.25, 0.25, 0.25);

		let mut t = vec![
			DVec4::ZERO;
			self.columns.len() + self.children.len()
		];

		for column in &self.columns {
			for (i, base) in column.iter().enumerate() {
				match base {
					DnaNucleoBase::Adenine => {
						t[i] = DVec4::X;
					}
					DnaNucleoBase::Cytosine => {
						t[i] = DVec4::Y;
					}
					DnaNucleoBase::Guanine => {
						t[i] = DVec4::Z;
					}
					DnaNucleoBase::Thymine => {
						t[i] = DVec4::W;
					}
					_ => t[i] = any,
				}
			}

			for i in 0..self.children.len() {
				let left = self.substitutions[i].0
					* t[self.children[i].0];
				let right = self.substitutions[i].1
					* t[self.children[i].1];

				let parent = left * right;
				t[i + column.len()] = parent;
			}

			out += t.last().unwrap().element_sum().ln();
		}

		out
	}
}

#[cfg(test)]
mod test {
	#[test]
	fn glam() {
		use glam::{DMat4, DVec4};

		let v = DVec4::new(0.5, 0.1, 0.2, 0.4);
		let m = DMat4::from_cols_array_2d(&[
			[1.0, 2.0, 3.0, 4.0],
			[0.3, 2.0, 3.0, 4.0],
			[1.2, 2.0, 3.0, 4.0],
			[1.0, 2.0, 5.0, 4.0],
		]);

		let mul = m * v;
		println!("{mul}");
	}
}
