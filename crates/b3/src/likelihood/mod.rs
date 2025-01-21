mod cpu;
mod gpu;
// mod thread;

pub use cpu::CpuLikelihood;
pub use gpu::GpuLikelihood;
// #[allow(unused)] // TODO: use dynamically in `State`
// pub use thread::ThreadedLikelihood;

pub type Row<const N: usize> = linalg::Vector<f64, N>;

pub trait Likelihood {
	type Row: Default;
	type Substitution;

	fn propose(
		&mut self,
		nodes: &[usize],
		substitutions: &[Self::Substitution],
		children: &[usize],
	) -> f64;

	fn accept(&mut self);

	fn reject(&mut self);
}
