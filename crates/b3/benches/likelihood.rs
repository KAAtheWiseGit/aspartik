use criterion::{criterion_group, criterion_main, Criterion};

use b3::{
	likelihood::GpuLikelihood,
	log,
	mcmc::{run, Config, DynLikelihood},
	model::DnaModel,
	operator::{
		scheduler::WeightedScheduler, Operator, TreeNarrowExchange,
		TreeScale, TreeSlide, TreeWideExchange,
	},
	probability::Compound,
	util, Distribution, State, Transitions,
};

fn likelihood(length: usize) {
	let (seqs, tree) = util::make_tree("data/100.fasta".as_ref());
	let model = Box::new(DnaModel::JukesCantor);

	let likelihood = Box::new(GpuLikelihood::new(util::dna_to_rows(&seqs)));
	let likelihoods: Vec<DynLikelihood<4>> = vec![likelihood];

	let num_edges = (seqs.len() - 1) * 2;
	let transitions = Transitions::<4>::new(num_edges);

	let mut state = State::new(tree);
	let prior = Box::new(Compound::new([]));

	// Local
	let narrow_exchange = TreeNarrowExchange::new();
	let wide_exchange = TreeWideExchange::new();
	let slide: Box<dyn Operator> = TreeSlide::new(Distribution::Uniform);
	// Global
	let scale: Box<dyn Operator> =
		TreeScale::new(0.75, Distribution::Uniform);

	let mut scheduler = WeightedScheduler::new(
		[narrow_exchange, wide_exchange, slide, scale],
		[25.0, 25.0, 48.0, 2.0],
	);

	let logger = log::Logger::new(1_000_000, None, vec![], vec![]);
	log::init(vec![logger]);

	let config = Config {
		burnin: 0,
		length,
		save_state_every: 5000,
	};

	run(
		config,
		&mut state,
		prior,
		&mut scheduler,
		likelihoods,
		transitions,
		model,
	)
	.unwrap();
}

fn bench(c: &mut Criterion) {
	c.bench_function("likelihood", |b| b.iter(|| likelihood(10_001)));
}

criterion_group!(
	name = benches;
	config = Criterion::default().sample_size(10);
	targets = bench
);
criterion_main!(benches);
