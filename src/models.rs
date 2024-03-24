pub trait Sampler<T: Clone> {
    fn sample(arr: &[T], k: usize) -> Option<Vec<T>>;
}
