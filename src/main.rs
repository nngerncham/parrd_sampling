use csv::Writer;
use parrd_sampling::utils::my_bencher::benchmark_core;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args[1].eq("single") {
        let mut csv_result_wtr = Writer::from_path("analysis/results/single_core_result.csv")?;
        csv_result_wtr.write_record(["algorithm", "threads", "n", "k", "rep", "dtype", "time"])?;
        let _ = benchmark_core(&mut csv_result_wtr, 1);
    } else if args[1] == "multi" {
        let mut csv_result_wtr = Writer::from_path("analysis/results/multi_core_result.csv")?;
        csv_result_wtr.write_record(["algorithm", "threads", "n", "k", "rep", "dtype", "time"])?;
        let _ = benchmark_core(&mut csv_result_wtr, 12);
    } else if args[1] == "multiht" {
        let mut csv_result_wtr = Writer::from_path("analysis/results/multiht_core_result.csv")?;
        csv_result_wtr.write_record(["algorithm", "threads", "n", "k", "rep", "dtype", "time"])?;
        let _ = benchmark_core(&mut csv_result_wtr, 24);
    }

    Ok(())
}
