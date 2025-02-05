#![allow(dead_code)]

use rand_distr::{Distribution as _, Exp, Gamma, Normal, Triangular, Uniform};
use statrs::distribution::Laplace;

use crate::operator::Rng;

pub enum Distribution {
	Uniform,

	Triangular,

	Normal {
		mean: f64,
		std_dev: f64,
	},

	Exponential {
		rate: f64,
	},

	Gamma {
		shape: f64,
		scale: f64,
	},

	Laplace {
		location: f64,
		scale: f64,
	},

	/// <https://pmc.ncbi.nlm.nih.gov/articles/PMC3845170/>
	Bactrian,
}

impl Distribution {
	/// Returns a number sampled from the whole real numbers line or `None`
	/// for distributions which don't support that.
	pub fn random_line(&self, rng: &mut Rng) -> Option<f64> {
		match self {
			Distribution::Normal { mean, std_dev } => {
				let normal =
					Normal::new(*mean, *std_dev).unwrap();
				Some(normal.sample(rng))
			}
			Distribution::Gamma { shape, scale } => {
				let gamma = Gamma::new(*shape, *scale).unwrap();
				Some(gamma.sample(rng))
			}
			Distribution::Laplace { location, scale } => {
				let _laplace = Laplace::new(*location, *scale)
					.unwrap();

				todo!("`statrs` depends on an older `rand` version")
			}
			_ => None,
		}
	}

	/// Returns a number sampled on a `(0, infinity)` semi-infinite
	/// interval or `None` for distributions which don't support that.
	///
	/// Full-line distributions are transformed using exponentiation.
	pub fn random_semi_interval(&self, rng: &mut Rng) -> Option<f64> {
		if let Some(point) = self.random_line(rng) {
			return Some(point);
		}

		match self {
			Distribution::Exponential { rate } => {
				let exp = Exp::new(*rate).unwrap();
				Some(exp.sample(rng))
			}
			_ => None,
		}
	}

	/// Return a random value from the `(low, high)` interval.
	pub fn random_range(&self, low: f64, high: f64, rng: &mut Rng) -> f64 {
		assert!(low < high);

		if let Some(point) = self.random_line(rng) {
			return interval_to_range(point.exp(), low, high);
		}
		if let Some(point) = self.random_semi_interval(rng) {
			return interval_to_range(point, low, high);
		}

		match self {
			Distribution::Uniform => {
				let dist = Uniform::new(low, high).unwrap();

				dist.sample(rng)
			}
			Distribution::Triangular => {
				let center = (low + high) / 2.0;
				let dist = Triangular::new(low, high, center)
					.unwrap();

				dist.sample(rng)
			}
			_ => unreachable!(),
		}
	}

	pub fn random_range_with(
		&self,
		low: f64,
		high: f64,
		value: f64,
		rng: &mut Rng,
	) -> f64 {
		assert!(low < high);
		assert!(
			low < value && value < high,
			"The value {value} is not in range {low}-{high}"
		);

		let ratio = (high - value) / (value - low);

		if let Some(point) = self.random_line(rng) {
			let ratio = ratio * point.exp();
			return interval_to_range(ratio, low, high);
		}
		if let Some(point) = self.random_semi_interval(rng) {
			let ratio = ratio * point;
			return interval_to_range(ratio, low, high);
		}

		match self {
			Distribution::Uniform => {
				let dist = Uniform::new(low, high).unwrap();

				dist.sample(rng)
			}
			Distribution::Triangular => {
				let center = (low + high) / 2.0;
				let dist = Triangular::new(low, high, center)
					.unwrap();

				dist.sample(rng)
			}
			_ => unreachable!(),
		}
	}
}

fn interval_to_range(ratio: f64, low: f64, high: f64) -> f64 {
	low + (high - low) / (ratio + 1.0)
}
