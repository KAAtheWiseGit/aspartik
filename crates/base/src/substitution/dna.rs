use nalgebra::Matrix4;

/// Instantaneous transition rate matrix.
type Q = Matrix4<f64>;

#[rustfmt::skip]
pub fn jukes_cantor() -> Q {
	const THIRD: f64 = 1.0 / 3.0;

	Q::new(
		-1.0, THIRD, THIRD, THIRD,
		THIRD, -1.0, THIRD, THIRD,
		THIRD, THIRD, -1.0, THIRD,
		THIRD, THIRD, THIRD, -1.0,
	)
}

#[rustfmt::skip]
pub fn k80(kappa: f64) -> Q {
	let scale = 1.0 / (2.0 + kappa);

	Q::new(
		-2.0 - kappa, 1.0, kappa, 1.0,
		1.0, -2.0 - kappa, 1.0, kappa,
		kappa, 1.0, -2.0 - kappa, 1.0,
		1.0, kappa, 1.0, -2.0 - kappa,
	) * scale
}

#[rustfmt::skip]
pub fn f81(a: f64, c: f64, g: f64) -> Q {
	let t = 1.0 - a - c - g;
	let scale = 1.0 / (1.0 - a * a - c * c - g * g - t * t);

	Q::new(
		a - 1.0, c, g, t,
		a, c - 1.0, g, t,
		a, c, g - 1.0, t,
		a, c, g, t - 1.0,
	) * scale
}

// TODO: HKY, GTR
