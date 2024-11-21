use std::ops::Mul;

pub mod dna;

pub trait Model: Send {
	type Item;
	type Row: Copy + Mul<Output = Self::Row> + Default + Send;
	type Substitution: Copy
		+ Mul<Self::Row, Output = Self::Row>
		+ Default
		+ Send;

	fn to_row(item: &Self::Item) -> Self::Row;

	fn probability(row: &Self::Row) -> f64;

	fn substitution(&self, distance: f64) -> Self::Substitution;
}
