use crate::{lapack, RowMatrix, Vector};

impl<const N: usize> RowMatrix<f64, N, N> {
	pub fn eigen(
		&self,
	) -> (Vector<f64, N>, RowMatrix<f64, N, N>, RowMatrix<f64, N, N>) {
		eigen(self, true, true)
	}

	pub fn eigenvectors(&self) -> RowMatrix<f64, N, N> {
		let (_, _, right) = eigen(self, false, true);
		right
	}

	pub fn eigenvalues(&self) -> Vector<f64, N> {
		let (values, _, _) = eigen(self, false, false);
		values
	}
}

fn eigen<const N: usize>(
	matrix: &RowMatrix<f64, N, N>,
	left: bool,
	right: bool,
) -> (Vector<f64, N>, RowMatrix<f64, N, N>, RowMatrix<f64, N, N>) {
	if matrix.is_symmetric() {
		let (_, values, vectors) = lapack::dsyev(matrix, left || right);
		(values, vectors, vectors)
	} else {
		let (_, values, left, right) =
			lapack::dgeev(matrix, left, right);
		(values, left, right)
	}
}
