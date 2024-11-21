use super::Likelihood;
use crate::tree::Update;
use base::substitution::Model;
use shchurvec::ShchurVec;

pub struct CpuLikelihood<M: Model> {
	model: M,
	substitutions: ShchurVec<M::Substitution>,
	probabilities: Vec<ShchurVec<M::Row>>,

	updated_nodes: Option<Vec<usize>>,
}

impl<M: Model> Likelihood for CpuLikelihood<M> {
	type Model = M;

	fn new(
		sites: Vec<Vec<<Self::Model as Model>::Item>>,
		model: Self::Model,
	) -> Self {
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
			for (i, base) in site.iter().enumerate() {
				probability.set(i, M::to_row(base));
			}
			probability.accept();
		}

		Self {
			model,
			substitutions,
			probabilities,

			updated_nodes: None,
		}
	}

	fn propose(&mut self, update: Update) {
		self.update_substitutions(&update.edges, &update.lengths);
		self.update_probabilities(&update.nodes, &update.children);
	}

	fn likelihood(&self) -> f64 {
		self.probabilities
			.iter()
			.map(|p| M::probability(p.last().unwrap()))
			.sum()
	}

	fn accept(&mut self) {
		self.substitutions.accept();
		for probability in &mut self.probabilities {
			probability.accept();
		}
	}

	fn reject(&mut self) {
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

impl<M: Model> CpuLikelihood<M> {
	fn update_substitutions(&mut self, edges: &[usize], distances: &[f64]) {
		for (edge, distance) in edges.iter().zip(distances) {
			self.substitutions
				.set(*edge, self.model.substitution(*distance));
		}
	}

	fn update_probabilities(
		&mut self,
		nodes: &[usize],
		children: &[(usize, usize)],
	) {
		let num_leaves = (self.probabilities[0].len() + 1) / 2;
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
}
