#![allow(dead_code)]

use anyhow::{anyhow, Result};
use rand::Rng as _;
use rand_distr::{
	Beta, Cauchy, ChiSquared, Distribution as _, Exp, Gamma, LogNormal,
	Normal, Poisson, StandardNormal, StudentT, Triangular, Uniform,
};

use crate::State;

pub enum Distribution {
	Uniform,

	Triangular,

	Beta {
		alpha: String,
		beta: String,
	},

	Normal {
		mean: String,
		std_dev: String,
	},

	Cauchy {
		location: String,
		scale: String,
	},

	Exponential {
		rate: String,
	},

	Gamma {
		shape: String,
		scale: String,
	},

	InverseGamma {
		shape: String,
		scale: String,
	},

	Chi {
		df: String,
	},

	ChiSquared {
		df: String,
	},

	Poisson {
		rate: String,
	},

	StudentT {
		df: String,
	},

	Laplace {
		location: String,
		scale: String,
	},

	LogNormal {
		mean: String,
		std_dev: String,
	},

	/// <https://pmc.ncbi.nlm.nih.gov/articles/PMC3845170/>
	Bactrian {
		m: String,
		std_dev: String,
	},
}

fn get_real(state: &mut State, name: &str) -> Result<f64> {
	state.param(name)?.one_real().ok_or_else(|| {
		anyhow!(
			"Expected the parameter {} to be a single real value",
			name
		)
	})
}

fn get_integer(state: &mut State, name: &str) -> Result<i64> {
	state.param(name)?.one_integer().ok_or_else(|| {
		anyhow!(
			"Expected the parameter {} to be a single integer value",
			name
		)
	})
}

impl Distribution {
	/// Returns a number sampled from the whole real numbers line or `None`
	/// for distributions which don't support that.
	pub fn random_line(&self, state: &mut State) -> Result<Option<f64>> {
		match self {
			Distribution::Normal { mean, std_dev } => {
				let mean = get_real(state, mean)?;
				let std_dev = get_real(state, std_dev)?;

				let dist = Normal::new(mean, std_dev).unwrap();
				Ok(Some(dist.sample(&mut state.rng)))
			}
			Distribution::StudentT { df } => {
				let df = get_real(state, df)?;

				let dist = StudentT::new(df).unwrap();
				Ok(Some(dist.sample(&mut state.rng)))
			}
			Distribution::Laplace { location, scale } => {
				let location = get_real(state, location)?;
				let scale = get_real(state, scale)?;

				// <https://en.wikipedia.org/wiki/Laplace_distribution#Random_variate_generation>
				let u: f64 = state.rng.random_range(-0.5..0.5);
				let x = location
					- scale * u.signum()
						* (1.0 - 2.0 * u.abs()).ln();

				Ok(Some(x))
			}
			Distribution::Cauchy { location, scale } => {
				let location = get_real(state, location)?;
				let scale = get_real(state, scale)?;

				let dist = Cauchy::new(location, scale)?;

				Ok(Some(dist.sample(&mut state.rng)))
			}
			Distribution::Bactrian { m, std_dev } => {
				let m = get_real(state, m)?;
				let std_dev = get_real(state, std_dev)?;

				let dist = Normal::new(0.0, std_dev).unwrap();
				let mut point = dist.sample(&mut state.rng);
				point *= (1.0 - m * m).sqrt();
				if state.rng.random::<bool>() {
					point += m;
				} else {
					point -= m;
				}

				Ok(Some(point))
			}
			_ => Ok(None),
		}
	}

	/// Returns a number sampled on a `(0, infinity)` semi-infinite
	/// interval or `None` for distributions which don't support that.
	///
	/// Full-line distributions are transformed using exponentiation.
	pub fn random_semi_interval(
		&self,
		state: &mut State,
	) -> Result<Option<f64>> {
		if let Some(point) = self.random_line(state)? {
			return Ok(Some(point));
		}

		match self {
			Distribution::Exponential { rate } => {
				let rate = get_real(state, rate)?;

				let dist = Exp::new(rate).unwrap();
				Ok(Some(dist.sample(&mut state.rng)))
			}

			Distribution::Chi { df } => {
				let df = get_integer(state, df)?;

				Ok(Some((0..df)
					.map(|_| {
						let x: f64 = StandardNormal
							.sample(&mut state.rng);
						x.powi(2)
					})
					.sum::<f64>()
					.sqrt()))
			}

			Distribution::ChiSquared { df } => {
				let df = get_integer(state, df)?;

				let dist = ChiSquared::new(df as f64).unwrap();
				Ok(Some(dist.sample(&mut state.rng)))
			}

			Distribution::Gamma { shape, scale } => {
				let shape = get_real(state, shape)?;
				let scale = get_real(state, scale)?;

				let dist = Gamma::new(shape, scale).unwrap();
				Ok(Some(dist.sample(&mut state.rng)))
			}

			Distribution::InverseGamma { shape, scale } => {
				let shape = get_real(state, shape)?;
				let scale = get_real(state, scale)?;

				let dist = Gamma::new(shape, scale).unwrap();
				let x = dist.sample(&mut state.rng);
				Ok(Some(1.0 / x))
			}

			Distribution::Poisson { rate } => {
				let rate = get_real(state, rate)?;

				let dist = Poisson::new(rate)?;

				Ok(Some(dist.sample(&mut state.rng)))
			}

			Distribution::LogNormal { mean, std_dev } => {
				let mean = get_real(state, mean)?;
				let std_dev = get_real(state, std_dev)?;

				let dist =
					LogNormal::new(mean, std_dev).unwrap();
				Ok(Some(dist.sample(&mut state.rng)))
			}
			_ => Ok(None),
		}
	}

	/// Return a random value from the `(low, high)` interval.
	pub fn random_range(
		&self,
		low: f64,
		high: f64,
		state: &mut State,
	) -> Result<f64> {
		assert!(low < high);

		if let Some(point) = self.random_line(state)? {
			return Ok(interval_to_range(point.exp(), low, high));
		}
		if let Some(point) = self.random_semi_interval(state)? {
			return Ok(interval_to_range(point, low, high));
		}

		match self {
			Distribution::Uniform => {
				let dist = Uniform::new(low, high).unwrap();

				Ok(dist.sample(&mut state.rng))
			}
			Distribution::Triangular => {
				let center = (low + high) / 2.0;
				let dist = Triangular::new(low, high, center)
					.unwrap();

				Ok(dist.sample(&mut state.rng))
			}
			Distribution::Beta { alpha, beta } => {
				let alpha = get_real(state, alpha)?;
				let beta = get_real(state, beta)?;

				let dist = Beta::new(alpha, beta).unwrap();

				Ok(low + dist.sample(&mut state.rng)
					* (high - low))
			}
			_ => unreachable!(),
		}
	}

	pub fn random_range_with(
		&self,
		low: f64,
		high: f64,
		value: f64,
		state: &mut State,
	) -> Result<f64> {
		assert!(low < high);
		assert!(
			low < value && value < high,
			"The value {value} is not in range {low}-{high}"
		);

		let ratio = (high - value) / (value - low);

		if let Some(point) = self.random_line(state)? {
			let ratio = ratio * point.exp();
			return Ok(interval_to_range(ratio, low, high));
		}
		if let Some(point) = self.random_semi_interval(state)? {
			let ratio = ratio * point;
			return Ok(interval_to_range(ratio, low, high));
		}

		match self {
			Distribution::Uniform => {
				let dist = Uniform::new(low, high).unwrap();

				Ok(dist.sample(&mut state.rng))
			}
			Distribution::Triangular => {
				let center = (low + high) / 2.0;
				let dist = Triangular::new(low, high, center)
					.unwrap();

				Ok(dist.sample(&mut state.rng))
			}
			Distribution::Beta { .. } => {
				// Beta's parameters are set by the state, so we
				// can't align it to the value
				self.random_range(low, high, state)
			}
			_ => unreachable!(),
		}
	}
}

fn interval_to_range(ratio: f64, low: f64, high: f64) -> f64 {
	low + (high - low) / (ratio + 1.0)
}
