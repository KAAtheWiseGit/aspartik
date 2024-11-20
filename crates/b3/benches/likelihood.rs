use criterion::{criterion_group, criterion_main, Criterion};
use rand::{rngs::SmallRng, seq::SliceRandom, SeedableRng};

use std::hint::black_box;

use b3::{
	distribution::Distribution,
	mcmc::{run, Config},
	operator::{
		scheduler::WeightedScheduler, NarrowExchange, Operator, Scale,
		Slide,
	},
	probability::Compound,
	state::State,
	tree::Tree,
};
use base::{seq::DnaSeq, DnaNucleoBase};

type Data = (Vec<DnaSeq>, Vec<f64>, Vec<usize>);

fn data(num_leaves_pow: usize, length: usize) -> Data {
	let num_leaves = 2_usize.pow(num_leaves_pow as u32);

	let mut rng = SmallRng::seed_from_u64(4);

	let bases = [
		DnaNucleoBase::Adenine,
		DnaNucleoBase::Cytosine,
		DnaNucleoBase::Guanine,
		DnaNucleoBase::Thymine,
	];
	let mut seqs: Vec<DnaSeq> = vec![];
	for _ in 0..num_leaves {
		let mut seq = DnaSeq::new();
		for _ in 0..length {
			let base = bases.choose(&mut rng).unwrap();
			seq.push(*base);
		}
		seqs.push(seq);
	}

	let weights = (0..(num_leaves * 2 - 1))
		.map(|e| e as f64 * 0.005)
		.collect();

	let mut children = vec![];
	let mut prev = 0;
	for level in 0..num_leaves_pow {
		let size =
			2_usize.pow(num_leaves_pow as u32 - 1 - level as u32);

		for i in 0..size {
			let left = prev + 2 * i;
			let right = prev + 2 * i + 1;
			children.push(left);
			children.push(right);
		}

		prev += size * 2;
	}

	(seqs, weights, children)
}

fn likelihood(data: &Data, length: usize) {
	let (seqs, weights, children) = data;
	let tree = Tree::new(weights, children);
	let mut state = State::new(tree, seqs);
	let prior = Box::new(Compound::new([]));

	// Local
	let exchange = NarrowExchange::new();
	let slide: Box<dyn Operator> = Slide::new(Distribution::Uniform);
	// Global
	let scale: Box<dyn Operator> = Scale::new(0.75, Distribution::Uniform);

	let mut scheduler = WeightedScheduler::new(
		[exchange, slide, scale],
		[50.0, 48.0, 2.0],
	);

	let config = Config {
		burnin: 0,
		length,
		state: 10,
		trees: (length / 10_000).max(1),
	};

	run(config, &mut state, prior, &mut scheduler);
}

fn bench(c: &mut Criterion) {
	let data = black_box(data(12, 1700));

	c.bench_function("likelihood", |b| b.iter(|| likelihood(&data, 1_000)));
}

criterion_group!(
	name = benches;
	config = Criterion::default().sample_size(10);
	targets = bench
);
criterion_main!(benches);
