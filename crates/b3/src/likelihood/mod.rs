use crate::tree::Update;
use base::substitution::Model;

mod cpu;
mod thread;

pub use cpu::CpuLikelihood;
pub use thread::ThreadedLikelihood;

pub trait Likelihood {
	type Model: Model;

	fn new(
		sites: Vec<Vec<<Self::Model as Model>::Item>>,
		model: Self::Model,
	) -> Self;

	fn propose(&mut self, update: Update);

	fn likelihood(&self) -> f64;

	fn accept(&mut self);

	fn reject(&mut self);
}
