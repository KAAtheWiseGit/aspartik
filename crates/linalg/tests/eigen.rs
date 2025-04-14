use linalg::{RowMatrix, Vector};

const EPSILON: f64 = 1e-15;

fn assert_vector_eq<const N: usize>(
	left: Vector<f64, N>,
	right: Vector<f64, N>,
) {
	for i in 0..N {
		let diff = (left[i] - right[i]).abs();
		if diff > EPSILON {
			panic!(
				"Vectors are not equal:\n\t{:?}\n\t{:?}\n",
				left, right
			);
		}
	}
}

fn assert_matrix_eq<const N: usize, const M: usize>(
	left: RowMatrix<f64, N, M>,
	right: RowMatrix<f64, N, M>,
) {
	for i in 0..N {
		for j in 0..M {
			let diff = (left[i][j] - right[i][j]).abs();
			if diff > EPSILON {
				panic!(
					"Matrices are not equal:\n\t{:?}\n\t{:?}\n",
					left, right
				);
			}
		}
	}
}

#[test]
fn basic_eigenvalues() {
	let m = RowMatrix::from([[4.0, 1.0], [2.0, -1.0]]);
	assert_eq!([4.372281323269014, -1.3722813232690143], m.eigenvalues());

	let m = RowMatrix::from([
		[1.0, -1.0 / 3.0, -1.0 / 3.0, -1.0 / 3.0],
		[-1.0 / 3.0, 1.0, -1.0 / 3.0, -1.0 / 3.0],
		[-1.0 / 3.0, -1.0 / 3.0, 1.0, -1.0 / 3.0],
		[-1.0 / 3.0, -1.0 / 3.0, -1.0 / 3.0, 1.0],
	]);
	assert_vector_eq(
		[0.0, 4.0 / 3.0, 4.0 / 3.0, 4.0 / 3.0].into(),
		m.eigenvalues(),
	);
}

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

	assert_matrix_eq(jc, inverse * diag * eigenvectors);
	assert_matrix_eq(jc * 0.1, inverse * (diag * 0.1) * eigenvectors);
}
