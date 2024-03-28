use common_traits::Integer;
use std::cell::UnsafeCell;
use std::ptr;

// https://stackoverflow.com/questions/65178245/how-do-i-write-to-a-mutable-slice-from-multiple-threads-at-arbitrary-indexes-wit
#[derive(Copy, Clone)]
pub struct UnsafeSlice<'a, T> {
    slice: &'a [UnsafeCell<T>],
}
unsafe impl<'a, T: Send + Sync> Send for UnsafeSlice<'a, T> {}
unsafe impl<'a, T: Send + Sync> Sync for UnsafeSlice<'a, T> {}

impl<'a, T> UnsafeSlice<'a, T> {
    pub fn new(slice: &'a mut [T]) -> Self {
        let ptr = slice as *mut [T] as *const [UnsafeCell<T>];
        Self {
            slice: unsafe { &*ptr },
        }
    }

    /// SAFETY: It is UB if two threads write to the same index without
    /// synchronization.
    pub unsafe fn write(&self, i: usize, value: T) {
        let ptr = self.slice[i].get();
        *ptr = value;
    }

    pub unsafe fn read(&self, i: usize) -> &T {
        let ptr = self.slice[i].get();
        &*ptr
    }
}

impl<'a, T: Clone + Sized + Send + Sync> UnsafeSlice<'a, T> {
    pub unsafe fn swap(&self, loc_a: usize, loc_b: usize) {
        let ptr_a = self.slice[loc_a].get();
        let ptr_b = self.slice[loc_b].get();
        ptr::swap(ptr_a, ptr_b);
    }
}

impl<'a, T: Integer + Ord> UnsafeSlice<'a, T> {
    pub unsafe fn write_max(&self, i: usize, value: T) {
        let og_value = self.read(i);
        self.write(i, *og_value.max(&value));
    }
}
