use lapack_sys as lapack;

use std::ffi::{c_char, c_int};

use crate::{RowMatrix, Vector};

impl<const N: usize> RowMatrix<f64, N, N> {
	pub fn eigen(
		&self,
	) -> (Vector<f64, N>, RowMatrix<f64, N, N>, RowMatrix<f64, N, N>) {
		if self.is_symmetric() {
			todo!()
		} else {
			let (_, values, left, right) = dgeev(self, true, true);
			(values, left, right)
		}
	}

	pub fn eigenvectors(&self) -> RowMatrix<f64, N, N> {
		let (_, _, _, out) = dgeev(self, false, true);
		out
	}

	pub fn eigenvalues(&self) -> Vector<f64, N> {
		let (_, out, _, _) = dgeev(self, false, false);
		out
	}
}

fn calc_char(yes: bool) -> c_char {
	if yes {
		b'V' as c_char
	} else {
		b'N' as c_char
	}
}

fn dgeev<const N: usize>(
	matrix: &RowMatrix<f64, N, N>,
	left: bool,
	right: bool,
) -> (
	i32,
	Vector<f64, N>,
	RowMatrix<f64, N, N>,
	RowMatrix<f64, N, N>,
) {
	let jobvl = calc_char(left);
	let jobvr = calc_char(right);
	let n = N as c_int;

	let mut a = *matrix;
	let lda = N as c_int;

	let mut wr: Vector<f64, N> = Default::default();
	let mut wi: Vector<f64, N> = Default::default();

	let mut vl: RowMatrix<f64, N, N> = Default::default();
	let ldvl = N as c_int;

	let mut vr: RowMatrix<f64, N, N> = Default::default();
	let ldvr = N as c_int;

	let mut work = vec![0f64; 4 * N];
	let lwork = work.len() as c_int;

	let mut info = 0i32;

	unsafe {
		lapack::dgeev_(
			&jobvl,
			&jobvr,
			&n,
			a.as_mut_ptr(),
			&lda,
			wr.as_mut_ptr(),
			wi.as_mut_ptr(),
			vl[0].as_mut_ptr(),
			&ldvl,
			vr[0].as_mut_ptr(),
			&ldvr,
			work.as_mut_ptr(),
			&lwork,
			&mut info,
		)
	}

	(info, wr, vl, vr)
}
