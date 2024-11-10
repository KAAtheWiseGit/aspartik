use nalgebra::Matrix4;
use wide::f64x4;

use base::{seq::DnaSeq, DnaNucleoBase};

type Row = f64x4;
type Substitution = [f64x4; 4];

pub struct Coalescent {
	columns: Vec<DnaSeq>,
	children: Vec<(usize, usize)>,
	substitutions: Vec<(Substitution, Substitution)>,
}

impl Coalescent {
	pub fn new<S>(
		sequences: S,
		substitution_model: Matrix4<f64>,
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

			let left = to_sub(left);
			let right = to_sub(right);

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

		let mut t = vec![
			f64x4::new([0.0, 0.0, 0.0, 0.0]);
			self.columns.len() + self.children.len()
		];

		for column in &self.columns {
			for (i, base) in column.iter().enumerate() {
				match base {
					DnaNucleoBase::Adenine => {
						t[i] = f64x4::new([
							1.0, 0.0, 0.0, 0.0,
						]);
					}
					DnaNucleoBase::Cytosine => {
						t[i] = f64x4::new([
							0.0, 0.0, 1.0, 0.0,
						]);
					}
					DnaNucleoBase::Guanine => {
						t[i] = f64x4::new([
							0.0, 0.0, 1.0, 0.0,
						]);
					}
					DnaNucleoBase::Thymine => {
						t[i] = f64x4::new([
							0.0, 0.0, 0.0, 1.0,
						]);
					}
					_ => {
						t[i] = f64x4::new([
							0.25, 0.25, 0.25, 0.25,
						])
					}
				}
			}

			for i in 0..self.children.len() {
				let left = multiply(
					t[self.children[i].0],
					self.substitutions[i].0,
				);
				let right = multiply(
					t[self.children[i].1],
					self.substitutions[i].1,
				);

				let parent = left * right;
				t[i + column.len()] = parent;
			}

			out += t.last().unwrap().reduce_add().ln();
		}

		out
	}
}

#[inline(always)]
fn multiply(row: Row, sub: Substitution) -> f64x4 {
	let a = sub[0] * row.as_array_ref()[0];
	let c = sub[1] * row.as_array_ref()[1];
	let g = sub[2] * row.as_array_ref()[2];
	let t = sub[3] * row.as_array_ref()[3];

	f64x4::new([
		a.reduce_add(),
		c.reduce_add(),
		g.reduce_add(),
		t.reduce_add(),
	])
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
