use std::f64::consts::E;

use super::Probability;
use crate::State;

pub struct Poisson {
	parameter: String,
	lambda: String,
}

impl Probability for Poisson {
	fn probability(&self, state: &State) -> f64 {
		let Ok(Some(param)) = state
			.get_parameter(&self.parameter)
			.map(|p| p.as_integer())
		else {
			return f64::NEG_INFINITY;
		};
		let Ok(Some(lambda)) =
			state.get_parameter(&self.lambda).map(|p| p.as_real())
		else {
			return f64::NEG_INFINITY;
		};
		let lambda = lambda.values[0];

		param.values
			.iter()
			.copied()
			.map(|k| {
				if k == 0 {
					return 0.0;
				}

				// λ^k e^(-λ) / k!
				lambda.powi(k as i32) * E.powf(-lambda)
					/ factorial(k)
			})
			.map(|p: f64| p.ln())
			.sum()
	}
}

fn factorial(k: i64) -> f64 {
	assert!(k > 0);
	let index = k as usize;
	const FACTORIAL: [f64; 31] = [
		// zero, invalid
		0.0,
		1.0,
		2.0,
		6.0,
		24.0,
		120.0,
		720.0,
		5040.0,
		40320.0,
		362880.0,
		3628800.0,
		39916800.0,
		479001600.0,
		6227020800.0,
		87178291200.0,
		1307674368000.0,
		20922789888000.0,
		355687428096000.0,
		6402373705728000.0,
		121645100408832000.0,
		2432902008176640000.0,
		51090942171709440000.0,
		1124000727777607680000.0,
		25852016738884976640000.0,
		620448401733239439360000.0,
		15511210043330985984000000.0,
		403291461126605635584000000.0,
		10888869450418352160768000000.0,
		304888344611713860501504000000.0,
		8841761993739701954543616000000.0,
		265252859812191058636308480000000.0,
	];

	FACTORIAL.get(index).copied().unwrap_or(f64::INFINITY)
}
