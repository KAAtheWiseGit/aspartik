#![allow(dead_code)]

use rand_distr::{Distribution as IDistribution, Normal, Triangular, Uniform};
use statrs::distribution::Laplace;

use crate::operator::Rng;

pub enum Distribution {
	Uniform,

	Triangular,

	Normal {
		mean: f64,
		std_dev: f64,
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
	/// Return a random value from `low` to `high`, with the center being in
	/// the middle of the segment.
	pub fn gen_range(&self, low: f64, high: f64, rng: &mut Rng) -> f64 {
		assert!(low < high);

		let center = (low + high) / 2.0;
		match self {
			Distribution::Uniform => {
				let dist = Uniform::new(low, high).unwrap();

				dist.sample(rng)
			}
			Distribution::Triangular => {
				let dist = Triangular::new(low, high, center)
					.unwrap();

				dist.sample(rng)
			}
			Distribution::Normal { mean, std_dev } => {
				let _dist =
					Normal::new(*mean, *std_dev).unwrap();

				todo!()
			}
			Distribution::Laplace { location, scale } => {
				let _dist = Laplace::new(*location, *scale)
					.unwrap();

				// TODO: sample from the range
				todo!()
			}
			_ => todo!(),
		}
	}

	pub fn gen_range_from(
		&self,
		low: f64,
		high: f64,
		center: f64,
		rng: &mut Rng,
	) -> f64 {
		assert!(low < center && center < high);

		match self {
			// Centering doesn't do anything for uniform
			// distributions
			Distribution::Uniform => {
				let dist = Uniform::new(low, high).unwrap();

				dist.sample(rng)
			}
			Distribution::Triangular => {
				let dist = Triangular::new(low, high, center)
					.unwrap();

				dist.sample(rng)
			}
			_ => todo!(),
		}
	}
}
