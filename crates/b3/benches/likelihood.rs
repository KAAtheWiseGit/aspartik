use criterion::{criterion_group, criterion_main, Criterion};

use b3::tree::*;
use base::seq::DnaSeq;

fn likelihood() {
	use base::substitution::dna::jukes_cantor;

	let seqs = vec![
		DnaSeq::try_from("AAGCT".repeat(10).as_ref()).unwrap(),
		DnaSeq::try_from("CAGCT".repeat(10).as_ref()).unwrap(),
		DnaSeq::try_from("ATGCA".repeat(10).as_ref()).unwrap(),
		DnaSeq::try_from("ATGCT".repeat(10).as_ref()).unwrap(),
		DnaSeq::try_from("TAGCA".repeat(10).as_ref()).unwrap(),
	];
	let tree = vec![(2, 3), (0, 1), (5, 4), (6, 7)];
	let distances = vec![0.75, 0.60, 1.1, 0.9, 0.85, 0.8, 0.5, 0.7, 0.3];

	let coalescent =
		Tree::new(seqs, jukes_cantor(), &distances, &tree);

	for _ in 0..1_000_000 {
		coalescent.likelihood_();
	}
}

fn bench(c: &mut Criterion) {
	c.bench_function("likelihood", |b| b.iter(|| likelihood()));
}

criterion_group!(
	name = benches;
	config = Criterion::default().sample_size(10);
	targets = bench
);
criterion_main!(benches);
