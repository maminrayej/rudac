use crate::algo::find::median_with;
use crate::algo::transform::partition_with;
use std::cmp::Ordering;

pub fn quick_sort<T: Ord + std::fmt::Debug>(slice: &mut [T]) {
    quick_sort_with(slice, &|x1: &T, x2: &T| x1.cmp(x2))
}

pub fn quick_sort_with<T, F>(slice: &mut [T], compare: &F)
where
    F: Fn(&T, &T) -> Ordering,
{
    if slice.len() == 0 || slice.len() == 1 {
        return;
    }

    let low = 0;
    let high = slice.len() - 1;

    if low < high {
        let pivot_index = median_with(slice, compare);

        let pivot_index = partition_with(slice, low, high, pivot_index, compare);

        quick_sort_with(&mut slice[low..pivot_index], compare);
        quick_sort_with(&mut slice[pivot_index + 1..high + 1], compare);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn algo_sort_quick_1() {
        let mut vec = Vec::with_capacity(100);

        for i in (0..100).rev() {
            vec.push(i);
        }

        quick_sort(&mut vec);

        for i in 0..100 {
            assert_eq!(vec[i], i);
        }
    }
}
