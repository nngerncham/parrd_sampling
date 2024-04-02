use std::{collections::HashSet, marker::PhantomData};

use rand::Rng;

use super::sampl_interface::Sampler;

pub struct NaiveSampler<T: Clone + Sized + Send + Sync> {
    marker: PhantomData<T>,
}

impl<T: Clone + Sized + Send + Sync> Sampler<T> for NaiveSampler<T> {
    fn sample(arr: &[T], k: usize) -> Option<Vec<T>> {
        let mut ans = Vec::with_capacity(k);
        let mut idx_left: HashSet<usize> = (0..arr.len()).collect();
        let mut rng = rand::thread_rng();

        while ans.len() < k {
            let idx = rng.gen_range(0..arr.len());
            if idx_left.contains(&idx) {
                ans.push(arr[idx].clone());
                idx_left.remove(&idx);
            }
        }

        Some(ans)
    }
}

mod test {
    #[test]
    fn small_naive_sample() {
        use super::NaiveSampler;
        use crate::samplers::sampl_interface::Sampler;

        let xs = vec![3, 5, 1, 2, 3, 8, 6, 3];
        let xs_sample = NaiveSampler::sample(&xs, 3);
        assert_eq!(xs_sample.unwrap().len(), 3);
    }

    #[test]
    fn big_naive_sample() {
        use super::NaiveSampler;
        use crate::samplers::sampl_interface::Sampler;

        let n = 1_000_000;
        let k = 10_000;
        let xs: Vec<i32> = (0..n).collect();
        let xs_sample = NaiveSampler::sample(&xs, k);
        assert_eq!(xs_sample.unwrap().len(), k);
    }
}
