//! Kitchen sink utilities.
use anyhow::{bail, Result};
use pyo3::prelude::*;
use pyo3::types::{PySlice, PySliceIndices, PyTuple};

use std::fs::File;

use crate::likelihood::Row;
use base::{seq::DnaSeq, DnaNucleoBase};
use io::fasta::FastaReader;
use linalg::Vector;

pub fn dna_to_rows(seqs: &[DnaSeq]) -> Vec<Vec<Row<4>>> {
	let width = seqs[0].len();
	let height = seqs.len();

	let mut out = vec![vec![Vector::default(); height]; width];

	// TODO: find a place for this
	fn to_row(base: &DnaNucleoBase) -> Vector<f64, 4> {
		match base {
			DnaNucleoBase::Adenine => [1.0, 0.0, 0.0, 0.0],
			DnaNucleoBase::Cytosine => [0.0, 1.0, 0.0, 0.0],
			DnaNucleoBase::Guanine => [0.0, 0.0, 1.0, 0.0],
			DnaNucleoBase::Thymine => [0.0, 0.0, 0.0, 1.0],

			_ => [0.25, 0.25, 0.25, 0.25],
		}
		.into()
	}

	#[expect(clippy::needless_range_loop)]
	for i in 0..width {
		for j in 0..height {
			out[i][j] = to_row(&seqs[j][i])
		}
	}

	out
}

#[derive(Debug)]
pub struct SlicesIter {
	slices: Vec<PySliceIndices>,
	slice_index: usize,
	curr_index: isize,
}

impl Iterator for SlicesIter {
	type Item = usize;

	fn next(&mut self) -> Option<usize> {
		// get currently active slice
		let mut slice = self.slices.get(self.slice_index)?;

		self.curr_index += slice.step;
		// if we have overrun the current slice, advance to the
		// next one
		if self.curr_index >= slice.stop {
			self.slice_index += 1;
			slice = self.slices.get(self.slice_index)?;
			// set the index to the start of the next slice
			self.curr_index = slice.start;
		}

		Some(self.curr_index as usize)
	}
}

/// Iterator over the numbers specified by either a single slice or a tuple of
/// slices.
pub fn slices_iter(key: Bound<PyAny>, length: usize) -> Result<SlicesIter> {
	let mut slices = Vec::new();
	let length = length as isize;

	if let Ok(slice) = key.downcast::<PySlice>() {
		let slice = slice.indices(length)?;
		slices.push(slice);
	} else if let Ok(tuple) = key.downcast::<PyTuple>() {
		for item in tuple.into_iter() {
			let Ok(slice) = item.downcast::<PySlice>() else {
				bail!(
					"Expected tuple members to be slices, got {}",
					item.get_type().name()?
				)
			};
			let slice = slice.indices(length)?;
			if slice.step < 0 {
				bail!("Negative slice step is not supported");
			}
			slices.push(slice);
		}

		// Slices will never be empty, because `list[]` is not a valid
		// Python syntax
	} else {
		bail!(
			"Expected a slice or a tuple of slices, got {}",
			key.get_type().name()?
		);
	}

	// Since `curr_index` is isize and the `indices` method will only return
	// positive values, we can do this
	let start = slices[0].start - slices[0].step;

	Ok(SlicesIter {
		slices,
		slice_index: 0,
		curr_index: start,
	})
}

#[macro_export]
macro_rules! py_bail {
	($type:ident, $($arg:tt)*) => {
		return Err($type::new_err(format!($($arg)*)).into());
	}
}

#[macro_export]
macro_rules! py_call_method {
	($py:ident, $obj:expr, $name:literal) => {{
		use pyo3::intern;
		$obj.call_method0($py, intern!($py, $name))
	}};
	($py:ident, $obj:expr, $name:literal, $args:expr) => {{
		use pyo3::intern;
		$obj.call_method1($py, intern!($py, $name), $args)
	}};
}

pub fn read_fasta(path: &str) -> Result<Vec<Vec<Row<4>>>> {
	let file = File::open(path)?;
	let fasta = FastaReader::<DnaNucleoBase, _>::new(file);
	let mut seqs = Vec::new();
	let mut names = Vec::new();

	for record in fasta {
		let record = record?;
		names.push(record.description().to_owned());
		seqs.push(record.into_sequence());
	}

	Ok(dna_to_rows(&seqs))
}
