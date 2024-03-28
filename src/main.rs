mod samplers;
mod utils;

use rand::{thread_rng, Rng};
use samplers::perm_sampler::{knuth_shuffle, par_permute_k};

fn main() {
    let n = 20;
    let k = 5;
    let mut rng = thread_rng();
    let swap_targets: Vec<usize> = (0..n).map(|i| rng.gen_range(i..n)).collect();
    let xs: Vec<usize> = (0..n).collect();

    // println!("{:?}", &swap_targets);
    // println!("{:?}", &range);

    let seq_result = knuth_shuffle(&xs, k, &swap_targets);
    let par_result = par_permute_k(&xs, k, &swap_targets).unwrap();

    println!(
        "{:?}",
        &(seq_result
            .iter()
            .zip(&par_result)
            .map(|(&a, &b)| (a, b))
            .collect::<Vec<(usize, usize)>>())
    );
}
