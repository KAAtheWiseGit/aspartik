use approx::assert_relative_eq;
use proptest::prelude::*;

use linalg::RowMatrix;

#[test]
fn roundtrip() {
	let jc = RowMatrix::from([
		[-1.0, 1.0 / 3.0, 1.0 / 3.0, 1.0 / 3.0],
		[1.0 / 3.0, -1.0, 1.0 / 3.0, 1.0 / 3.0],
		[1.0 / 3.0, 1.0 / 3.0, -1.0, 1.0 / 3.0],
		[1.0 / 3.0, 1.0 / 3.0, 1.0 / 3.0, -1.0],
	]);

	let diag = RowMatrix::from_diagonal(jc.eigenvalues());
	let eigenvectors = jc.eigenvectors();
	let inverse = eigenvectors.inverse();

	assert_relative_eq!(jc, inverse * diag * eigenvectors);
	assert_relative_eq!(jc * 0.1, inverse * (diag * 0.1) * eigenvectors);
}

proptest! {
	// XXX: proper generation strategy which doesn't create large
	// overflowing values
	#[test]
	fn eigen_2(
		a in 0.0..100.0, b in 0.0..100.0,
		c in 0.0..100.0, d in 0.0..100.0,
	) {
		let m = RowMatrix::from([[a, b], [c, d]]);

		let eigenvalues = m.eigenvalues();
		let eigenvectors = m.eigenvectors();

		for i in 0..2 {
			assert_relative_eq!(
				m * eigenvectors[i],
				eigenvectors[i] * eigenvalues[i],
				max_relative = 1e-10,
			);
		}
	}

	// XXX: diagonal matrix strategy
	#[test]
	fn inverse_2(
		a in 0.1..100.0, b in 0.1..100.0,
		c in 0.1..100.0,
	) {
		let m = RowMatrix::from([[a, b], [b, c]]);

		let diag = RowMatrix::from_diagonal(m.eigenvalues());
		let eigenvectors = m.eigenvectors();
		let inverse = eigenvectors.inverse();

		println!("diag = {diag}");
		println!("eigenvectors = {eigenvectors}");
		println!("inverse = {inverse}");

		assert_relative_eq!(
			m,
			inverse * diag * eigenvectors,
			max_relative = 1e-10
		);
	}
}
