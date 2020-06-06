use std::cmp::{Ord, Ordering};

/// Fibonacci search is a search algorithm that finds the position of a target value within a sorted array.
/// Returns index of the found item, None otherwise
/// 
/// It has the advantage that one only needs addition and subtraction to calculate the indices of the accessed array elements, 
/// while classical binary search needs bit-shift, division or multiplication
///
/// # Arguments
/// * `slice`: slice of ordered data
/// * `item`: item to be searched for
///
/// # Examples
/// ```
/// use rudac::algo::search::fibonacci_search;
///
/// let mut vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
///
/// assert_eq!(vec[fibonacci_search(&vec, &1).unwrap()], 1);
/// assert_eq!(vec[fibonacci_search(&vec, &2).unwrap()], 2);
/// assert_eq!(vec[fibonacci_search(&vec, &3).unwrap()], 3);
///
/// assert_eq!(fibonacci_search(&vec, &11), None);
/// assert_eq!(fibonacci_search(&vec, &12), None);
/// assert_eq!(fibonacci_search(&vec, &13), None);
/// ```
pub fn fibonacci_search<T: Ord>(slice: &[T], item: &T) -> Option<usize> {
    fibonacci_search_with(slice, item, &|x1: &T, x2: &T| {x1.cmp(&x2)})
}

/// Fibonacci search is a search algorithm that finds the position of a target value within a sorted array.
/// Returns index of the found item, None otherwise
/// 
/// It has the advantage that one only needs addition and subtraction to calculate the indices of the accessed array elements, 
/// while classical binary search needs bit-shift, division or multiplication
///
/// # Arguments
/// * `slice`: slice of ordered data
/// * `item`: item to be searched for
/// * `compare`: custom comparison closure
///
/// # Examples
/// ```
/// use rudac::algo::search::fibonacci_search_with;
///
/// // consider a vector of 2d points
/// let mut vec = vec![(3,1), (4,2), (5,3), (3,4), (10,5), (2,6), (6,7), (9,8), (8,9), (1,10)];
/// 
/// let compare = |x1: &(usize, usize),x2: &(usize, usize)| {x1.1.cmp(&x2.1)};
/// assert_eq!(vec[fibonacci_search_with(&vec, &(3,1), &compare).unwrap()], (3,1));
/// assert_eq!(vec[fibonacci_search_with(&vec, &(4,2), &compare).unwrap()], (4,2));
/// assert_eq!(vec[fibonacci_search_with(&vec, &(5,3), &compare).unwrap()], (5,3));
///
/// assert_eq!(fibonacci_search_with(&vec, &(1,11), &compare), None);
/// assert_eq!(fibonacci_search_with(&vec, &(1,12), &compare), None);
/// assert_eq!(fibonacci_search_with(&vec, &(1,13), &compare), None);
/// ```
pub fn fibonacci_search_with<T, F>(slice: &[T], item: &T, compare: &F) -> Option<usize>
where
    F: Fn(&T, &T) -> Ordering,
{
    let n = slice.len();
    let mut fib2 = 0;
    let mut fib1 = 1;
    let mut fib = fib2 + fib1;

    while fib < n {
        fib2 = fib1;
        fib1 = fib;
        fib = fib2 + fib1;
    }

    let mut offset = -1;

    while fib > 1 {
        let i = std::cmp::min(offset + (fib2 as i64), (n - 1) as i64) as usize;

        if compare(&slice[i], item) == Ordering::Less {
            fib = fib1;
            fib1 = fib2;
            fib2 = fib - fib1;
            offset = i as i64;
        } else if compare(&slice[i], item) == Ordering::Greater {
            fib = fib2;
            fib1 = fib1 - fib2;
            fib2 = fib - fib1;
        } else {
            return Some(i);
        }
    }

    let upper = std::cmp::min((offset + 1) as usize, n - 1);
    if compare(&slice[upper], item) == Ordering::Equal {
        return Some(upper);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn algo_search_fibonacci_search() {
        let mut vec = Vec::with_capacity(100);

        for i in 50..100 {
            vec.push(i);
        }

        for i in 50..100 {
            assert_eq!(vec[fibonacci_search(&vec, &i).unwrap()], i);
        }

        for i in 0..50 {
            assert_eq!(fibonacci_search(&vec, &i), None);
        }
        for i in 100..150 {
            assert_eq!(fibonacci_search(&vec, &i), None);
        }
    }
}
