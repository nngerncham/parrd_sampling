use common_traits::Hash;
use rand::Rng;
use rayon::iter::{
    IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator, ParallelIterator,
};
use std::marker::PhantomData;

use crate::{
    samplers::sampl_interface::Sampler,
    utils::{cwslice::UnsafeSlice, prefix_scan::par_scan},
};

const PREFIX_DIVISOR: usize = 50;

struct PermutationSampler<T: Clone + Sized + Send + Sync> {
    marker: PhantomData<T>,
}

fn generate_swaps(n: usize) -> Vec<usize> {
    (0..n) // H in the J. Shun paper
        .into_par_iter()
        .map_init(rand::thread_rng, |rng, i| rng.gen_range(i..n))
        .collect::<Vec<usize>>()
}

fn par_permute_k<T: Clone + Sized + Send + Sync>(
    arr: &[T],
    k: usize,
    swap_targets: &[usize],
) -> Option<Vec<T>> {
    let n = arr.len();

    let mut reservation = vec![0usize; n]; // init'd as -1 in paper but I can't see the point
    let resv_slice = UnsafeSlice::new(&mut reservation);

    let reserve = |i: usize| unsafe {
        resv_slice.write_max(i, i);
        resv_slice.write_max(swap_targets[i], i);
    };

    let mut ans = arr.to_vec();
    let ans_slice = UnsafeSlice::new(&mut ans);
    let commit = |i: usize| -> usize {
        unsafe {
            if resv_slice.read(i) == &i && resv_slice.read(swap_targets[i]) == &i {
                ans_slice.swap(i, swap_targets[i]);
                0
            } else {
                1
            }
        }
    };

    let mut swapped_count = 0;
    let mut idx_remaining = (0..n).collect::<Vec<usize>>();
    let mut prefix_size = ((n - swapped_count) / PREFIX_DIVISOR).max(PREFIX_DIVISOR);
    // maxed with PREFIX_DIVISOR so if prefix_size < PREFIX_DIVISOR then it doesn't become 0

    while swapped_count < k {
        // do reserve and commit
        idx_remaining
            .par_iter()
            .take(prefix_size)
            .for_each(|&idx| reserve(idx));
        let fail_commits: Vec<usize> = idx_remaining
            .par_iter()
            .take(prefix_size)
            .map(|&idx| commit(idx))
            .collect();

        // pack things together for next round
        let (failed_count, pack_locs) = par_scan(&fail_commits);
        let mut new_idx_remaining = vec![0usize; failed_count];
        let new_idx_remaining_slice = UnsafeSlice::new(&mut new_idx_remaining);
        idx_remaining
            .par_iter()
            .enumerate()
            .take(prefix_size)
            .for_each(|(i, &idx): (usize, &usize)| {
                if fail_commits[i] == 1 {
                    unsafe {
                        new_idx_remaining_slice.write(pack_locs[i], idx);
                    }
                }
            });

        // # processed - # failed = # successful
        swapped_count += fail_commits.len() - failed_count; 
        prefix_size = ((n - swapped_count) / PREFIX_DIVISOR).max(PREFIX_DIVISOR);
        idx_remaining = new_idx_remaining;
    }

    Some(ans[..k].to_vec())
}

impl<T: Clone + Hash + Sized + Send + Sync> Sampler<T> for PermutationSampler<T> {
    fn sample(arr: &[T], k: usize) -> Option<Vec<T>> {
        let n = arr.len();
        let swap_targets = generate_swaps(n);
        par_permute_k(arr, k, &swap_targets)
    }
}

mod test {
    #[allow(dead_code)]
    fn knuth_shuffle(arr: &[usize], k: usize, swap_targets: &[usize]) -> Vec<usize> {
        let mut ans = arr.to_vec();
        swap_targets
            .iter()
            .take(k)
            .enumerate()
            .for_each(|(i, &target)| ans.swap(i, target));

        ans[..k].to_vec()
    }

    #[test]
    fn swap_generation() {
        let n = 1_000_000;
        let swap_targets = super::generate_swaps(n);
        assert!(swap_targets
            .iter()
            .enumerate()
            .all(|(i, &target)| i <= target && target < n));
    }

    #[test]
    fn perm_par_is_seq_small() {
        use rand::thread_rng;
        use rand::Rng;

        let n = 20;
        let k = 5;
        let mut rng = thread_rng();
        let swap_targets: Vec<usize> = (0..n).map(|i| rng.gen_range(i..n)).collect();
        let xs: Vec<usize> = (0..n).collect();

        // println!("{:?}", &swap_targets);
        // println!("{:?}", &range);

        let seq_result = knuth_shuffle(&xs, k, &swap_targets);
        let par_result = super::par_permute_k(&xs, k, &swap_targets).unwrap();

        println!("{:?}", &(seq_result.iter().zip(&par_result)));

        assert_eq!(&seq_result, &par_result);
    }
}
