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

	pub fn inverse(&self) -> Self {
		let (lu, ipiv) = lapack::dgetrf(self);
		lapack::dgetri(&lu, &ipiv)
	}
}

fn eigen<const N: usize>(
	matrix: &RowMatrix<f64, N, N>,
	left: bool,
	right: bool,
) -> (Vector<f64, N>, RowMatrix<f64, N, N>, RowMatrix<f64, N, N>) {
	if matrix.is_symmetric() {
		let (values, vectors) = lapack::dsyev(matrix, left || right);
		(values, vectors, vectors)
	} else {
		lapack::dgeev(matrix, left, right)
	}
}
