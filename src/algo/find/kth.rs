use std::cmp::{Ord, Ordering};
use crate::algo::transform::partition_with;

pub fn kth<T: Ord>(slice: &mut [T], k: usize) -> usize {
    kth_with(slice, k, |x1: &T, x2:&T| {x1.cmp(x2)})
}

pub fn kth_with<T, F>(slice: &mut [T], k: usize, compare: F) -> usize
where
    F: Fn(&T, &T) -> Ordering,
{
    let mut current_slice = &mut slice[..];
    let mut size = current_slice.len();

    while size != 1 {
        let mut index = 0;
        for i in (0..size).step_by(5) {
            let upper = std::cmp::min(i + 5, size);
            set_median_with(&mut current_slice[i..upper], &compare);

            current_slice.swap(index, index * 5);
            index += 1;
        }

        current_slice = &mut current_slice[0..index];
        size = current_slice.len();
    }

    size = slice.len();

    let pivot_index = partition_with(slice, 0, size - 1, 0, &compare);

    if k < pivot_index {
        return kth_with(&mut slice[0..pivot_index], k, compare);
    } else if k > pivot_index {
        return pivot_index + 1 + kth_with(&mut slice[pivot_index + 1..], k - pivot_index - 1, compare);
    } else {
        return pivot_index;
    }
}

fn set_median_with<T, F>(slice: &mut [T], compare: &F)
where
    F: Fn(&T, &T) -> Ordering,
{
    let size = slice.len();
    for i in 0..size {
        for j in 0..size - i - 1 {
            if compare(&slice[j], &slice[j + 1]) == Ordering::Greater {
                slice.swap(j, j + 1)
            }
        }
    }

    slice.swap(0, size / 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn algo_find_set_median_1() {
        let mut vec1 = vec![1];
        let mut vec2 = vec![2, 1];
        let mut vec3 = vec![2, 3, 1];
        let mut vec4 = vec![4, 3, 1, 2];
        let mut vec5 = vec![3, 4, 2, 1, 5];
        let mut vec10 = vec![10, 6, 1, 4, 2, 3, 7, 9, 8, 5];

        set_median_with(&mut vec1, &|x1: &usize, x2: &usize| {x1.cmp(x2)});
        set_median_with(&mut vec2, &|x1: &usize, x2: &usize| {x1.cmp(x2)});
        set_median_with(&mut vec3, &|x1: &usize, x2: &usize| {x1.cmp(x2)});
        set_median_with(&mut vec4, &|x1: &usize, x2: &usize| {x1.cmp(x2)});
        set_median_with(&mut vec5, &|x1: &usize, x2: &usize| {x1.cmp(x2)});
        set_median_with(&mut vec10, &|x1: &usize, x2: &usize| {x1.cmp(x2)});

        assert_eq!(vec1, vec![1]);
        assert_eq!(vec2, vec![2, 1]);
        assert_eq!(vec3, vec![2, 1, 3]);
        assert_eq!(vec4, vec![3, 2, 1, 4]);
        assert_eq!(vec5, vec![3, 2, 1, 4, 5]);
        assert_eq!(vec10, vec![6, 2, 3, 4, 5, 1, 7, 8, 9, 10]);
    }

    #[test]
    fn algo_find_kth_1() {
        let mut vec = vec![10, 6, 1, 4, 2, 3, 7, 9, 8, 5];

        for i in 0..vec.len() {
            let index = kth(&mut vec, i);
            assert_eq!(vec[index], i + 1);
        }
    }

    #[test]
    fn algo_find_kth_2() {
        let mut vec = Vec::<usize>::with_capacity(100);

        for i in (0..vec.capacity()).rev() {
            vec.push(i);
        }

        for i in 0..vec.len() {
            let index = kth(&mut vec, i);
            assert_eq!(vec[index], i);
        }
    }
}
