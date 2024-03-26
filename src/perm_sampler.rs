use common_traits::Hash;
use rand::Rng;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::marker::PhantomData;

use crate::{cwslice::UnsafeSlice, models::Sampler};

struct PermutationSampler<T: Clone + Hash + Sized + Send + Sync> {
    marker: PhantomData<T>,
}

fn generate_swaps(n: usize) -> Vec<usize> {
    (0..n) // H in the J. Shun paper
        .into_par_iter()
        .map_init(rand::thread_rng, |rng, i| rng.gen_range(i..n))
        .collect::<Vec<usize>>()
}

impl<T: Clone + Hash + Sized + Send + Sync> Sampler<T> for PermutationSampler<T> {
    fn sample(arr: &[T], k: usize) -> Option<Vec<T>> {
        let n = arr.len();
        let swap_targets = generate_swaps(n);

        let mut reservation = vec![0usize; n]; // init'd as -1 in paper but I can't see the point
        let resv_slice = UnsafeSlice::new(&mut reservation);

        None
    }
}

mod test {
    #[test]
    fn swap_generation() {
        let n = 1_000_000;
        let swap_targets = super::generate_swaps(n);
        assert!(swap_targets
            .iter()
            .enumerate()
            .all(|(i, &target)| i <= target && target < n));
    }
}
