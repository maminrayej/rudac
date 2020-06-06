use std::cmp::{Ord, Ordering};

pub fn linear_search<T: Ord>(slice: &[T], item: &T) -> Option<usize> {
    linear_search_with(slice, item, &|x1: &T, x2: &T| x1.cmp(&x2))
}

pub fn linear_search_with<T, F>(slice: &[T], item: &T, compare: &F) -> Option<usize>
where
    F: Fn(&T, &T) -> Ordering,
{
    for i in 0..slice.len() {
        if compare(&slice[i], item) == Ordering::Equal {
            return Some(i);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn algo_search_linear_search() {
        let mut vec = Vec::with_capacity(100);

        for i in 50..100 {
            vec.push(i);
        }

        for i in 50..100 {
            assert_eq!(vec[linear_search(&vec, &i).unwrap()], i);
        }

        for i in 0..50 {
            assert_eq!(linear_search(&vec, &i), None);
        }
        for i in 100..150 {
            assert_eq!(linear_search(&vec, &i), None);
        }
    }
}
