use nalgebra::{Dyn, Matrix4, OMatrix, U4};

use base::{seq::DnaSeq, DnaNucleoBase};

pub struct Coalescent {
	columns: Vec<DnaSeq>,
	children: Vec<(usize, usize)>,
	substitutions: Vec<(Substitution, Substitution)>,
}

impl Coalescent {
	pub fn likelihood(&self) -> f64 {
		let mut out = 1.0;

		for column in &self.columns {
			out *= prune_likelihood(
				column,
				&self.children,
				&self.substitutions,
			);
		}

		out
	}
}

type Table = OMatrix<f64, Dyn, U4>;
type Substitution = Matrix4<f64>;

fn prune_likelihood(
	bases: &DnaSeq,
	children: &[(usize, usize)],
	substitutions: &[(Substitution, Substitution)],
) -> f64 {
	let mut t = Table::repeat(bases.len() + children.len(), 0.0);

	for (i, base) in bases.iter().enumerate() {
		let j = match base {
			DnaNucleoBase::Adenine => 0,
			DnaNucleoBase::Cytosine => 1,
			DnaNucleoBase::Guanine => 2,
			DnaNucleoBase::Thymine => 3,
			_ => unreachable!(),
		};

		t[(i, j)] = 1.0;
	}

	for i in 0..children.len() {
		let left = t.row(children[i].0) * substitutions[i].0;
		let right = t.row(children[i].1) * substitutions[i].1;

		let parent = left.component_mul(&right);
		t.set_row(i + bases.len(), &parent);
	}

	t.row(t.shape().0 - 1).sum().ln()
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn minimal() {
		use base::substitution::dna::jukes_cantor;

		let bases = DnaSeq::try_from("AAGCT").unwrap();

		let tree = vec![(2, 3), (0, 1), (5, 4), (6, 7)];
	}
}
