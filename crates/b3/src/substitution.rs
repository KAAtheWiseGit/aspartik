#![allow(unused)]

use std::marker::PhantomData;

use base::{
	seq::Character,
	substitution::{self, Substitution},
};
use linalg::RowMatrix;
use shchurvec::ShchurVec;

use crate::state::StateRef;

pub enum Model {
	JukesCantor,
	K80,
	F81,
	Hky,
	Gtr,
}

pub struct Substitutions<const N: usize> {
	model: Model,
	parameters: Vec<String>,

	current: Substitution<N>,

	p: RowMatrix<f64, N, N>,
	diag: RowMatrix<f64, N, N>,
	inv_p: RowMatrix<f64, N, N>,

	transitions: ShchurVec<RowMatrix<f64, N, N>>,
}

impl<const N: usize> Substitutions<N> {
	pub fn new(model: Model, length: usize) -> Self {
		let transitions =
			ShchurVec::repeat(RowMatrix::default(), length);

		Self {
			model,
			parameters: vec![],

			current: Default::default(),

			p: Default::default(),
			diag: Default::default(),
			inv_p: Default::default(),

			transitions,
		}
	}

	pub fn update_model(&mut self, state: &StateRef) {
		// model update pulling from state

		// compare new model with the current one

		// if it's different, update all transitions
	}

	pub fn update_edges(&mut self, edges: &[usize], distances: &[f64]) {
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
