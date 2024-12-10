use linalg::RowMatrix;

pub type Substitution<const N: usize> = RowMatrix<f64, N, N>;

pub fn jukes_cantor() -> Substitution<4> {
	RowMatrix::from([
		[-3.0, 1.0, 1.0, 1.0],
		[1.0, -3.0, 1.0, 1.0],
		[1.0, 1.0, -3.0, 1.0],
		[1.0, 1.0, 1.0, -3.0],
	]) * (1.0 / 3.0)
}

pub fn k80(kappa: f64) -> Substitution<4> {
	let diag = -2.0 - kappa;
	let scale = 1.0 / (2.0 + kappa);

	RowMatrix::from([
		[diag, 1.0, kappa, 1.0],
		[1.0, diag, 1.0, kappa],
		[kappa, 1.0, diag, 1.0],
		[1.0, kappa, 1.0, diag],
	]) * scale
}

pub fn f81(a: f64, c: f64, g: f64, t: f64) -> Substitution<4> {
	assert_eq!(a + c + g + t, 1.0);

	let scale = 1.0 / (1.0 - a * a - c * c - g * g - t * t);

	RowMatrix::from([
		[a - 1.0, c, g, t],
		[a, c - 1.0, g, t],
		[a, c, g - 1.0, t],
		[a, c, g, t - 1.0],
	]) * scale
}
