use rand::{rngs::ThreadRng, Rng};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::samplers::sampl_interface::Sampler;
use crate::utils::{cwslice::UnsafeSlice, prefix_scan::par_scan};
use core::hash::Hash;
use std::{cmp::Ordering, marker::PhantomData, usize};

fn quick_select<T: Clone + Hash + Sized>(
    xs: &[(T, u64)],
    k: usize,
    rng: &mut ThreadRng,
) -> (T, u64) {
    let n = xs.len();
    let pivot_idx = rng.gen_range(0..n);
    let pivot_priority = xs[pivot_idx].1;

    let leq_elements: Vec<(T, u64)> = xs
        .iter()
        .filter(|(_, x2)| x2 <= &pivot_priority)
        .cloned()
        .collect();

    match leq_elements.len().cmp(&k) {
        Ordering::Equal => xs[pivot_idx].clone(),
        Ordering::Greater => quick_select(&leq_elements, k, rng),
        _ => {
            let gt_elements: Vec<(T, u64)> = xs
                .iter()
                .filter(|(_, x2)| x2 <= &pivot_priority)
                .cloned()
                .collect();
            quick_select(&gt_elements, k - leq_elements.len(), rng)
        }
    }
}

fn par_quick_select<T: Clone + Hash + Sized + Send + Sync>(
    xs: &[(T, u64)],
    k: usize,
    rng: &mut ThreadRng,
) -> (T, u64) {
    let n = xs.len();
    let pivot_idx = rng.gen_range(0..n);
    let pivot_priority = xs[pivot_idx].1;

    let leq_flags: Vec<usize> = xs
        .par_iter()
        .map(|(x, priority)| match priority.cmp(&pivot_priority) {
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
            xs.par_iter().enumerate().for_each(|(i, (x, priority))| {
                if leq_flags[i] == 1 {
                    unsafe {
                        left_slice.write(left_locs[i], (x.clone(), *priority));
                    }
                }
            });

            par_quick_select(&left, k, rng)
        }
        Ordering::Less => {
            let gt_flags: Vec<usize> = xs
                .par_iter()
                .map(|(x, priority)| match priority.cmp(&pivot_priority) {
                    Ordering::Greater => 1,
                    _ => 0,
                })
                .collect();
            let (gt_count, gt_locs) = par_scan(&gt_flags);

            let mut right = vec![xs[0].clone(); gt_count];
            let right_slice = UnsafeSlice::new(&mut right);
            xs.par_iter().enumerate().for_each(|(i, (x, priority))| {
                if gt_flags[i] == 1 {
                    unsafe {
                        right_slice.write(gt_locs[i], (x.clone(), *priority));
                    }
                }
            });

            par_quick_select(&right, k - leq_count, rng)
        }
    }
}

pub struct SeqPrioritySampler<T: Clone + Hash + Sized + Send + Sync> {
    marker: PhantomData<T>,
}

impl<T: Clone + Hash + Sized + Send + Sync> Sampler<T> for SeqPrioritySampler<T> {
    fn sample(arr: &[T], k: usize) -> Option<Vec<T>> {
        match arr.len().cmp(&k) {
            Ordering::Less => return None,
            Ordering::Equal => return Some(arr.to_vec()),
            Ordering::Greater => {}
        }

        let mut rng = rand::thread_rng();
        let zipped: Vec<(T, u64)> = arr
            .iter()
            .cloned()
            .zip(arr.iter().map(|_| rng.gen::<u64>()))
            .collect();
        let kth_element = quick_select(&zipped, k, &mut rng);
        let kth_hash = fxhash::hash64(&kth_element);

        Some(
            arr.iter()
                .filter(|x| fxhash::hash64(x) <= kth_hash)
                .cloned()
                .collect(),
        )
    }
}

pub struct ParPrioritySampler<T: Clone + Hash + Sized + Send + Sync> {
    marker: PhantomData<T>,
}

impl<T: Clone + Hash + Sized + Send + Sync> Sampler<T> for ParPrioritySampler<T> {
    fn sample(arr: &[T], k: usize) -> Option<Vec<T>> {
        match arr.len().cmp(&k) {
            Ordering::Less => return None,
            Ordering::Equal => return Some(arr.to_vec()),
            Ordering::Greater => {}
        }

        let mut rng = rand::thread_rng();
        let xs: Vec<(T, u64)> = arr
            .par_iter()
            .cloned()
            .zip(
                arr.par_iter()
                    .map_init(rand::thread_rng, |rng, _| rng.gen::<u64>()),
            )
            .collect();
        let (_, pivot_priority) = par_quick_select(&xs, k, &mut rng);

        let leq_flags: Vec<usize> = xs
            .par_iter()
            .map(|(_, priority)| match priority.cmp(&pivot_priority) {
                Ordering::Less | Ordering::Equal => 1,
                _ => 0,
            })
            .collect();
        let (count, locs) = par_scan(&leq_flags);

        let mut samples = vec![xs[0].0.clone(); count];
        let samples_slice = UnsafeSlice::new(&mut samples);
        xs.par_iter().enumerate().for_each(|(i, (x, _))| {
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
        use super::ParPrioritySampler;
        use crate::samplers::sampl_interface::Sampler;

        let k = 50_000;
        let sample_size = 10_000_000;
        let population = (0..sample_size).collect::<Vec<i32>>();
        let samples = ParPrioritySampler::sample(&population, k);

        assert_eq!(k, samples.unwrap().len());
    }

    #[test]
    fn ps_rd_test_len() {
        use super::ParPrioritySampler;
        use crate::samplers::sampl_interface::Sampler;

        let k = 500_000;
        let sample_size = 10_000_000;
        let population = (0..sample_size)
            .map(|_| rand::random::<i32>())
            .collect::<Vec<i32>>();
        let samples = ParPrioritySampler::sample(&population, k);

        assert_eq!(k, samples.unwrap().len());
    }
}
