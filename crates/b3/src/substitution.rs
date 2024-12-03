use base::{seq::Character, substitution::Model};
use linalg::RowMatrix;
use shchurvec::ShchurVec;

pub struct Substitutions<C: Character, const N: usize> {
	model: Model<C, N>,
	substitutions: ShchurVec<RowMatrix<f64, N, N>>,
}

impl<C: Character, const N: usize> Substitutions<C, N> {
	pub fn new(model: Model<C, N>, length: usize) -> Self {
		let substitutions =
			ShchurVec::repeat(RowMatrix::default(), length);

		Self {
			model,
			substitutions,
		}
	}

	pub fn propose(&mut self, edges: &[usize], distances: &[f64]) {
		for (edge, distance) in edges.iter().zip(distances) {
			self.substitutions
				.set(*edge, self.model.substitution(*distance));
		}
	}

	pub fn accept(&mut self) {
		self.substitutions.accept();
	}

	pub fn reject(&mut self) {
		self.substitutions.reject();
	}

	pub fn substitutions(&self) -> Vec<RowMatrix<f64, N, N>> {
		self.substitutions.iter().copied().collect()
	}

	pub fn model(&self) -> &Model<C, N> {
		&self.model
	}
}
