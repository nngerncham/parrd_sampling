use criterion::{criterion_group, criterion_main, Criterion};
use parrd_sampling::samplers::{
    naive_sampler::NaiveSampler, perm_sampler::PermutationSampler,
    priority_sampler::ParPrioritySampler, sampl_interface::Sampler,
};

pub fn bench_single_core(c: &mut Criterion) {
    rayon::ThreadPoolBuilder::new()
        .num_threads(32)
        .build_global()
        .unwrap();

    let sample_size = 100_000;
    let percentages = vec![0.1, 0.25, 0.5, 0.75, 0.9];
    let population: Vec<i32> = (0..sample_size).collect();

    for percentage in percentages {
        let k = (percentage * sample_size as f32).round() as usize;

        c.bench_function(
            &format!("MultiNaive{}", (percentage * 100f32) as usize),
            |b| b.iter(|| NaiveSampler::sample(&population, k)),
        );

        c.bench_function(
            &format!("MultiPriority{}", (percentage * 100f32) as usize),
            |b| b.iter(|| ParPrioritySampler::sample(&population, k)),
        );

        c.bench_function(
            &format!("MultiPermutation{}", (percentage * 100f32) as usize),
            |b| b.iter(|| PermutationSampler::sample(&population, k)),
        );
    }
}

criterion_group!(benches, bench_single_core);
criterion_main!(benches);
