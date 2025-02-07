#![allow(dead_code)]

use rand::Rng as _;
use rand_distr::{
	Beta, ChiSquared, Distribution as _, Exp, Gamma, LogNormal, Normal,
	StudentT, Triangular, Uniform,
};
use statrs::distribution::Laplace;

use crate::operator::Rng;

pub enum Distribution {
	Uniform,

	Triangular,

	Beta {
		alpha: f64,
		beta: f64,
	},

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

	ChiSquared {
		df: f64,
	},

	StudentT {
		df: f64,
	},

	Laplace {
		location: f64,
		scale: f64,
	},

	LogNormal {
		mean: f64,
		std_dev: f64,
	},

	/// <https://pmc.ncbi.nlm.nih.gov/articles/PMC3845170/>
	Bactrian {
		m: f64,
		std_dev: f64,
	},
}

impl Distribution {
	/// Returns a number sampled from the whole real numbers line or `None`
	/// for distributions which don't support that.
	pub fn random_line(&self, rng: &mut Rng) -> Option<f64> {
		match self {
			Distribution::Normal { mean, std_dev } => {
				let dist =
					Normal::new(*mean, *std_dev).unwrap();
				Some(dist.sample(rng))
			}
			Distribution::Gamma { shape, scale } => {
				let dist = Gamma::new(*shape, *scale).unwrap();
				Some(dist.sample(rng))
			}
			Distribution::ChiSquared { df } => {
				let dist = ChiSquared::new(*df).unwrap();
				Some(dist.sample(rng))
			}
			Distribution::StudentT { df } => {
				let dist = StudentT::new(*df).unwrap();
				Some(dist.sample(rng))
			}
			Distribution::Laplace { location, scale } => {
				let _dist = Laplace::new(*location, *scale)
					.unwrap();

				todo!("`statrs` depends on an older `rand` version")
			}
			Distribution::Bactrian { m, std_dev } => {
				let dist = Normal::new(0.0, *std_dev).unwrap();
				let mut point = dist.sample(rng);
				point *= (1.0 - m * m).sqrt();
				if rng.random::<bool>() {
					point += m;
				} else {
					point -= m;
				}

				Some(point)
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
				let dist = Exp::new(*rate).unwrap();
				Some(dist.sample(rng))
			}
			Distribution::LogNormal { mean, std_dev } => {
				let dist = LogNormal::new(*mean, *std_dev)
					.unwrap();
				Some(dist.sample(rng))
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
			Distribution::Beta { alpha, beta } => {
				let dist = Beta::new(*alpha, *beta).unwrap();

				low + dist.sample(rng) * (high - low)
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
			Distribution::Beta { .. } => {
				todo!()
			}
			_ => unreachable!(),
		}
	}
}

fn interval_to_range(ratio: f64, low: f64, high: f64) -> f64 {
	low + (high - low) / (ratio + 1.0)
}
