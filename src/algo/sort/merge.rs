use std::cmp::{Ord, Ordering};

/// Merge sort is an efficient, general-purpose, comparison-based sorting algorithm
/// 
/// # Arguments
/// * `slice`: slice of data to be sorted
/// 
/// # Examples
/// ```
/// use rudac::algo::sort::merge_sort;
/// 
/// let mut vec = vec![10, 6, 1, 4, 2, 3, 7, 9, 8, 5];
/// 
/// merge_sort(&mut vec);
/// 
/// assert_eq!(vec, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
/// ```
pub fn merge_sort<T: Copy + Ord>(slice: &mut [T]) {
    merge_sort_with(slice, &|left: &T, right: &T| left.cmp(&right))
}

/// Merge sort is an efficient, general-purpose, comparison-based sorting algorithm
/// 
/// # Arguments
/// * `slice`: slice of data to be sorted
/// * `compare`: custom comparison closure
/// 
/// # Examples
/// ```
/// use rudac::algo::sort::merge_sort_with;
/// 
/// // consider vector of 2d points
/// let mut vec = vec![(1, 10), (2,6), (3,1), (3,4), (4,2), (5,3), (6,7), (8,9), (9,8), (10,5)];
/// 
/// // sort based on y axis
/// merge_sort_with(&mut vec, &|x1,x2| {x1.1.cmp(&x2.1)});
/// 
/// assert_eq!(vec, vec![(3,1), (4,2), (5,3), (3,4), (10,5), (2,6), (6,7), (9,8), (8,9), (1,10)]);
/// ```
pub fn merge_sort_with<T: Copy, F>(slice: &mut [T], compare: &F)
where
    F: Fn(&T, &T) -> Ordering,
{
    let slice_len = slice.len();
    let middle = slice_len / 2;

    if slice_len <= 1 {
        return;
    }

    merge_sort_with(&mut slice[0..middle], compare);
    merge_sort_with(&mut slice[middle..slice_len], compare);

    let mut merge: Vec<T> = slice.to_vec();

    merge_with(&slice[0..middle], &slice[middle..slice_len], &mut merge[..], compare);

    slice.copy_from_slice(&merge);
}

fn merge_with<T: Copy, F>(left: &[T], right: &[T], merge: &mut [T], compare: F)
where
    F: Fn(&T, &T) -> Ordering,
{
    assert_eq!(left.len() + right.len(), merge.len());
    let mut left_index = 0;
    let mut right_index = 0;
    let mut merged_index = 0;
    while left_index < left.len() && right_index < right.len() {
        if compare(&left[left_index], &right[right_index]) == Ordering::Less {
            merge[merged_index] = left[left_index];
            merged_index += 1;
            left_index += 1;
        } else {
            merge[merged_index] = right[right_index];
            merged_index += 1;
            right_index += 1;
        }
    }
    if left_index < left.len() {
        merge[merged_index..].copy_from_slice(&left[left_index..]);
    }
    if right_index < right.len() {
        merge[merged_index..].copy_from_slice(&right[right_index..]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn algo_sort_merge_1() {
        let mut vec = Vec::with_capacity(100);

        for left_index in (0..100).rev() {
            vec.push(left_index);
        }

        merge_sort(&mut vec);

        for left_index in 0..100 {
            assert_eq!(vec[left_index], left_index);
        }
    }
}
