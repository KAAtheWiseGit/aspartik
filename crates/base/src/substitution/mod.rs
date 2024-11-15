use std::ops::Mul;

pub mod dna;

pub trait Model {
	type Row;
	type Substitution: Mul<Self::Row, Output = Self::Row>;

	fn substitution(&self, distance: f64) -> Self::Substitution;
}
