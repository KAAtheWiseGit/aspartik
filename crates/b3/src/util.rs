//! Kitchen sink utilities.

use std::{fs::File, path::Path};

use crate::Tree;
use base::{seq::DnaSeq, DnaNucleoBase};
use io::fasta::FastaReader;
use linalg::Vector;

pub fn random_tree(data: &Path) -> (Vec<DnaSeq>, Tree) {
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

	// A very primitive comb-like tree
	let num_leaves = seqs.len();
	let num_internals = num_leaves - 1;
	let mut children = vec![];
	for i in 0..num_internals {
		// current node id is num_leaves + i

		// leaf child
		children.push(i);

		// the next internal or the second child for the last internal
		if i + 1 < num_internals {
			children.push(num_leaves + i + 1);
		} else {
			children.push(i + 1);
		}
	}

	let mut weights = vec![];
	for _ in 0..num_leaves {
		weights.push(num_internals as f64 * 0.01);
	}
	for i in 0..num_internals {
		weights.push(i as f64 * 0.01);
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
