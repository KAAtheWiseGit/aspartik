use linalg::{RowMatrix, Vector};

mod cpu;
mod gpu;
// mod thread;

use cpu::CpuLikelihood;
use gpu::GpuLikelihood;

// #[allow(unused)] // TODO: use dynamically in `State`
// pub use thread::ThreadedLikelihood;

type Row<const N: usize> = Vector<f64, N>;
type Transition<const N: usize> = RowMatrix<f64, N, N>;

trait LikelihoodTrait<const N: usize> {
	fn propose(
		&mut self,
		nodes: &[usize],
		transitions: &[Transition<N>],
		children: &[usize],
	) -> f64;

	fn accept(&mut self);

	fn reject(&mut self);
}

pub struct Likelihood<const N: usize> {
	inner: Box<dyn LikelihoodTrait<N>>,
}

impl Likelihood<4> {
	pub fn gpu(sites: Vec<Vec<Row<4>>>) -> Self {
		Self {
			inner: Box::new(GpuLikelihood::new(sites)),
		}
	}
}

impl<const N: usize> Likelihood<N> {
	pub fn cpu(sites: Vec<Vec<Row<N>>>) -> Self {
		Self {
			inner: Box::new(CpuLikelihood::new(sites)),
		}
	}

	pub fn propose(
		&mut self,
		nodes: &[usize],
		transitions: &[Transition<N>],
		children: &[usize],
	) -> f64 {
		self.inner.propose(nodes, transitions, children)
	}

	pub fn accept(&mut self) {
		self.inner.accept();
	}

	pub fn reject(&mut self) {
		self.inner.reject();
	}
}
