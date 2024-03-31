use csv::Result;

use crate::samplers::{
    naive_sampler::NaiveSampler,
    perm_sampler::{FullPermutationSampler, PermutationSampler, SeqPermutationSampler},
    priority_sampler::{ParPrioritySampler, SeqPrioritySampler},
    sampl_interface::Sampler,
};

const REPEATS: usize = 3;
const PROBLEM_SIZE: usize = 50_000_000;

pub fn benchmark_core(wtr: &mut csv::Writer<std::fs::File>, core_count: usize) -> Result<()> {
    let bench_start = std::time::Instant::now();
    rayon::ThreadPoolBuilder::new()
        .num_threads(core_count)
        .build_global()
        .unwrap();

    // run the benchmark
    let ks: Vec<usize> = [0.1, 0.25, 0.5, 0.75, 0.9]
        .iter()
        .map(|k| (k * PROBLEM_SIZE as f64) as usize)
        .collect();

    for k in ks {
        let data = (0..PROBLEM_SIZE).map(|i| i as i32).collect::<Vec<i32>>();
        println!("Benchmarking with k = {}", k);

        for repeat in 0..REPEATS {
            println!("Naive {}", repeat + 1);
            let start = std::time::Instant::now();
            let _ = NaiveSampler::sample(&data, k);
            let end = std::time::Instant::now().duration_since(start);
            wtr.write_record([
                "Naive",
                &core_count.to_string(),
                &PROBLEM_SIZE.to_string(),
                &k.to_string(),
                &repeat.to_string(),
                "i32",
                &end.as_micros().to_string(),
            ])?;

            println!("SeqPriority {}", repeat + 1);
            let start = std::time::Instant::now();
            let _ = SeqPrioritySampler::sample(&data, k);
            let end = std::time::Instant::now().duration_since(start);
            wtr.write_record([
                "SeqPriority",
                &core_count.to_string(),
                &PROBLEM_SIZE.to_string(),
                &k.to_string(),
                &repeat.to_string(),
                "i32",
                &end.as_micros().to_string(),
            ])?;

            println!("ParPriority {}", repeat + 1);
            let start = std::time::Instant::now();
            let _ = ParPrioritySampler::sample(&data, k);
            let end = std::time::Instant::now().duration_since(start);
            wtr.write_record([
                "ParPriority",
                &core_count.to_string(),
                &PROBLEM_SIZE.to_string(),
                &k.to_string(),
                &repeat.to_string(),
                "i32",
                &end.as_micros().to_string(),
            ])?;

            println!("SeqPermutation {}", repeat + 1);
            let start = std::time::Instant::now();
            let _ = SeqPermutationSampler::sample(&data, k);
            let end = std::time::Instant::now().duration_since(start);
            wtr.write_record([
                "SeqPermutation",
                &core_count.to_string(),
                &PROBLEM_SIZE.to_string(),
                &k.to_string(),
                &repeat.to_string(),
                "i32",
                &end.as_micros().to_string(),
            ])?;

            println!("FullPermutation {}", repeat + 1);
            let start = std::time::Instant::now();
            let _ = FullPermutationSampler::sample(&data, k);
            let end = std::time::Instant::now().duration_since(start);
            wtr.write_record([
                "FullPermutation",
                &core_count.to_string(),
                &PROBLEM_SIZE.to_string(),
                &k.to_string(),
                &repeat.to_string(),
                "i32",
                &end.as_micros().to_string(),
            ])?;

            println!("ParPermutation {}", repeat + 1);
            let start = std::time::Instant::now();
            let _ = PermutationSampler::sample(&data, k);
            let end = std::time::Instant::now().duration_since(start);
            wtr.write_record([
                "ParPermutation",
                &core_count.to_string(),
                &PROBLEM_SIZE.to_string(),
                &k.to_string(),
                &repeat.to_string(),
                "i32",
                &end.as_micros().to_string(),
            ])?;
        }
    }

    let bench_end = std::time::Instant::now().duration_since(bench_start);
    println!("Benchmark took a total of {} seconds", bench_end.as_secs());

    Ok(())
}
