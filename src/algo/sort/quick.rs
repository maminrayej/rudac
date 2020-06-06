use crate::algo::find::median_with;
use crate::algo::transform::partition_with;
use std::cmp::Ordering;

/// Quicksort is an efficient sorting algorithm
/// 
/// # Arguments
/// * `slice`: slice of data to be sorted
/// 
/// # Examples
/// ```
/// use rudac::algo::sort::quick_sort;
/// 
/// let mut vec = vec![10, 6, 1, 4, 2, 3, 7, 9, 8, 5];
/// 
/// quick_sort(&mut vec);
/// 
/// assert_eq!(vec, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
/// ```
pub fn quick_sort<T: Ord>(slice: &mut [T]) {
    quick_sort_with(slice, &|x1: &T, x2: &T| x1.cmp(x2))
}

/// Quicksort is an efficient sorting algorithm
/// 
/// # Arguments
/// * `slice`: slice of data to be sorted
/// * `compare`: custom comparison closure
/// 
/// # Examples
/// ```
/// use rudac::algo::sort::quick_sort_with;
/// 
/// // consider vector of 2d points
/// let mut vec = vec![(1, 10), (2,6), (3,1), (3,4), (4,2), (5,3), (6,7), (8,9), (9,8), (10,5)];
/// 
/// // sort based on y axis
/// quick_sort_with(&mut vec, &|x1,x2| {x1.1.cmp(&x2.1)});
/// 
/// assert_eq!(vec, vec![(3,1), (4,2), (5,3), (3,4), (10,5), (2,6), (6,7), (9,8), (8,9), (1,10)]);
/// ```
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

        let pivot_index = partition_with(&mut slice[low..high+1], pivot_index, compare);

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
