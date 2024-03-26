use rand::{rngs::ThreadRng, Rng};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::cwslice::UnsafeSlice;
use crate::models::Sampler;
use core::hash::Hash;
use std::{cmp::Ordering, marker::PhantomData, usize};

struct PrioritySampler<T: Clone + Hash + Sized + Send + Sync> {
    marker: PhantomData<T>,
}

fn par_scan_up<'a>(xs: &'a [i32], aux: &UnsafeSlice<'a, i32>, aux_offset: usize) -> i32 {
    if xs.len() == 1 {
        xs[0]
    } else {
        let m = xs.len() / 2;
        let (xs_left, xs_right) = xs.split_at(m);
        let (left, right) = rayon::join(
            || par_scan_up(xs_left, aux, aux_offset),
            || par_scan_up(xs_right, aux, aux_offset + m),
        );
        unsafe {
            aux.write(aux_offset + m - 1, left);
        }
        left + right
    }
}

fn par_scan_down(
    aux: &[i32],
    res: &UnsafeSlice<'_, i32>,
    ps: i32,
    res_offset: usize,
    res_size: usize,
) {
    if res_size == 1 {
        unsafe {
            res.write(res_offset, ps);
        }
    } else {
        let m = res_size / 2;
        let (aux_left, aux_right) = aux.split_at(m);
        rayon::join(
            || par_scan_down(aux_left, res, ps, res_offset, m),
            || {
                par_scan_down(
                    aux_right,
                    res,
                    ps + aux[m - 1],
                    res_offset + m,
                    res_size - m,
                )
            },
        );
    }
}

fn par_scan(xs: &[i32]) -> (i32, Vec<i32>) {
    let mut ell = vec![0; xs.len() - 1];
    let ell_slice = UnsafeSlice::new(&mut ell);
    let mut res = vec![0; xs.len()];
    let res_slice = UnsafeSlice::new(&mut res);

    let total = par_scan_up(xs, &ell_slice, 0);
    par_scan_down(&ell, &res_slice, 0, 0, xs.len());
    (total, res)
}

fn par_quick_select<'a, T: Clone + Hash + Sized + Send + Sync>(
    xs: &[T],
    k: usize,
    rng: &mut ThreadRng,
) -> T {
    let n = xs.len();
    let pivot_idx = rng.gen_range(0..n);
    let pivot_hash = fxhash::hash64(&xs[pivot_idx]);

    let leq_flags: Vec<i32> = xs
        .par_iter()
        .map(|x: &T| match fxhash::hash64(&x).cmp(&pivot_hash) {
            Ordering::Less | Ordering::Equal => 1,
            _ => 0,
        })
        .collect();
    let (leq_count, left_locs) = par_scan(&leq_flags);

    match leq_count.cmp(&(k as i32)) {
        Ordering::Equal => xs[pivot_idx].clone(),
        Ordering::Greater => {
            let mut left = vec![xs[0].clone(); leq_count as usize];
            let left_slice = UnsafeSlice::new(&mut left);
            xs.par_iter().enumerate().for_each(|(i, x): (usize, &T)| {
                if leq_flags[i] == 1 {
                    unsafe {
                        left_slice.write(left_locs[i] as usize, x.clone());
                    }
                }
            });

            par_quick_select(&left, k, rng)
        }
        Ordering::Less => {
            let gt_flags: Vec<i32> = xs
                .par_iter()
                .map(|x: &T| match fxhash::hash64(&x).cmp(&pivot_hash) {
                    Ordering::Greater => 1,
                    _ => 0,
                })
                .collect();
            let (gt_count, gt_locs) = par_scan(&gt_flags);

            let mut right = vec![xs[0].clone(); gt_count as usize];
            let right_slice = UnsafeSlice::new(&mut right);
            xs.par_iter().enumerate().for_each(|(i, x): (usize, &T)| {
                if gt_flags[i] == 1 {
                    unsafe {
                        right_slice.write(gt_locs[i] as usize, x.clone());
                    }
                }
            });

            par_quick_select(&right, k - leq_count as usize, rng)
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
        let leq_flags: Vec<i32> = xs
            .par_iter()
            .map(|x: &T| match fxhash::hash64(&x).cmp(&kth_hash) {
                Ordering::Less | Ordering::Equal => 1,
                _ => 0,
            })
            .collect();
        let (count, locs) = par_scan(&leq_flags);

        let mut samples = vec![xs[0].clone(); count as usize];
        let samples_slice = UnsafeSlice::new(&mut samples);
        xs.par_iter().enumerate().for_each(|(i, x): (usize, &T)| {
            if leq_flags[i] == 1 {
                unsafe {
                    samples_slice.write(locs[i] as usize, x.clone());
                }
            }
        });

        Some(samples)
    }
}

mod test {
    #[test]
    fn prefix_sum() {
        use crate::priority_sampler::par_scan;
        use rand::seq::SliceRandom;

        let sample_size = 1_000;
        let mut population = (0..sample_size).collect::<Vec<i32>>();
        let mut rng = rand::thread_rng();
        population.shuffle(&mut rng);

        let mut acc = 0;
        let mut seq_ps: Vec<i32> = Vec::with_capacity(sample_size as usize);
        population.iter().for_each(|elm| {
            seq_ps.push(acc);
            acc += elm;
        });

        let (ps, partials) = par_scan(&population);

        assert_eq!(acc, ps);
        assert_eq!(&seq_ps, &partials);
    }

    #[test]
    fn ps_test_len() {
        use crate::{models::Sampler, priority_sampler::PrioritySampler};

        let k = 50_000;
        let sample_size = 10_000_000;
        let population = (0..sample_size).collect::<Vec<i32>>();
        let samples = PrioritySampler::sample(&population, k);

        assert_eq!(k, samples.unwrap().len());
    }

    #[test]
    fn ps_rd_test_len() {
        use crate::{models::Sampler, priority_sampler::PrioritySampler};

        let k = 50_000;
        let sample_size = 10_000_000;
        let population = (0..sample_size)
            .map(|_| rand::random::<i32>())
            .collect::<Vec<i32>>();
        let samples = PrioritySampler::sample(&population, k);

        assert_eq!(k, samples.unwrap().len());
    }
}
