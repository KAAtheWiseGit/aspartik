def target_dir [] {
	cargo metadata --format-version 1 | from json | get target_directory
}

export def flame [
	package: string
	bench: string
] {
	let dst = target_dir | path join "flamegraph/likelihood.svg"
	mkdir ($dst | path dirname)

	(
		cargo flamegraph
			--package $package
			--bench $bench
			--output $dst
			--palette rust
			--skip-after "criterion::bencher::Bencher<M>::iter"
	)

	rm --permanent --force **/perf.data*
}
