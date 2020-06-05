use std::cmp::{Ord, Ordering};

pub fn partition<T: Ord>(slice: &mut [T], low: usize, high: usize, pivot_index: usize) -> usize {
    partition_with(slice, low, high, pivot_index, &|x1: &T, x2: &T| {x1.cmp(x2)})
}

pub fn partition_with<T, F>(
    slice: &mut [T],
    low: usize,
    high: usize,
    pivot_index: usize,
    compare: &F,
) -> usize
where
    F: Fn(&T, &T) -> Ordering,
{
    slice.swap(high, pivot_index);

    let mut i = low;
    for j in low..high {
        if compare(&slice[j], &slice[high]) == Ordering::Less {
            slice.swap(i, j);
            i += 1;
        }
    }
    slice.swap(i, high);

    i
}