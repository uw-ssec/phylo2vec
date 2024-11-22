use criterion::criterion_main;

mod benchmarks;

criterion_main!(
	benchmarks::get_pairs::get_pairs,
	benchmarks::get_ancestry_dtype::get_ancestry_datatypes
);
