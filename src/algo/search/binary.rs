use std::cmp::{Ord, Ordering};

/// Binary search is a search algorithm that finds the position of a target value within a sorted array.
///
/// Returns index of the found item, None otherwise
///
/// # Arguments
/// * `slice`: slice of ordered data
/// * `item`: item to be searched for
///
/// # Examples
/// ```
/// use rudac::algo::search::binary_search;
///
/// let mut vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
///
/// assert_eq!(vec[binary_search(&vec, &1).unwrap()], 1);
/// assert_eq!(vec[binary_search(&vec, &2).unwrap()], 2);
/// assert_eq!(vec[binary_search(&vec, &3).unwrap()], 3);
///
/// assert_eq!(binary_search(&vec, &11), None);
/// assert_eq!(binary_search(&vec, &12), None);
/// assert_eq!(binary_search(&vec, &13), None);
/// ```
pub fn binary_search<T: Ord>(slice: &[T], item: &T) -> Option<usize> {
    binary_search_with(slice, item, &|x1: &T, x2: &T| x1.cmp(&x2))
}

/// Binary search is a search algorithm that finds the position of a target value within a sorted array.
///
/// Returns index of the found item, None otherwise
///
/// # Arguments
/// * `slice`: slice of ordered data
/// * `item`: item to be searched for
/// * `compare`: custom comparison closure
///
/// # Examples
/// ```
/// use rudac::algo::search::binary_search_with;
///
/// // consider a vector of 2d points
/// let mut vec = vec![(3,1), (4,2), (5,3), (3,4), (10,5), (2,6), (6,7), (9,8), (8,9), (1,10)];
/// 
/// let compare = |x1: &(usize, usize),x2: &(usize, usize)| {x1.1.cmp(&x2.1)};
/// assert_eq!(vec[binary_search_with(&vec, &(3,1), &compare).unwrap()], (3,1));
/// assert_eq!(vec[binary_search_with(&vec, &(4,2), &compare).unwrap()], (4,2));
/// assert_eq!(vec[binary_search_with(&vec, &(5,3), &compare).unwrap()], (5,3));
///
/// assert_eq!(binary_search_with(&vec, &(1,11), &compare), None);
/// assert_eq!(binary_search_with(&vec, &(1,12), &compare), None);
/// assert_eq!(binary_search_with(&vec, &(1,13), &compare), None);
/// ```
pub fn binary_search_with<T, F>(slice: &[T], item: &T, compare: &F) -> Option<usize>
where
    F: Fn(&T, &T) -> Ordering,
{
    let mut left = 0;
    let mut right = slice.len() - 1;

    while left <= right {
        let mid: usize;

        if right == 0 {
            mid = right;
            if compare(&slice[mid], item) == Ordering::Equal {
                return Some(mid);
            } else {
                return None;
            }
        } else {
            mid = (left + (right - 1)) / 2 + 1;
        }

        if compare(&slice[mid], item) == Ordering::Equal {
            return Some(mid);
        } else if compare(&slice[mid], item) == Ordering::Less {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn algo_search_binary_search() {
        let mut vec = Vec::with_capacity(100);

        for i in 50..100 {
            vec.push(i);
        }

        for i in 50..100 {
            assert_eq!(vec[binary_search(&vec, &i).unwrap()], i);
        }

        for i in 0..50 {
            assert_eq!(binary_search(&vec, &i), None);
        }
        for i in 100..150 {
            assert_eq!(binary_search(&vec, &i), None);
        }
    }
}
