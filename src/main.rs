use csv::Writer;
use parrd_sampling::utils::my_bencher::benchmark_core;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // let mut csv_result_wtr = Writer::from_path("target/single_core_result.csv")?;
    // csv_result_wtr.write_record(["algorithm", "threads", "n", "k", "rep", "dtype", "time(mu)"])?;
    // let _ = benchmark_core(&mut csv_result_wtr, 1);

    let mut csv_result_wtr = Writer::from_path("analysis/results/multi_core_result.csv")?;
    csv_result_wtr.write_record(["algorithm", "threads", "n", "k", "rep", "dtype", "time(mu)"])?;
    let _ = benchmark_core(&mut csv_result_wtr, 12);

    // let mut csv_result_wtr = Writer::from_path("target/multiht_core_result.csv")?;
    // csv_result_wtr.write_record(["algorithm", "threads", "n", "k", "rep", "dtype", "time(mu)"])?;
    // let _ = benchmark_core(&mut csv_result_wtr, 24);

    Ok(())
}
