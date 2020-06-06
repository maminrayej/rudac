use std::cmp::{Ord, Ordering};

pub fn merge_sort<T: Copy + Ord>(slice: &mut [T]) {
    merge_sort_with(slice, &|left: &T, right: &T| left.cmp(&right))
}

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
