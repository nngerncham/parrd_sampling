use rand::{rngs::ThreadRng, Rng};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::samplers::sampl_interface::Sampler;
use crate::utils::{cwslice::UnsafeSlice, prefix_scan::par_scan};
use core::hash::Hash;
use std::{cmp::Ordering, marker::PhantomData, usize};

struct PrioritySampler<T: Clone + Hash + Sized + Send + Sync> {
    marker: PhantomData<T>,
}

fn par_quick_select<T: Clone + Hash + Sized + Send + Sync>(
    xs: &[T],
    k: usize,
    rng: &mut ThreadRng,
) -> T {
    let n = xs.len();
    let pivot_idx = rng.gen_range(0..n);
    let pivot_hash = fxhash::hash64(&xs[pivot_idx]);

    let leq_flags: Vec<usize> = xs
        .par_iter()
        .map(|x: &T| match fxhash::hash64(&x).cmp(&pivot_hash) {
            Ordering::Less | Ordering::Equal => 1,
            _ => 0,
        })
        .collect();
    let (leq_count, left_locs) = par_scan(&leq_flags);

    match leq_count.cmp(&k) {
        Ordering::Equal => xs[pivot_idx].clone(),
        Ordering::Greater => {
            let mut left = vec![xs[0].clone(); leq_count];
            let left_slice = UnsafeSlice::new(&mut left);
            xs.par_iter().enumerate().for_each(|(i, x): (usize, &T)| {
                if leq_flags[i] == 1 {
                    unsafe {
                        left_slice.write(left_locs[i], x.clone());
                    }
                }
            });

            par_quick_select(&left, k, rng)
        }
        Ordering::Less => {
            let gt_flags: Vec<usize> = xs
                .par_iter()
                .map(|x: &T| match fxhash::hash64(&x).cmp(&pivot_hash) {
                    Ordering::Greater => 1,
                    _ => 0,
                })
                .collect();
            let (gt_count, gt_locs) = par_scan(&gt_flags);

            let mut right = vec![xs[0].clone(); gt_count];
            let right_slice = UnsafeSlice::new(&mut right);
            xs.par_iter().enumerate().for_each(|(i, x): (usize, &T)| {
                if gt_flags[i] == 1 {
                    unsafe {
                        right_slice.write(gt_locs[i], x.clone());
                    }
                }
            });

            par_quick_select(&right, k - leq_count, rng)
        }
    }
}

impl<T: Clone + Hash + Sized + Send + Sync> Sampler<T> for PrioritySampler<T> {
    fn sample(arr: &[T], k: usize) -> Option<Vec<T>> {
        match arr.len().cmp(&k) {
            Ordering::Less => return None,
            Ordering::Equal => return Some(arr.to_vec()),
            Ordering::Greater => {}
        }

        let mut rng = rand::thread_rng();
        let xs = arr.to_vec();
        let kth_element = par_quick_select(&xs, k, &mut rng);
        let kth_hash = fxhash::hash64(&kth_element);
        let leq_flags: Vec<usize> = xs
            .par_iter()
            .map(|x: &T| match fxhash::hash64(&x).cmp(&kth_hash) {
                Ordering::Less | Ordering::Equal => 1,
                _ => 0,
            })
            .collect();
        let (count, locs) = par_scan(&leq_flags);

        let mut samples = vec![xs[0].clone(); count];
        let samples_slice = UnsafeSlice::new(&mut samples);
        xs.par_iter().enumerate().for_each(|(i, x): (usize, &T)| {
            if leq_flags[i] == 1 {
                unsafe {
                    samples_slice.write(locs[i], x.clone());
                }
            }
        });

        Some(samples)
    }
}

mod test {
    #[test]
    fn ps_test_len() {
        use super::PrioritySampler;
        use crate::samplers::sampl_interface::Sampler;

        let k = 50_000;
        let sample_size = 10_000_000;
        let population = (0..sample_size).collect::<Vec<i32>>();
        let samples = PrioritySampler::sample(&population, k);

        assert_eq!(k, samples.unwrap().len());
    }

    #[test]
    fn ps_rd_test_len() {
        use super::PrioritySampler;
        use crate::samplers::sampl_interface::Sampler;

        let k = 500_000;
        let sample_size = 10_000_000;
        let population = (0..sample_size)
            .map(|_| rand::random::<i32>())
            .collect::<Vec<i32>>();
        let samples = PrioritySampler::sample(&population, k);

        assert_eq!(k, samples.unwrap().len());
    }
}
