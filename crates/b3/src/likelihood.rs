#![allow(dead_code)]

use base::substitution::Model;
use shchurvec::ShchurVec;

pub struct Likelihood<M: Model> {
	sites: Vec<Vec<M::Item>>,
	model: M,
	substitutions: ShchurVec<M::Substitution>,
	probabilities: Vec<ShchurVec<M::Row>>,

	updated_nodes: Option<Vec<usize>>,
}

impl<M: Model> Likelihood<M> {
	pub fn new<S>(sites: S, model: M) -> Self
	where
		S: IntoIterator<Item = Vec<M::Item>>,
	{
		let sites: Vec<_> = sites.into_iter().collect();
		let substitutions = ShchurVec::repeat(
			M::Substitution::default(),
			sites[0].len() * 2,
		);

		let mut probabilities = vec![
			ShchurVec::repeat(
				M::Row::default(),
				sites[0].len() * 2 - 1
			);
			sites.len()
		];
		for (site, probability) in sites.iter().zip(&mut probabilities)
		{
			// This will fill up the `ShchurVec` hash table to the
			// size equal or bigger to that of the main storage
			// area.  And `accept` doesn't free that.  So, if memory
			// usage gets out of hand, this is a likely culprit.
			for (i, base) in site.iter().enumerate() {
				probability.set(i, M::to_row(base));
			}
			probability.accept();
		}

		Self {
			sites,
			model,
			substitutions,
			probabilities,

			updated_nodes: None,
		}
	}

	pub fn update_substitutions(
		&mut self,
		edges: &[usize],
		distances: &[f64],
	) {
		for (edge, distance) in edges.iter().zip(distances) {
			self.substitutions
				.set(*edge, self.model.substitution(*distance));
		}
	}

	pub fn update_probabilities(
		&mut self,
		num_leaves: usize,
		nodes: &[usize],
		children: &[(usize, usize)],
	) {
		self.updated_nodes = Some(nodes.into());

		for probability in &mut self.probabilities {
			for (i, (left, right)) in nodes.iter().zip(children) {
				let left = self.substitutions
					[(i - num_leaves) * 2] * probability
					[*left];
				let right = self.substitutions
					[(i - num_leaves) * 2 + 1]
					* probability[*right];
				probability.set(*i, left * right);
			}
		}
	}

	pub fn likelihood(&self) -> f64 {
		self.probabilities
			.iter()
			.map(|p| M::probability(p.last().unwrap()))
			.sum()
	}

	pub fn accept(&mut self) {
		self.substitutions.accept();
		for probability in &mut self.probabilities {
			probability.accept();
		}
	}

	pub fn reject(&mut self) {
		let nodes = self.updated_nodes.take().expect(
			"Reject must be called after 'update_probabilities'",
		);

		self.substitutions.reject();

		for probability in &mut self.probabilities {
			for node in &nodes {
				probability.unset(*node);
			}
			// All of the edited items have been manually unset, so
			// there's no need for `accept` or `reject`.
		}
	}
}
