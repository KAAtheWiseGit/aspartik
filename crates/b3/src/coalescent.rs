use nalgebra::{Dyn, Matrix4, OMatrix, U4};

use base::{seq::DnaSeq, DnaNucleoBase};

pub struct Coalescent {
	sequences: Vec<DnaSeq>,
	tree: Vec<(usize, usize)>,
}

impl Coalescent {
	pub fn likelihood(&self) -> f64 {
		let mut out = 1.0;

		for i in 0..self.sequences[0].length() {
			let bases: Vec<DnaNucleoBase> =
				self.sequences.iter().map(|s| s[i]).collect();

			out *= prune_likelihood(bases, &self.tree);
		}

		out
	}
}

type Table = OMatrix<f64, Dyn, U4>;

fn prune_likelihood(bases: Vec<DnaNucleoBase>, tree: &[(usize, usize)]) -> f64 {
	let mut jk = Matrix4::from_element(1.0 / 3.0);
	jk[(0, 0)] = -1.0;
	jk[(1, 1)] = -1.0;
	jk[(2, 2)] = -1.0;
	jk[(3, 3)] = -1.0;
	let jk = jk;

	let mut t = Table::repeat(bases.len() + tree.len(), 0.0);

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

	for (i, (left, right)) in tree.iter().enumerate() {
		let len_left = 1.0;
		let len_right = 1.0;

		let prob_left = t.row(*left) * (jk * len_left).exp();
		let prob_right = t.row(*right) * (jk * len_right).exp();

		let new = prob_left.component_mul(&prob_right);
		t.set_row(i + bases.len(), &new);
	}

	t.row(t.shape().0 - 1).product()
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn minimal() {
		let bases = vec![
			DnaNucleoBase::Adenine,
			DnaNucleoBase::Adenine,
			DnaNucleoBase::Guanine,
			DnaNucleoBase::Cytosine,
			DnaNucleoBase::Thymine,
		];

		let tree = vec![(2, 3), (0, 1), (5, 4), (6, 7)];

		prune_likelihood(bases.clone(), &tree);
	}
}
