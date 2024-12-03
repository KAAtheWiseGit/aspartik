mod cpu;
// TODO: mod thread;

pub use cpu::CpuLikelihood;
// pub use thread::ThreadedLikelihood;

pub trait Likelihood {
	type Row: Default;
	type Substitution;

	fn propose(
		&mut self,
		substitutions: &[Self::Substitution],
		nodes: &[usize],
		children: &[(usize, usize)],
	);

	fn likelihood(&self) -> f64;

	fn accept(&mut self);

	fn reject(&mut self);
}
