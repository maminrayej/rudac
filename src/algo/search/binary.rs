use std::cmp::{Ord, Ordering};

pub fn binary_search<T: Ord>(slice: &[T], item: &T) -> Option<usize> {
    binary_search_with(slice, item, &|x1: &T, x2: &T| {x1.cmp(&x2)})
}

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
            }
            else {
                return None;
            }
        }
        else {
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