use linalg::{RowMatrix, Vector};

use super::Model;
use crate::DnaNucleoBase;

type Row = Vector<f64, 4>;
type Substitution = RowMatrix<f64, 4, 4>;

pub struct Dna4Substitution {
	inner: Matrix,
}

// It's fine, the other variant will be big too
#[allow(clippy::large_enum_variant)]
enum Matrix {
	Diagonalizable(Diagonalizable),
	Defective(Defective),
}

struct Diagonalizable {
	p: Substitution,
	p_rev: Substitution,
	d: Substitution,
}

struct Defective {
	// TODO
}

impl Dna4Substitution {
	pub fn jukes_cantor() -> Self {
		let p = Substitution::from([
			[-1.0, -1.0, -1.0, 1.0],
			[0.0, 0.0, 0.0, 1.0],
			[0.0, 0.0, 0.0, 1.0],
			[0.0, 0.0, 0.0, 1.0],
		]);
		let p_rev = Substitution::from([
			[-1.0 / 4.0, -1.0 / 4.0, -1.0 / 4.0, 3.0 / 4.0],
			[-1.0 / 4.0, -1.0 / 4.0, 3.0 / 4.0, -1.0 / 4.0],
			[-1.0 / 4.0, 3.0 / 4.0, -1.0 / 4.0, -1.0 / 4.0],
			[1.0 / 4.0, 1.0 / 4.0, 1.0 / 4.0, 1.0 / 4.0],
		]);
		let d = Substitution::from([
			[-4.0 / 3.0, 0.0, 0.0, 0.0],
			[0.0, -4.0 / 3.0, 0.0, 0.0],
			[0.0, 0.0, -4.0 / 3.0, 0.0],
			[0.0, 0.0, 0.0, 0.0],
		]);

		Self {
			inner: Matrix::Diagonalizable(Diagonalizable {
				p,
				p_rev,
				d,
			}),
		}
	}
}

impl Dna4Substitution {
	fn as_diagonalizable(&self) -> Option<&Diagonalizable> {
		match &self.inner {
			Matrix::Diagonalizable(d) => Some(d),
			_ => None,
		}
	}
}

impl Model for Dna4Substitution {
	type Item = DnaNucleoBase;
	type Row = Row;
	type Substitution = Substitution;

	fn to_row(item: &DnaNucleoBase) -> Row {
		match item {
			DnaNucleoBase::Adenine => [1.0, 0.0, 0.0, 0.0],
			DnaNucleoBase::Cytosine => [0.0, 0.0, 1.0, 0.0],
			DnaNucleoBase::Guanine => [0.0, 0.0, 1.0, 0.0],
			DnaNucleoBase::Thymine => [0.0, 0.0, 0.0, 1.0],
			// TODO: other types
			_ => [0.25, 0.25, 0.25, 0.25],
		}
		.into()
	}

	fn probability(row: &Row) -> f64 {
		row.sum().ln()
	}

	fn substitution(&self, distance: f64) -> Substitution {
		if let Some(diag) = self.as_diagonalizable() {
			let mut e_d = diag.d;
			for i in 0..4 {
				e_d[(i, i)] = f64::exp(distance * e_d[(i, i)]);
			}

			diag.p * e_d * diag.p_rev
		} else {
			todo!()
		}
	}
}
