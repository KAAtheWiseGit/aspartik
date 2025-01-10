use super::Probability;
use crate::State;

pub struct Uniform {
	parameter: String,
}

impl Uniform {
	pub fn new(parameter: String) -> Self {
		Self { parameter }
	}
}

impl Probability for Uniform {
	// can be cached, but I doubt that's needed
	fn probability(&self, state: &State) -> f64 {
		let Ok(Some(param)) = state
			.get_parameter(&self.parameter)
			.map(|p| p.as_real())
		else {
			return 0.0;
		};

		let (Some(min), Some(max)) = (param.min, param.max) else {
			return 1.0;
		};

		let prob = 1.0 / (max - min);
		prob.ln()
	}
}
