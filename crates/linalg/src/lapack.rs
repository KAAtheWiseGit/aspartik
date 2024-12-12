//! Wrappers around the LAPACK functions and utilities for them.

use std::ffi::{c_char, c_int};

use crate::{RowMatrix, Vector};

fn calc_char(yes: bool) -> c_char {
	if yes {
		b'V' as c_char
	} else {
		b'N' as c_char
	}
}

pub fn dgeev<const N: usize>(
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
		lapack_sys::dgeev_(
			&jobvl,
			&jobvr,
			&n,
			a.as_mut_ptr(),
			&lda,
			wr.as_mut_ptr(),
			wi.as_mut_ptr(),
			vl.as_mut_ptr(),
			&ldvl,
			vr.as_mut_ptr(),
			&ldvr,
			work.as_mut_ptr(),
			&lwork,
			&mut info,
		)
	}

	(info, wr, vl, vr)
}

pub fn dsyev<const N: usize>(
	matrix: &RowMatrix<f64, N, N>,
	compute_eigenvectors: bool,
) -> (i32, Vector<f64, N>, RowMatrix<f64, N, N>) {
	let jobz = calc_char(compute_eigenvectors);
	// doesn't matter, as the input must be symmetric
	let uplo = b'U' as c_char;

	let n = N as c_int;

	let mut a = *matrix;
	let lda = N as c_int;

	let mut w: Vector<f64, N> = Default::default();

	let mut work = vec![0.0; 4 * N];
	let lwork = 4 * N as c_int;

	let mut info: i32 = 0;

	unsafe {
		lapack_sys::dsyev_(
			&jobz,
			&uplo,
			&n,
			a.as_mut_ptr(),
			&lda,
			w.as_mut_ptr(),
			work.as_mut_ptr(),
			&lwork,
			&mut info,
		)
	}

	(info, w, a)
}
