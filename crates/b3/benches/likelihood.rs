use criterion::{criterion_group, criterion_main, Criterion};
use rand::{rngs::SmallRng, seq::SliceRandom, SeedableRng};

use b3::{
	mcmc::{run, Config},
	operator::{scheduler::TurnScheduler, NarrowExchange, Operator},
	probability::Compound,
	state::State,
	tree::Tree,
};
use base::{seq::DnaSeq, DnaNucleoBase};

fn data() -> (Vec<DnaSeq>, Vec<f64>, Vec<usize>) {
	let mut rng = SmallRng::seed_from_u64(4);

	let bases = [
		DnaNucleoBase::Adenine,
		DnaNucleoBase::Cytosine,
		DnaNucleoBase::Guanine,
		DnaNucleoBase::Thymine,
	];
	let mut seqs: Vec<DnaSeq> = vec![];
	for _ in 0..512 {
		let mut seq = DnaSeq::new();
		for _ in 0..1024 {
			let base = bases.choose(&mut rng).unwrap();
			seq.push(*base);
		}
		seqs.push(seq);
	}

	let weights = (0..1023).map(|e| e as f64 * 0.005).collect();

	let mut children = vec![];
	let mut prev = 0;
	for level in 0..=8 {
		let size = 2_usize.pow(8 - level);

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

fn likelihood(seqs: &[DnaSeq], weights: &[f64], children: &[usize]) {
	let tree = Tree::new(seqs, weights, children);
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
	let data = data();

	c.bench_function("likelihood", |b| {
		b.iter(|| likelihood(&data.0, &data.1, &data.2))
	});
}

criterion_group!(
	name = benches;
	config = Criterion::default().sample_size(10);
	targets = bench
);
criterion_main!(benches);
