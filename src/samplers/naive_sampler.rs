use std::{cmp::Ordering, marker::PhantomData};

use rand::Rng;

use super::sampl_interface::Sampler;

pub struct NaiveSampler<T: Clone + Sized + Send + Sync> {
    marker: PhantomData<T>,
}

impl<T: Clone + Sized + Send + Sync> Sampler<T> for NaiveSampler<T> {
    fn sample(arr: &[T], k: usize) -> Option<Vec<T>> {
        let mut working_arr = arr.to_vec();
        match k.cmp(&arr.len()) {
            Ordering::Greater => None,
            _ => {
                let mut rng = rand::thread_rng();
                let mut ans = Vec::with_capacity(k);
                for _ in 0..k {
                    let idx = rng.gen_range(0..working_arr.len());
                    let to_remove = working_arr.remove(idx);
                    ans.push(to_remove);
                }

                Some(ans)
            }
        }
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
