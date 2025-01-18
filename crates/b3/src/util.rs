//! Kitchen sink utilities.

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
		weights[left] = weights[node + num_leaves] + 0.01;
		weights[right] = weights[node + num_leaves] + 0.01;

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
