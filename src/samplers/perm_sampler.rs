use common_traits::Hash;
use rand::Rng;
use rayon::iter::{
    IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator, ParallelIterator,
};
use std::{
    marker::PhantomData,
    sync::atomic::{AtomicUsize, Ordering as AtomicOrdering},
};

use crate::{
    samplers::sampl_interface::Sampler,
    utils::{cwslice::UnsafeSlice, prefix_scan::par_scan},
};

const PREFIX_DIVISOR: usize = 100;

#[allow(dead_code)]
pub fn knuth_shuffle(arr: &[usize], k: usize, swap_targets: &[usize]) -> Vec<usize> {
    let mut ans = arr.to_vec();
    swap_targets
        .iter()
        .take(k)
        .enumerate()
        .for_each(|(i, &target)| ans.swap(i, target));

    ans[..k].to_vec()
}

pub struct PermutationSampler<T: Clone + Sized + Send + Sync> {
    marker: PhantomData<T>,
}

fn generate_swaps(n: usize) -> Vec<usize> {
    (0..n) // H in the J. Shun paper
        .into_par_iter()
        .map_init(rand::thread_rng, |rng, i| rng.gen_range(i..n))
        .collect::<Vec<usize>>()
}

pub fn par_permute_k<T: Clone + Sized + Send + Sync>(
    arr: &[T],
    k: usize,
    swap_targets: &[usize],
) -> Option<Vec<T>> {
    let n = arr.len();

    let reservation: Vec<AtomicUsize> = (0..n)
        .into_par_iter()
        .map(|_| AtomicUsize::new(n))
        .collect(); // init'd as -1 in paper but I can't see the point
    let reserve = |i: usize| {
        reservation[i].fetch_min(i, AtomicOrdering::AcqRel);
        reservation[swap_targets[i]].fetch_min(i, AtomicOrdering::AcqRel);
    };

    let mut ans = arr.to_vec();
    let ans_slice = UnsafeSlice::new(&mut ans);
    let commit = |i: usize| -> usize {
        let swap_idx = swap_targets[i];
        unsafe {
            if reservation[i].load(AtomicOrdering::Acquire) == i
                && reservation[swap_idx].load(AtomicOrdering::Acquire) == i
            {
                ans_slice.swap(i, swap_idx);
                0
            } else {
                1
            }
        }
    };

    let mut idx_remaining = (0..k).collect::<Vec<usize>>();
    let mut prefix_size = (idx_remaining.len() / PREFIX_DIVISOR).max(PREFIX_DIVISOR);
    // max btw PREFIX_DIVISOR so if prefix_size < PREFIX_DIVISOR then it =/> 0

    while !idx_remaining.is_empty() {
        // do reserve and commit
        idx_remaining.par_iter().take(prefix_size).for_each(|&idx| {
            reserve(idx);
        });
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
            .for_each(|(i, &idx)| {
                reservation[idx].store(n, AtomicOrdering::Release);
                reservation[swap_targets[idx]].store(n, AtomicOrdering::Release);
                if fail_commits[i] == 1 {
                    unsafe {
                        new_idx_remaining_slice.write(pack_locs[i], idx);
                    }
                }
            });

        // new # successful += # processed - # failed
        idx_remaining = new_idx_remaining;
        prefix_size = (idx_remaining.len() / PREFIX_DIVISOR).max(PREFIX_DIVISOR);
        // println!("{:?}", &swap_targets);
        // println!("{:?}", &reservation);
        // println!("{:?}", &idx_remaining);
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
        let k = 10;
        let mut rng = thread_rng();
        let swap_targets: Vec<usize> = (0..n).map(|i| rng.gen_range(i..n)).collect();
        let xs: Vec<usize> = (0..n).collect();

        let seq_result = super::knuth_shuffle(&xs, k, &swap_targets);
        let par_result = super::par_permute_k(&xs, k, &swap_targets).unwrap();

        println!("{:?}", &swap_targets);
        // println!(
        //     "{:?}",
        //     &(seq_result
        //         .iter()
        //         .zip(&par_result)
        //         .map(|(&a, &b)| (a, b))
        //         .collect::<Vec<(usize, usize)>>())
        // );

        assert_eq!(&seq_result, &par_result);
    }
}
