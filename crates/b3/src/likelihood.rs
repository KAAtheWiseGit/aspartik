#![allow(dead_code)]

use base::{seq::DnaSeq, DnaNucleoBase};
use linalg::{RowMatrix, Vector};

type Substitution = RowMatrix<f64, 4, 4>;
type Row = Vector<f64, 4>;

pub struct DnaLikelihood {
	sites: Vec<DnaSeq>,
	// TODO: model
	substitutions: Vec<(Substitution, Substitution)>,
	probabilities: Vec<Vec<Row>>,
}

impl DnaLikelihood {
	pub fn new<S>(sites: S) -> Self
	where
		S: IntoIterator<Item = DnaSeq>,
	{
		let sites: Vec<_> = sites.into_iter().collect();
		let substitutions =
			vec![
				(Substitution::default(), Substitution::default());
				sites[0].len()
			];

		let mut probabilities =
			vec![vec![Row::default(); sites[0].len()]; sites.len()];
		for (site, probability) in sites.iter().zip(&mut probabilities)
		{
			for (i, base) in site.iter().enumerate() {
				probability[i] = to_row(base);
			}
		}

		Self {
			sites,
			substitutions,
			probabilities,
		}
	}

	pub fn update_substitutions(&mut self, /* nodes, distances, l/r */) {
		todo!()
	}

	pub fn update_probabilities(&mut self /* nodes, tree */) {
		todo!()
	}

	pub fn likelihood(&self) -> f64 {
		self.probabilities
			.iter()
			.map(|p| p.last().unwrap().sum().ln())
			.sum()
	}
}

fn to_row(base: &DnaNucleoBase) -> Row {
	match base {
		DnaNucleoBase::Adenine => [1.0, 0.0, 0.0, 0.0],
		DnaNucleoBase::Cytosine => [0.0, 0.0, 1.0, 0.0],
		DnaNucleoBase::Guanine => [0.0, 0.0, 1.0, 0.0],
		DnaNucleoBase::Thymine => [0.0, 0.0, 0.0, 1.0],
		// TODO: other types
		_ => [0.25, 0.25, 0.25, 0.25],
	}
	.into()
}
