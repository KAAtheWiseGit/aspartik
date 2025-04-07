use linalg::{RowMatrix, Vector};

mod cpu;
mod gpu;
// mod thread;

// #[allow(unused)] // TODO: use dynamically in `State`
// pub use thread::ThreadedLikelihood;

type Row<const N: usize> = Vector<f64, N>;
type Transition<const N: usize> = RowMatrix<f64, N, N>;

pub trait Likelihood<const N: usize> {
	fn propose(
		&mut self,
		nodes: &[usize],
		transitions: &[Transition<N>],
		children: &[usize],
	) -> f64;

	fn accept(&mut self);

	fn reject(&mut self);
}
