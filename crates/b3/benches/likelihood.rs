use criterion::{criterion_group, criterion_main, Criterion};

use b3::{
	mcmc::{run, Config},
	operator::{scheduler::TurnScheduler, NarrowExchange, Operator},
	probability::Compound,
	state::State,
	tree::Tree,
};
use base::{seq::DnaSeq, substitution::dna::jukes_cantor};

fn likelihood() {
	let seqs = vec![
		DnaSeq::try_from("AAGCT".repeat(350).as_ref()).unwrap(),
		DnaSeq::try_from("CAGCT".repeat(350).as_ref()).unwrap(),
		DnaSeq::try_from("ATGCA".repeat(350).as_ref()).unwrap(),
		DnaSeq::try_from("ATGCT".repeat(350).as_ref()).unwrap(),
		DnaSeq::try_from("TAGCA".repeat(350).as_ref()).unwrap(),
	];
	let children = vec![(2, 3), (0, 1), (5, 4), (6, 7)];
	let weights = vec![0.75, 0.60, 1.1, 0.9, 0.85, 0.8, 0.5, 0.7, 0.3];
	let tree = Tree::new(seqs.clone(), jukes_cantor(), &weights, &children);
	let mut state = State::new(tree);
	let prior = Box::new(Compound::new([]));

	let operator: Box<dyn Operator> = Box::new(NarrowExchange());
	let mut scheduler = TurnScheduler::new([operator]);

	let config = Config {
		burnin: 0,
		length: 1_000_000,
		state: 10_000,
		trees: 10_000,
	};

	run(config, &mut state, prior, &mut scheduler);
}

fn bench(c: &mut Criterion) {
	c.bench_function("likelihood", |b| b.iter(likelihood));
}

criterion_group!(
	name = benches;
	config = Criterion::default().sample_size(10);
	targets = bench
);
criterion_main!(benches);
