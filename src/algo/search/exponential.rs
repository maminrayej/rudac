use crate::algo::search::binary_search_with;
use std::cmp::{Ord, Ordering};

/// Exponential search is a search algorithm that finds the position of a target value within a sorted array.
/// Suitable for searching sorted, unbounded/infinite lists.
/// Returns index of the found item, None otherwise
///
/// # Arguments
/// * `slice`: slice of ordered data
/// * `item`: item to be searched for
///
/// # Examples
/// ```
/// use rudac::algo::search::exponential_search;
///
/// let mut vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
///
/// assert_eq!(vec[exponential_search(&vec, &1).unwrap()], 1);
/// assert_eq!(vec[exponential_search(&vec, &2).unwrap()], 2);
/// assert_eq!(vec[exponential_search(&vec, &3).unwrap()], 3);
///
/// assert_eq!(exponential_search(&vec, &11), None);
/// assert_eq!(exponential_search(&vec, &12), None);
/// assert_eq!(exponential_search(&vec, &13), None);
/// ```
pub fn exponential_search<T: Ord>(slice: &[T], item: &T) -> Option<usize> {
    exponential_search_with(slice, item, &|x1: &T, x2: &T| x1.cmp(x2))
}

/// Exponential search is a search algorithm that finds the position of a target value within a sorted array.
/// Suitable for searching sorted, unbounded/infinite lists.
/// Returns index of the found item, None otherwise
///
/// # Arguments
/// * `slice`: slice of ordered data
/// * `item`: item to be searched for
/// * `compare`: custom comparison closure
/// 
/// # Examples
/// ```
/// use rudac::algo::search::exponential_search_with;
///
/// // consider a vector of 2d points
/// let mut vec = vec![(3,1), (4,2), (5,3), (3,4), (10,5), (2,6), (6,7), (9,8), (8,9), (1,10)];
/// 
/// let compare = |x1: &(usize, usize),x2: &(usize, usize)| {x1.1.cmp(&x2.1)};
/// assert_eq!(vec[exponential_search_with(&vec, &(3,1), &compare).unwrap()], (3,1));
/// assert_eq!(vec[exponential_search_with(&vec, &(4,2), &compare).unwrap()], (4,2));
/// assert_eq!(vec[exponential_search_with(&vec, &(5,3), &compare).unwrap()], (5,3));
///
/// assert_eq!(exponential_search_with(&vec, &(1,11), &compare), None);
/// assert_eq!(exponential_search_with(&vec, &(1,12), &compare), None);
/// assert_eq!(exponential_search_with(&vec, &(1,13), &compare), None);
/// ```
pub fn exponential_search_with<T, F>(slice: &[T], item: &T, compare: &F) -> Option<usize>
where
    F: Fn(&T, &T) -> Ordering,
{
    let n = slice.len();

    if compare(&slice[0], item) == Ordering::Equal {
        return Some(0);
    }

    let mut i = 1;
    while i < n
        && (compare(&slice[i], item) == Ordering::Less
            || compare(&slice[i], item) == Ordering::Equal)
    {
        i *= 2;
    }

    let upper = std::cmp::min(i + 1, n);
    let index = binary_search_with(&slice[i / 2..upper], item, compare);

    if index.is_some() {
        return Some(index.unwrap() + i / 2);
    }
    else {
        return None
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn algo_search_exponential_search() {
        let mut vec = Vec::with_capacity(100);

        for i in 50..100 {
            vec.push(i);
        }

        for i in 50..100 {
            assert_eq!(vec[exponential_search(&vec, &i).unwrap()], i);
        }

        for i in 0..50 {
            assert_eq!(exponential_search(&vec, &i), None);
        }
        
        for i in 100..150 {
            assert_eq!(exponential_search(&vec, &i), None);
        }
    }
}