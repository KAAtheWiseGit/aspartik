#![allow(dead_code)]

use base::substitution::Model;

pub struct Likelihood<M: Model> {
	sites: Vec<Vec<M::Item>>,
	model: M,
	substitutions: Vec<(M::Substitution, M::Substitution)>,
	probabilities: Vec<Vec<M::Row>>,
}

impl<M: Model> Likelihood<M> {
	pub fn new<S>(sites: S, model: M) -> Self
	where
		S: IntoIterator<Item = Vec<M::Item>>,
	{
		let sites: Vec<_> = sites.into_iter().collect();
		let substitutions = vec![
			(
				M::Substitution::default(),
				M::Substitution::default()
			);
			sites[0].len()
		];

		let mut probabilities =
			vec![
				vec![M::Row::default(); sites[0].len()];
				sites.len()
			];
		for (site, probability) in sites.iter().zip(&mut probabilities)
		{
			for (i, base) in site.iter().enumerate() {
				probability[i] = M::to_row(base);
			}
		}

		Self {
			sites,
			model,
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
			.map(|p| M::probability(p.last().unwrap()))
			.sum()
	}
}
