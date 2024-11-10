def target_dir [] {
	cargo metadata --format-version 1 | from json | get target_directory
}

export def "b3 flame" [] {
	let dst = target_dir | path join "flamegraph/likelihood.svg"
	mkdir ($dst | path dirname)

	(
		cargo flamegraph
			--package b3
			--bench likelihood
			--output $dst
			--palette rust
			--skip-after likelihood::likelihood
	)
	rm --permanent --force **/perf.data*
}
