use crate::algo::search::binary_search_with;
use std::cmp::{Ord, Ordering};

pub fn exponential_search<T: Ord>(slice: &[T], item: &T) -> Option<usize> {
    exponential_search_with(slice, item, &|x1: &T, x2: &T| x1.cmp(x2))
}

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