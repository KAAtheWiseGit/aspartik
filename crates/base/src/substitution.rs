use std::marker::PhantomData;

use crate::{seq::Character, DnaNucleoBase};
use linalg::{RowMatrix, Vector};

type Row<const N: usize> = Vector<f64, N>;
type Substitution<const N: usize> = RowMatrix<f64, N, N>;

pub struct Model<C: Character, const N: usize> {
	inner: Matrix<N>,
	character: PhantomData<C>,
}

// It's fine, the other variant will be big too
#[allow(clippy::large_enum_variant)]
enum Matrix<const N: usize> {
	Diagonalizable(Diagonalizable<N>),
	Defective(Defective<N>),
}

struct Diagonalizable<const N: usize> {
	p: Substitution<N>,
	p_rev: Substitution<N>,
	d: Substitution<N>,
}

struct Defective<const N: usize> {
	// TODO
}

impl Model<DnaNucleoBase, 4> {
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
			character: PhantomData,
		}
	}

	pub fn to_row(item: &DnaNucleoBase) -> Row<4> {
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
}

impl<C: Character, const N: usize> Model<C, N> {
	pub fn probability(row: &Row<N>) -> f64 {
		row.sum().ln()
	}

	pub fn substitution(&self, distance: f64) -> Substitution<N> {
		if let Matrix::Diagonalizable(diag) = &self.inner {
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
