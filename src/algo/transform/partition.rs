use std::cmp::{Ord, Ordering};

/// Partitions the slice around the element at `pivot_index`.
/// Returns index of pivot after partitioning
/// 
/// # Arguments
/// * `slice`: slice of data to be partitioned
/// * `pivot_index`: index of the pivot. slice will be partitioned around item at this index
/// 
/// # Examples
/// ```
/// use rudac::algo::transform::partition;
/// 
/// let mut vec = vec![10, 6, 1, 4, 2, 3, 7, 9, 8, 5];
/// 
/// partition(&mut vec, 3);
/// 
/// //                          pivot
/// //                            '
/// assert_eq!(vec, vec![1, 2, 3, 4, 6, 10, 7, 9, 8, 5]);
/// ```
pub fn partition<T: Ord>(slice: &mut [T], pivot_index: usize) -> usize {
    partition_with(slice, pivot_index, &|x1: &T, x2: &T| x1.cmp(x2))
}

/// Partitions the slice around the element at `pivot_index`.
/// Returns index of pivot after partitioning
/// 
/// # Arguments
/// * `slice`: slice of data to be partitioned
/// * `pivot_index`: index of the pivot. slice will be partitioned around item at this index
/// * `compare`: custom comparing closure
/// 
/// # Examples
/// ```
/// use rudac::algo::transform::partition_with;
/// 
/// let mut vec = vec![(1, 10), (2,6), (3,1), (3,4), (4,2), (5,3), (6,7), (8,9), (9,8), (10,5)];
/// 
/// partition_with(&mut vec, 3, &|x1,x2| {x1.1.cmp(&x2.1)});
/// 
/// //                                            pivot
/// //                                              '
/// assert_eq!(vec, vec![(3, 1), (4, 2), (5, 3), (3, 4), (2, 6), (1, 10), (6, 7), (8, 9), (9, 8), (10, 5)]);
/// ```
pub fn partition_with<T, F>(
    slice: &mut [T],
    pivot_index: usize,
    compare: &F,
) -> usize
where
    F: Fn(&T, &T) -> Ordering,
{
    let low = 0;
    let high = slice.len() - 1;

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
