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

fn generate_swaps(n: usize) -> Vec<usize> {
    (0..n) // H in the J. Shun paper
        .into_par_iter()
        .map_init(rand::thread_rng, |rng, i| rng.gen_range(i..n))
        .collect::<Vec<usize>>()
}

#[allow(dead_code)]
fn knuth_shuffle<T: Clone + Sized>(arr: &[T], k: usize, swap_targets: &[usize]) -> Vec<T> {
    let mut ans = arr.to_vec();
    swap_targets
        .iter()
        .take(k)
        .enumerate()
        .for_each(|(i, &target)| ans.swap(i, target));

    ans[..k].to_vec()
}

fn par_permute_k<T: Clone + Sized + Send + Sync>(
    arr: &[T],
    k: usize,
    swap_targets: &[usize],
) -> Vec<T> {
    let n = arr.len();

    let reservation: Vec<AtomicUsize> = (0..n)
        .into_par_iter()
        .map(|_| AtomicUsize::new(n))
        .collect(); // init'd as -1 in paper but I can't see the point
    let reserve = |i: usize| {
        reservation[i].fetch_min(i, AtomicOrdering::Relaxed);
        reservation[swap_targets[i]].fetch_min(i, AtomicOrdering::Relaxed);
    };

    let mut ans = arr.to_vec();
    let ans_slice = UnsafeSlice::new(&mut ans);
    let commit = |i: usize| -> usize {
        let swap_idx = swap_targets[i];
        unsafe {
            if reservation[i].load(AtomicOrdering::Relaxed) == i
                && reservation[swap_idx].load(AtomicOrdering::Relaxed) == i
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
        let mut new_idx_remaining =
            vec![0usize; idx_remaining.len() - (pack_locs.len() - failed_count)];
        let new_idx_remaining_slice = UnsafeSlice::new(&mut new_idx_remaining);
        idx_remaining
            .par_iter()
            .enumerate()
            .take(prefix_size)
            .for_each(|(i, &idx)| {
                // reservation[idx].store(n, AtomicOrdering::Relaxed);
                reservation[swap_targets[idx]].store(n, AtomicOrdering::Relaxed);
                if fail_commits[i] == 1 {
                    unsafe {
                        new_idx_remaining_slice.write(pack_locs[i], idx);
                    }
                }
            });
        if prefix_size <= idx_remaining.len() {
            idx_remaining[prefix_size..]
                .par_iter()
                .enumerate()
                .for_each(|(i, &idx)| {
                    reservation[swap_targets[idx]].store(n, AtomicOrdering::Relaxed);
                    unsafe {
                        new_idx_remaining_slice.write(failed_count + i, idx);
                    }
                });
        }

        idx_remaining = new_idx_remaining;
        prefix_size = (idx_remaining.len() / PREFIX_DIVISOR).max(PREFIX_DIVISOR);
    }

    ans[..k].to_vec()
}

pub struct SeqPermutationSampler<T: Clone + Sized> {
    marker: PhantomData<T>,
}

impl<T: Clone + Sized> Sampler<T> for SeqPermutationSampler<T> {
    fn sample(arr: &[T], k: usize) -> Option<Vec<T>> {
        let swap_targets = generate_swaps(arr.len());
        Some(knuth_shuffle(arr, k, &swap_targets))
    }
}

pub struct FullPermutationSampler<T: Clone + Sized + Send + Sync> {
    marker: PhantomData<T>,
}

impl<T: Clone + Sized + Send + Sync> Sampler<T> for FullPermutationSampler<T> {
    fn sample(arr: &[T], k: usize) -> Option<Vec<T>> {
        let n = arr.len();
        let swap_targets = generate_swaps(n);
        Some(par_permute_k(arr, n, &swap_targets)[..k].to_vec())
    }
}

pub struct PermutationSampler<T: Clone + Sized + Send + Sync> {
    marker: PhantomData<T>,
}

impl<T: Clone + Sized + Send + Sync> Sampler<T> for PermutationSampler<T> {
    fn sample(arr: &[T], k: usize) -> Option<Vec<T>> {
        let n = arr.len();
        let swap_targets = generate_swaps(n);
        Some(par_permute_k(arr, k, &swap_targets))
    }
}

mod test {
    #[allow(dead_code)]
    fn seq_par_perm_eq_test(n: usize, k: usize) {
        use rand::thread_rng;
        use rand::Rng;

        let mut rng = thread_rng();
        let swap_targets: Vec<usize> = (0..n).map(|i| rng.gen_range(i..n)).collect();
        let xs: Vec<usize> = (0..n).collect();

        let seq_result = super::knuth_shuffle(&xs, k, &swap_targets);
        let par_result = super::par_permute_k(&xs, k, &swap_targets);

        assert_eq!(&seq_result, &par_result);
    }

    #[test]
    fn perm_par_is_seq_small_early() {
        seq_par_perm_eq_test(20, 10);
    }

    #[test]
    fn perm_par_is_seq_small_full() {
        seq_par_perm_eq_test(20, 20);
    }

    #[test]
    fn perm_par_is_seq_early() {
        let n = 10_000_000;
        let k = 500_000;
        seq_par_perm_eq_test(n, k);
    }

    #[test]
    fn perm_par_is_seq_full() {
        let n = 10_000_000;
        seq_par_perm_eq_test(n, n);
    }
}
