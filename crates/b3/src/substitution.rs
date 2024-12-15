use base::substitution::{self, Substitution};
use linalg::RowMatrix;
use shchurvec::ShchurVec;

use crate::state::StateRef;

pub trait Model {
	type Substitution;

	fn get_matrix(&self, state: &StateRef);
}

pub enum ModelT {
	JukesCantor,
	K80,
	F81,
	Hky,
	Gtr,
}

pub struct Substitutions<const N: usize> {
	current: Substitution<N>,

	p: RowMatrix<f64, N, N>,
	diag: RowMatrix<f64, N, N>,
	inv_p: RowMatrix<f64, N, N>,

	transitions: ShchurVec<RowMatrix<f64, N, N>>,
}

impl<const N: usize> Substitutions<N> {
	pub fn new(length: usize) -> Self {
		let transitions =
			ShchurVec::repeat(RowMatrix::default(), length);

		Self {
			current: Default::default(),

			p: Default::default(),
			diag: Default::default(),
			inv_p: Default::default(),

			transitions,
		}
	}

	/// Returns `true` if a full update is needed.
	pub fn update(&mut self, state: &StateRef) -> bool {
		let tree = state.get_tree();

		let full = todo!(); // self.update_sub(state);

		let edges: Vec<usize> = if full {
			(0..(tree.num_internals() * 2)).collect()
		} else {
			tree.edges_to_update()
		};
		let distances: Vec<f64> = edges
			.iter()
			.copied()
			.map(|e| tree.edge_distance(e))
			.collect();

		self.update_edges(&edges, &distances);

		full
	}

	/// Returns `true` if the substitution matrix has changed.
	fn update_sub(&mut self, sub: Substitution<N>) -> bool {
		todo!()
	}

	fn update_edges(&mut self, edges: &[usize], distances: &[f64]) {
		for (edge, distance) in edges.iter().zip(distances) {
			let transition = self.p
				* self.diag.map_diagonal(|v| v.exp())
				* self.inv_p;

			self.transitions.set(*edge, transition);
		}
	}

	pub fn accept(&mut self) {
		self.transitions.accept();
	}

	pub fn reject(&mut self) {
		self.transitions.reject();
	}

	pub fn substitutions(&self) -> Vec<RowMatrix<f64, N, N>> {
		self.transitions.iter().copied().collect()
	}
}
