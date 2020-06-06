use std::cmp::{Ord, Ordering};

pub fn fibonacci_search<T: Ord>(slice: &[T], item: &T) -> Option<usize> {
    fibonacci_search_with(slice, item, &|x1: &T, x2: &T| {x1.cmp(&x2)})
}

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
