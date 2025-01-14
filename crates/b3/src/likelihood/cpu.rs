use super::{Likelihood, Row};
use base::substitution::Substitution;
use shchurvec::ShchurVec;

pub struct CpuLikelihood<const N: usize> {
	probabilities: Vec<ShchurVec<Row<N>>>,

	updated_nodes: Vec<usize>,
}

impl<const N: usize> Likelihood for CpuLikelihood<N> {
	type Row = Row<N>;
	type Substitution = Substitution<N>;

	fn propose(
		&mut self,
		nodes: &[usize],
		substitutions: &[Self::Substitution],
		children: &[usize],
	) {
		assert_eq!(nodes.len() * 2, substitutions.len());
		assert_eq!(nodes.len() * 2, children.len());

		self.updated_nodes = nodes.to_vec();

		for probability in &mut self.probabilities {
			for i in 0..nodes.len() {
				let left = substitutions[i * 2]
					* probability[children[i * 2]];
				let right = substitutions[i * 2 + 1]
					* probability[children[i * 2 + 1]];
				probability.set(i, left * right);
			}
		}
	}

	fn likelihood(&self) -> f64 {
		self.probabilities
			.iter()
			.map(|p| p.last().unwrap().sum().ln())
			.sum()
	}

	fn accept(&mut self) {
		for probability in &mut self.probabilities {
			probability.accept();
		}
	}

	fn reject(&mut self) {
		let nodes = std::mem::take(&mut self.updated_nodes);

		for probability in &mut self.probabilities {
			for node in &nodes {
				probability.unset(*node);
			}
			// All of the edited items have been manually unset, so
			// there's no need for `accept` or `reject`.
		}
	}
}

impl<const N: usize> CpuLikelihood<N> {
	pub fn new(sites: Vec<Vec<Row<N>>>) -> Self {
		let mut probabilities = vec![
			ShchurVec::repeat(
				Row::<N>::default(),
				sites[0].len() * 2 - 1
			);
			sites.len()
		];

		for (i, site) in sites.iter().enumerate() {
			for (j, row) in site.iter().enumerate() {
				probabilities[i].set(j, *row);
			}
		}

		for (rows, probability) in sites.iter().zip(&mut probabilities)
		{
			for (i, row) in rows.iter().enumerate() {
				probability.set(i, *row);
			}
			probability.accept();
		}

		Self {
			probabilities,
			updated_nodes: Vec::new(),
		}
	}
}
