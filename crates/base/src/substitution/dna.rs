use super::Model;
use linalg::{RowMatrix, Vector};

type Row = Vector<f64, 4>;
type Substitution = RowMatrix<f64, 4, 4>;

pub struct Dna4Substitution {
	p: Substitution,
	p_rev: Substitution,
	d: Substitution,
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

		Self { p, p_rev, d }
	}
}

impl Model for Dna4Substitution {
	type Row = Row;
	type Substitution = Substitution;

	fn substitution(&self, distance: f64) -> Substitution {
		let mut e_d = self.d;
		for i in 0..4 {
			e_d[(i, i)] = f64::exp(distance * e_d[(i, i)]);
		}

		self.p * e_d * self.p_rev
	}
}
