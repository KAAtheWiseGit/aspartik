use anyhow::{bail, ensure, Result};
use rand::Rng as _;
use rand_distr::{
	Beta, Cauchy, ChiSquared, Distribution as _, Exp, Gamma, LogNormal,
	Normal, Poisson, StandardNormal, StudentT, Triangular, Uniform,
};
use statrs::distribution::{
	Beta as BetaS, Cauchy as CauchyS, Chi as ChiS,
	ChiSquared as ChiSquaredS, Continuous, Discrete, Exp as ExpS,
	Gamma as GammaS, InverseGamma as InverseGammaS, Laplace as LaplaceS,
	LogNormal as LogNormalS, Normal as NormalS, Poisson as PoissonS,
	StudentsT as StudentsTS,
};

use std::f64::consts::PI;

use crate::State;

#[derive(Debug, Clone)]
pub enum Distribution {
	// Interval
	Uniform,
	/// In [`random_range`] the distribution's center is set to the middle
	/// of the interval.  In [`random_range_with`] it's set to `value`.
	///
	///
	/// [`random_range`]: Distribution::random_range
	/// [`random_range_with`]: Distribution::random_range_with
	Triangular,
	/// Both `alpha` and `beta` must be real parameters.
	///
	/// As beta distribution is defined on the interval `[0, 1]` (or `(0,
	/// 1)`, depending on the parameters).  Therefore, the functions
	/// [`random_range`] and [`random_range_with`] will stretch it to fit
	/// the given `(low, high)` interval.  As the mean depends on the
	/// distribution's parameters, `random_range_with` works identically to
	/// `random_range` and ignores `value`.
	///
	///
	/// [`random_range`]: Distribution::random_range
	/// [`random_range_with`]: Distribution::random_range_with
	Beta {
		alpha: String,
		beta: String,
	},

	// Semi-interval
	/// `df` must be an integer parameter.
	Chi {
		df: String,
	},
	/// `df` must be an integer parameter.
	ChiSquared {
		df: String,
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
	LogNormal {
		mean: String,
		std_dev: String,
	},
	/// `rate` must be an integer parameter.
	Poisson {
		rate: String,
	},

	// Full line
	Cauchy {
		location: String,
		scale: String,
	},
	Laplace {
		location: String,
		scale: String,
	},
	Normal {
		mean: String,
		std_dev: String,
	},
	/// `df` must be a real parameter.
	StudentT {
		df: String,
	},
	/// Bactrian kernel as defined by [Yang & Rodríguez, 2013][ref].
	///
	/// `m` is the "spikiness" parameter, `std_dev` is the standard
	/// deviation of the underlying normal distribution.  The mean is set to
	/// 0.
	///
	/// [ref]: https://pmc.ncbi.nlm.nih.gov/articles/PMC3845170/
	Bactrian {
		m: String,
		std_dev: String,
	},
}

impl Distribution {
	/// Returns the density of a continuous distribution in the point `x`.
	///
	/// Throws an error for discrete distributions.
	pub fn pdf(&self, x: f64, state: &State) -> Result<f64> {
		match self {
			// Segment
			Distribution::Beta { alpha, beta } => {
				let alpha = state.one_real_param(alpha)?;
				let beta = state.one_real_param(beta)?;

				let dist = BetaS::new(alpha, beta)?;

				Ok(dist.pdf(x))
			}

			// Semi-interval
			Distribution::Chi { df } => {
				let df = state.one_integer_param(df)?;

				let dist = ChiS::new(df as u64)?;

				Ok(dist.pdf(x))
			}

			Distribution::ChiSquared { df } => {
				let df = state.one_integer_param(df)?;

				let dist = ChiSquaredS::new(df as f64)?;

				Ok(dist.pdf(x))
			}

			Distribution::Exponential { rate } => {
				let rate = state.one_real_param(rate)?;

				let dist = ExpS::new(rate)?;

				Ok(dist.pdf(x))
			}

			Distribution::Gamma { shape, scale } => {
				let shape = state.one_real_param(shape)?;
				let scale = state.one_real_param(scale)?;

				let dist = GammaS::new(shape, 1.0 / scale)?;

				Ok(dist.pdf(x))
			}

			Distribution::InverseGamma { shape, scale } => {
				let shape = state.one_real_param(shape)?;
				let scale = state.one_real_param(scale)?;

				let dist =
					InverseGammaS::new(shape, 1.0 / scale)?;

				Ok(dist.pdf(x))
			}

			Distribution::LogNormal { mean, std_dev } => {
				let mean = state.one_real_param(mean)?;
				let std_dev = state.one_real_param(std_dev)?;

				let dist = LogNormalS::new(mean, std_dev)?;

				Ok(dist.pdf(x))
			}

			// Full line
			Distribution::Cauchy { location, scale } => {
				let location =
					state.one_real_param(location)?;
				let scale = state.one_real_param(scale)?;

				let dist = CauchyS::new(location, scale)?;

				Ok(dist.pdf(x))
			}

			Distribution::Laplace { location, scale } => {
				let location =
					state.one_real_param(location)?;
				let scale = state.one_real_param(scale)?;

				let dist = LaplaceS::new(location, scale)?;

				Ok(dist.pdf(x))
			}

			Distribution::Normal { mean, std_dev } => {
				let mean = state.one_real_param(mean)?;
				let std_dev = state.one_real_param(std_dev)?;

				let dist = NormalS::new(mean, std_dev)?;

				Ok(dist.pdf(x))
			}

			Distribution::StudentT { df } => {
				let df = state.one_real_param(df)?;

				let dist = StudentsTS::new(0.0, 1.0, df)?;

				Ok(dist.pdf(x))
			}

			Distribution::Bactrian { m, std_dev } => {
				let m = state.one_real_param(m)?;
				ensure!(m < 1.0, "Bactrian m must be less than 1 (got {m})");
				let std_dev = state.one_real_param(std_dev)?;

				Ok(bactrian_pdf(x, m, std_dev))
			}

			Distribution::Uniform
			| Distribution::Triangular
			| Distribution::Poisson { .. } => bail!(
				"Distribution {self:?} does not have a PDF"
			),
		}
	}

	/// Returns the discrete probability of the point `x`.
	///
	/// Throws an error for continuous functions.
	pub fn pmf(&self, x: i64, state: &State) -> Result<f64> {
		match self {
			Distribution::Poisson { rate } => {
				let rate = state.one_integer_param(rate)?;

				let dist = PoissonS::new(rate as f64)?;

				Ok(dist.pmf(x as u64))
			}

			_ => bail!("Distribution {self:?} does not have a PMF"),
		}
	}

	/// Returns a number sampled from the whole real numbers line or `None`
	/// for distributions which don't support that.
	pub fn random_line(&self, state: &mut State) -> Result<Option<f64>> {
		match self {
			Distribution::Cauchy { location, scale } => {
				let location =
					state.one_real_param(location)?;
				let scale = state.one_real_param(scale)?;

				let dist = Cauchy::new(location, scale)?;

				Ok(Some(dist.sample(&mut state.rng)))
			}

			Distribution::Laplace { location, scale } => {
				let location =
					state.one_real_param(location)?;
				let scale = state.one_real_param(scale)?;

				// <https://en.wikipedia.org/wiki/Laplace_distribution#Random_variate_generation>
				let u: f64 = state.rng.random_range(-0.5..0.5);
				let x = location
					- scale * u.signum()
						* (1.0 - 2.0 * u.abs()).ln();

				Ok(Some(x))
			}

			Distribution::Normal { mean, std_dev } => {
				let mean = state.one_real_param(mean)?;
				let std_dev = state.one_real_param(std_dev)?;

				let dist = Normal::new(mean, std_dev).unwrap();
				Ok(Some(dist.sample(&mut state.rng)))
			}

			Distribution::StudentT { df } => {
				let df = state.one_real_param(df)?;

				let dist = StudentT::new(df).unwrap();
				Ok(Some(dist.sample(&mut state.rng)))
			}

			Distribution::Bactrian { m, std_dev } => {
				let m = state.one_real_param(m)?;
				ensure!(m < 1.0, "Bactrian m must be less than 1 (got {m})");
				let std_dev = state.one_real_param(std_dev)?;

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
			return Ok(Some(point.exp()));
		}

		match self {
			Distribution::Chi { df } => {
				let df = state.one_integer_param(df)?;

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
				let df = state.one_integer_param(df)?;

				let dist = ChiSquared::new(df as f64).unwrap();
				Ok(Some(dist.sample(&mut state.rng)))
			}

			Distribution::Exponential { rate } => {
				let rate = state.one_real_param(rate)?;

				let dist = Exp::new(rate).unwrap();
				Ok(Some(dist.sample(&mut state.rng)))
			}

			Distribution::Gamma { shape, scale } => {
				let shape = state.one_real_param(shape)?;
				let scale = state.one_real_param(scale)?;

				let dist = Gamma::new(shape, scale).unwrap();
				Ok(Some(dist.sample(&mut state.rng)))
			}

			Distribution::InverseGamma { shape, scale } => {
				let shape = state.one_real_param(shape)?;
				let scale = state.one_real_param(scale)?;

				let dist = Gamma::new(shape, scale).unwrap();
				let x = dist.sample(&mut state.rng);
				Ok(Some(1.0 / x))
			}

			Distribution::LogNormal { mean, std_dev } => {
				let mean = state.one_real_param(mean)?;
				let std_dev = state.one_real_param(std_dev)?;

				let dist =
					LogNormal::new(mean, std_dev).unwrap();
				Ok(Some(dist.sample(&mut state.rng)))
			}

			Distribution::Poisson { rate } => {
				let rate = state.one_integer_param(rate)?;

				let dist = Poisson::new(rate as f64)?;

				Ok(Some(dist.sample(&mut state.rng)))
			}

			_ => Ok(None),
		}
	}

	/// Return a random value from the `(low, high)` interval.
	///
	/// Distributions defined on an semi-open interval are sampled using
	/// [`random_semi_interval`] and are transformed into a point on the
	/// interval with the formula `low + (high - low) / (ratio + 1.0)`.
	/// Distributions defined on the full line are sampled with
	/// [`random_line`], exponentiated, and transformed in the same way as
	/// semi-interval distributions.
	///
	/// See [`Distribution`]'s documentation for behavior of the
	/// interval-defined distributions.
	///
	/// [`random_semi_interval`]: Distribution::random_semi_interval
	/// [`random_line`]: Distribution::random_line
	pub fn random_range(
		&self,
		low: f64,
		high: f64,
		state: &mut State,
	) -> Result<f64> {
		ensure!(
			low < high,
			"low ({low}) must be strictly smaller than high ({high})",
		);

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
				let alpha = state.one_real_param(alpha)?;
				let beta = state.one_real_param(beta)?;

				let dist = Beta::new(alpha, beta).unwrap();

				Ok(low + dist.sample(&mut state.rng)
					* (high - low))
			}

			_ => unreachable!(),
		}
	}

	/// Returns a value within the interval `(low, high)`, with the center
	/// of mass roughly near `value`.
	pub fn random_range_with(
		&self,
		low: f64,
		high: f64,
		value: f64,
		state: &mut State,
	) -> Result<f64> {
		ensure!(
			low < high,
			"low ({low}) must be strictly smaller than high ({high})",
		);
		ensure!(
			low < value && value < high,
			"The value ({value}) must lie between low and high ({low}-{high})",
		);

		let ratio = (high - value) / (value - low);

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

fn bactrian_pdf(x: f64, m: f64, s: f64) -> f64 {
	let m2 = m * m;
	let s2 = s * s;

	let c = 1. / (2. * s * (2. * PI * (1. - m2)).sqrt());

	let e1 = ((x + m * s).powi(2) / (2. * (1.0 - m2) * s2)).exp();
	let e2 = ((x - m * s).powi(2) / (2. * (1.0 - m2) * s2)).exp();

	c * (e1 + e2)
}
