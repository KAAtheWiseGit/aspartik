use criterion::{criterion_group, criterion_main, Criterion};

use std::{fs::File, hint::black_box};

use b3::{
	mcmc::{run, Config},
	operator::{
		scheduler::WeightedScheduler, NarrowExchange, Operator, Scale,
		Slide, WideExchange,
	},
	probability::Compound,
	Distribution, State, Tree,
};
use base::{seq::DnaSeq, DnaNucleoBase};
use io::fasta::FastaReader;

type Data = (Vec<DnaSeq>, Vec<f64>, Vec<usize>);

fn data(num_leaves_pow: usize) -> Data {
	let num_leaves = 2_usize.pow(num_leaves_pow as u32);

	let fasta: FastaReader<DnaNucleoBase, _> =
		FastaReader::new(File::open("data/test.fasta").unwrap());
	let seqs: Vec<DnaSeq> =
		fasta.take(num_leaves).map(|s| s.unwrap().into()).collect();

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
	let narrow_exchange = NarrowExchange::new();
	let wide_exchange = WideExchange::new();
	let slide: Box<dyn Operator> = Slide::new(Distribution::Uniform);
	// Global
	let scale: Box<dyn Operator> = Scale::new(0.75, Distribution::Uniform);

	let mut scheduler = WeightedScheduler::new(
		[narrow_exchange, wide_exchange, slide, scale],
		[25.0, 25.0, 48.0, 2.0],
	);

	let config = Config {
		burnin: 0,
		length,
		save_state_every: 5000,
		loggers: vec![],
	};

	run(config, &mut state, prior, &mut scheduler);
}

fn bench(c: &mut Criterion) {
	let data = black_box(data(9));

	c.bench_function("likelihood", |b| {
		b.iter(|| likelihood(&data, 10_001))
	});
}

criterion_group!(
	name = benches;
	config = Criterion::default().sample_size(10);
	targets = bench
);
criterion_main!(benches);
