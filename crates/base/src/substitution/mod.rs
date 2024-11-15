use std::ops::Mul;

pub mod dna;

pub trait Model {
	type Item;
	type Row: Copy + Mul<Output = Self::Row> + Default;
	type Substitution: Copy + Mul<Self::Row, Output = Self::Row> + Default;

	fn to_row(item: &Self::Item) -> Self::Row;

	fn substitution(&self, distance: f64) -> Self::Substitution;
}
