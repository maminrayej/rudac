use crate::algo::transform::partition_with;
use std::cmp::{Ord, Ordering};

/// Returns index of kth smallest item in the slice
///
/// # Arguments
/// * `slice`: slice of unordered data
/// * `k`: kth
///
/// # Panics
/// * panics if k is out of range: 0 <= k < slice.len()
///
/// # Examples
/// ```
/// use rudac::algo::find::kth;
///
/// let mut vec = vec![10, 6, 1, 4, 2, 3, 7, 9, 8, 5];
///
/// let fifth_item_index = kth(&mut vec, 4);
/// assert_eq!(vec[fifth_item_index], 5);
/// ```
pub fn kth<T: Ord>(slice: &mut [T], k: usize) -> usize {
    if k >= slice.len() {
        panic!("k is out of range: 0 <= k < slice.len()");
    }

    kth_with(slice, k, &|x1: &T, x2: &T| x1.cmp(x2))
}

/// Returns index of kth smallest item in the slice using a customized closure for comparison
///
/// # Arguments
/// * `slice`: slice of unordered data
/// * `k`: kth
/// * `compare`: custom comparing closure
/// 
/// # Panics
/// * panics if k is out of range: 0 <= k < slice.len()
///
/// # Examples
/// ```
/// use rudac::algo::find::kth_with;
/// 
/// // consider a vector of 2d points
/// let mut vec = vec![(1, 10), (2,6), (3,1), (3,4), (4,2), (5,3), (6,7), (8,9), (9,8), (10,5)];
/// 
/// // find index of kth smallest point based on their y axis only
/// let fifth_item_index = kth_with(&mut vec, 4, &|x1,x2| {x1.1.cmp(&x2.1)});
/// assert_eq!(vec[fifth_item_index], (10,5));
/// ```
pub fn kth_with<T, F>(slice: &mut [T], k: usize, compare: &F) -> usize
where
    F: Fn(&T, &T) -> Ordering,
{
    let mut current_slice = &mut slice[..];
    let mut size = current_slice.len();

    while size != 1 {
        let mut index = 0;
        for i in (0..size).step_by(5) {
            let upper = std::cmp::min(i + 5, size);
            set_median_with(&mut current_slice[i..upper], compare);

            current_slice.swap(index, index * 5);
            index += 1;
        }

        current_slice = &mut current_slice[0..index];
        size = current_slice.len();
    }

    let pivot_index = partition_with(slice, 0, compare);

    if k < pivot_index {
        return kth_with(&mut slice[0..pivot_index], k, compare);
    } else if k > pivot_index {
        return pivot_index
            + 1
            + kth_with(&mut slice[pivot_index + 1..], k - pivot_index - 1, compare);
    } else {
        return pivot_index;
    }
}

/// Returns index of smallest item in the slice
/// 
/// It is equivalent to calling kth(slice, 0)
/// 
/// # Arguments
/// * `slice`: slice of unordered data
///
/// # Examples
/// ```
/// use rudac::algo::find::min;
///
/// let mut vec = vec![10, 6, 1, 4, 2, 3, 7, 9, 8, 5];
///
/// let smallest_item_index = min(&mut vec);
/// assert_eq!(vec[smallest_item_index], 1);
/// ```
pub fn min<T: Ord>(slice: &mut [T]) -> usize {
    kth(slice, 0)
}

/// Returns index of smallest item in the slice using a customized closure for comparison
///
/// It is equivalent to calling kth_with(slice, 0)
/// 
/// # Arguments
/// * `slice`: slice of unordered data
/// * `compare`: custom comparing closure
///
/// # Examples
/// ```
/// use rudac::algo::find::min_with;
///
/// // consider a vector of 2d points
/// let mut vec = vec![(1, 10), (2,6), (3,1), (3,4), (4,2), (5,3), (6,7), (8,9), (9,8), (10,5)];
///
/// // find index of smallest point based on their y axis only
/// let smallest_item_index = min_with(&mut vec, &|x1,x2| {x1.1.cmp(&x2.1)});
/// assert_eq!(vec[smallest_item_index], (3,1));
/// ```
pub fn min_with<T, F>(slice: &mut [T], compare: &F) -> usize
where
    F: Fn(&T, &T) -> Ordering,
{
    kth_with(slice, 0, compare)
}

/// Returns index of largest item in the slice
/// 
/// It is equivalent to calling kth(slice, slice.len()-1)
/// 
/// # Arguments
/// * `slice`: slice of unordered data
///
/// # Examples
/// ```
/// use rudac::algo::find::max;
///
/// let mut vec = vec![10, 6, 1, 4, 2, 3, 7, 9, 8, 5];
///
/// let largest_item_index = max(&mut vec);
/// assert_eq!(vec[largest_item_index], 10);
/// ```
pub fn max<T: Ord>(slice: &mut [T]) -> usize {
    kth(slice, slice.len() - 1)
}

/// Returns index of largest item in the slice using a customized closure for comparison
///
/// It is equivalent to calling kth_with(slice, slice.len()-1)
/// 
/// # Arguments
/// * `slice`: slice of unordered data
/// * `compare`: custom comparing closure
///
/// # Examples
/// ```
/// use rudac::algo::find::max_with;
///
/// // consider a vector of 2d points
/// let mut vec = vec![(1, 10), (2,6), (3,1), (3,4), (4,2), (5,3), (6,7), (8,9), (9,8), (10,5)];
///
/// // find index of largest point based on their y axis only
/// let largest_item_index = max_with(&mut vec, &|x1,x2| {x1.1.cmp(&x2.1)});
/// assert_eq!(vec[largest_item_index], (1,10));
/// ```
pub fn max_with<T, F>(slice: &mut [T], compare: &F) -> usize
where
    F: Fn(&T, &T) -> Ordering,
{
    kth_with(slice, slice.len() - 1, compare)
}

/// Returns index of median item in the slice
/// 
/// It is equivalent to calling kth(slice, slice.len()/2)
/// 
/// # Arguments
/// * `slice`: slice of unordered data
///
/// # Examples
/// ```
/// use rudac::algo::find::median;
///
/// let mut vec = vec![10, 6, 1, 4, 2, 3, 7, 9, 8, 5];
///
/// let median_item_index = median(&mut vec);
/// assert_eq!(vec[median_item_index], 6);
/// ```
pub fn median<T: Ord>(slice: &mut [T]) -> usize {
    kth(slice, slice.len() / 2)
}

/// Returns index of median item in the slice using a customized closure for comparison
///
/// It is equivalent to calling kth_with(slice, slice.len()/2)
/// 
/// # Arguments
/// * `slice`: slice of unordered data
/// * `compare`: custom comparing closure
///
/// # Examples
/// ```
/// use rudac::algo::find::median_with;
///
/// // consider a vector of 2d points
/// let mut vec = vec![(1, 10), (2,6), (3,1), (3,4), (4,2), (5,3), (6,7), (8,9), (9,8), (10,5)];
///
/// // find index of median point based on their y axis only
/// let median_item_index = median_with(&mut vec, &|x1,x2| {x1.1.cmp(&x2.1)});
/// assert_eq!(vec[median_item_index], (2,6));
/// ```
pub fn median_with<T, F>(slice: &mut [T], compare: &F) -> usize
where
    F: Fn(&T, &T) -> Ordering,
{
    kth_with(slice, slice.len() / 2, compare)
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

        set_median_with(&mut vec1, &|x1: &usize, x2: &usize| x1.cmp(x2));
        set_median_with(&mut vec2, &|x1: &usize, x2: &usize| x1.cmp(x2));
        set_median_with(&mut vec3, &|x1: &usize, x2: &usize| x1.cmp(x2));
        set_median_with(&mut vec4, &|x1: &usize, x2: &usize| x1.cmp(x2));
        set_median_with(&mut vec5, &|x1: &usize, x2: &usize| x1.cmp(x2));
        set_median_with(&mut vec10, &|x1: &usize, x2: &usize| x1.cmp(x2));

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

    #[test]
    fn algo_find_kth_3() {
        let mut vec = vec![1, 1, 3, 4, 5, 5];

        let index = kth(&mut vec, 0);
        assert_eq!(vec[index], 1);

        let index = kth(&mut vec, 1);
        assert_eq!(vec[index], 1);

        let index = kth(&mut vec, 2);
        assert_eq!(vec[index], 3);

        let index = kth(&mut vec, 3);
        assert_eq!(vec[index], 4);

        let index = kth(&mut vec, 4);
        assert_eq!(vec[index], 5);

        let index = kth(&mut vec, 5);
        assert_eq!(vec[index], 5);
    }
}
