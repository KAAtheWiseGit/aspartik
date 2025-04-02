//! Kitchen sink utilities.

use anyhow::{bail, Result};
use pyo3::prelude::*;
use pyo3::types::{PySlice, PySliceIndices, PyTuple};

use std::{fs::File, path::Path};

use crate::Tree;
use base::{seq::DnaSeq, DnaNucleoBase};
use io::fasta::FastaReader;
use linalg::Vector;

pub fn make_tree(data: &Path) -> (Vec<DnaSeq>, Tree) {
	let fasta: FastaReader<DnaNucleoBase, _> =
		FastaReader::new(File::open(data).unwrap());

	let (seqs, names): (Vec<_>, Vec<_>) = fasta
		.map(|record| record.unwrap())
		.map(|record| {
			let name = record.description().to_owned();
			let seq: DnaSeq = record.into();
			(seq, name)
		})
		.unzip();

	// A very primitive mostly balanced tree
	let num_leaves = seqs.len();
	let num_internals = num_leaves - 1;

	use std::collections::VecDeque;
	let mut dangling: VecDeque<usize> = (0..num_leaves).collect();

	let mut children = vec![];
	for i in 0..num_internals {
		children.push(dangling.pop_front().unwrap());
		children.push(dangling.pop_front().unwrap());

		dangling.push_back(i + num_leaves);
	}

	let mut weights = vec![0.0; num_leaves + num_internals];
	let mut nodes = VecDeque::from([num_internals - 1]);
	while let Some(node) = nodes.pop_back() {
		let left = children[node * 2];
		let right = children[node * 2 + 1];
		weights[left] = weights[node + num_leaves] + 1.0;
		weights[right] = weights[node + num_leaves] + 1.0;

		if left >= num_leaves {
			nodes.push_back(left - num_leaves);
		}
		if right >= num_leaves {
			nodes.push_back(right - num_leaves);
		}
	}

	let tree = Tree::new(names, &weights, &children);

	(seqs, tree)
}

pub fn dna_to_rows(seqs: &[DnaSeq]) -> Vec<Vec<Vector<f64, 4>>> {
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

	#[allow(clippy::needless_range_loop)]
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
