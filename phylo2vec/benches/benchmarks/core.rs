use criterion::{criterion_group, BenchmarkId, Criterion};
use phylo2vec::tree_vec::ops;
use phylo2vec::utils::sample_vector;
use std::ops::RangeInclusive;
use std::time::Duration;

const SAMPLE_SIZES: RangeInclusive<u32> = 1..=10;

/// Benchmark to_newick with both ordered and unordered inputs
fn bench_to_newick(c: &mut Criterion) {
    let mut group = c.benchmark_group("to_newick");
    // Set logarithmic scale for plot
    group.plot_config(
        criterion::PlotConfiguration::default().summary_scale(criterion::AxisScale::Logarithmic),
    );

    for i in SAMPLE_SIZES {
        let sample_size = 10000 * i as usize;

        // Benchmark ordered case
        group.bench_with_input(
            BenchmarkId::new("ordered", sample_size),
            &sample_size,
            |b, &size| {
                b.iter(|| {
                    let v = sample_vector(size, true);
                    ops::to_newick_from_vector(&v)
                });
            },
        );

        // Benchmark unordered case
        group.bench_with_input(
            BenchmarkId::new("unordered", sample_size),
            &sample_size,
            |b, &size| {
                b.iter(|| {
                    let v = sample_vector(size, false);
                    ops::to_newick_from_vector(&v)
                });
            },
        );
    }
    group.finish();
}

/// Benchmark to_vector
fn bench_to_vector(c: &mut Criterion) {
    let mut group = c.benchmark_group("to_vector");
    // Set logarithmic scale for plot
    group.plot_config(
        criterion::PlotConfiguration::default().summary_scale(criterion::AxisScale::Logarithmic),
    );
    for i in SAMPLE_SIZES {
        let sample_size = 10000 * i as usize;
        group.bench_with_input(
            BenchmarkId::from_parameter(sample_size),
            &sample_size,
            |b, &size| {
                // Generate the Newick string once outside the benchmark loop
                let v = sample_vector(size, true);
                let newick = ops::to_newick_from_vector(&v);

                // Benchmark only the to_vector operation
                b.iter(|| ops::to_vector(&newick));
            },
        );
    }
    group.finish();
}

criterion_group! {
    name = core;
    config = Criterion::default()
        .sample_size(10)
        .warm_up_time(Duration::from_millis(1000));
    targets = bench_to_newick, bench_to_vector
}
