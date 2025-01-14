use linalg::RowMatrix;

pub type Substitution<const N: usize> = RowMatrix<f64, N, N>;

pub fn jukes_cantor() -> Substitution<4> {
	gtr(
		1.0, 1.0, 1.0, 1.0, 1.0, 1.0, // equal exchange
		0.25, 0.25, 0.25, 0.25, // equal probabilites
	)
}

pub fn k80(kappa: f64) -> Substitution<4> {
	gtr(
		// b, e
		1.0, kappa, 1.0, 1.0, kappa, 1.0,
		// equal probabilites
		0.25, 0.25, 0.25, 0.25,
	)
}

pub fn f81(p_a: f64, p_c: f64, p_g: f64, p_t: f64) -> Substitution<4> {
	gtr(1.0, 1.0, 1.0, 1.0, 1.0, 1.0, p_a, p_c, p_g, p_t)
}

pub fn hky(
	kappa: f64,
	p_a: f64,
	p_c: f64,
	p_g: f64,
	p_t: f64,
) -> Substitution<4> {
	gtr(
		1.0, kappa, 1.0, 1.0, kappa, 1.0, // exchange
		p_a, p_c, p_g, p_t, // probabilites
	)
}

// Sorry, a lot of free parameters here.
#[allow(clippy::too_many_arguments)]
pub fn gtr(
	// Exchange
	a: f64,
	b: f64,
	c: f64,
	d: f64,
	e: f64,
	f: f64,

	p_a: f64,
	p_c: f64,
	p_g: f64,
	p_t: f64,
) -> Substitution<4> {
	assert_eq!(p_a + p_c + p_g + p_t, 1.0);

	let scale = 1.0
		/ (2.0 * (a * p_a * p_c
			+ b * p_a * p_g + c * p_a * p_t
			+ d * p_c * p_g + e * p_c * p_t
			+ f * p_g * p_t));

	let sum1 = a * p_c + b * p_g + c * p_t;
	let sum2 = a * p_a + d * p_g + e * p_t;
	let sum3 = b * p_a + d * p_c + f * p_t;
	let sum4 = c * p_a + e * p_c + f * p_g;

	RowMatrix::from([
		[-sum1, a * p_c, b * p_g, c * p_t],
		[a * p_a, -sum2, d * p_g, e * p_t],
		[b * p_a, d * p_c, -sum3, f * p_t],
		[c * p_a, e * p_c, f * p_g, -sum4],
	]) * scale
}
