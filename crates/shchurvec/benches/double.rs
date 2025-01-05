use criterion::{criterion_group, criterion_main, Criterion};
use rand::{rngs::SmallRng, Rng, SeedableRng};

use std::hint::black_box;

use shchurvec::ShchurVec;

type Row = [f64; 4];

fn data(length: usize) -> ShchurVec<Row> {
	let mut out = ShchurVec::with_capacity(length);
	let mut rng = SmallRng::seed_from_u64(4);

	for _ in 0..length {
		out.push(rng.gen());
	}

	out
}

fn edit(v: &mut ShchurVec<Row>) {
	let mut rng = SmallRng::seed_from_u64(4);
	let num = v.len() / 10;

	for _ in 0..num {
		let i = rng.gen_range(0..v.len());
		v.set(i, rng.gen());
	}

	v.accept();
}

fn bench(c: &mut Criterion) {
	let mut v = black_box(data(100_000));

	c.bench_function("edit_accept", |b| {
		b.iter(|| {
			edit(&mut v);
			v.accept()
		});
	});

	c.bench_function("edit_reject", |b| {
		b.iter(|| {
			edit(&mut v);
			v.reject()
		});
	});
}

criterion_group!(
	name = double;
	config = Criterion::default();
	targets = bench
);
criterion_main!(double);
