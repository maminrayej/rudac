use std::cmp::Ord;

pub fn kth<T: Ord>(slice: &mut [T], k: usize) -> &T {
    let mut current_slice = &mut slice[..];
    let mut size = current_slice.len();

    while size != 1 {
        let mut index = 0;
        for i in (0..size).step_by(5) {
            let upper = std::cmp::min(i + 5, size);
            set_median(&mut current_slice[i..upper]);

            current_slice.swap(index, index * 5);
            index += 1;
        }

        current_slice = &mut current_slice[0..index];
        size = current_slice.len();
    }

    slice.swap(0, slice.len() - 1);

    let left_part = partition(slice, 0, slice.len() - 1);

    let len_low = (left_part - 0 + 1) as usize;
    if k < len_low {
        return kth(&mut slice[0..len_low], k);
    } else if k > len_low {
        return kth(&mut slice[len_low + 1..], k - len_low - 1);
    } else {
        return &slice[len_low];
    }
}

fn set_median<T: Ord>(slice: &mut [T]) {
    let size = slice.len();
    for i in 0..size {
        for j in 0..size - i - 1 {
            if slice[j] > slice[j + 1] {
                slice.swap(j, j + 1)
            }
        }
    }

    slice.swap(0, size / 2)
}

fn partition<T: Ord>(slice: &mut [T], low: usize, high: usize) -> i64 {
    let mut i = low;
    for j in low..high {
        if slice[j] < slice[high] {
            slice.swap(i, j);
            i += 1;
        }
    }
    slice.swap(i, high);

    i as i64 - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn algo_find_kth_set_median_1() {
        let mut vec1 = vec![1];
        let mut vec2 = vec![2, 1];
        let mut vec3 = vec![2, 3, 1];
        let mut vec4 = vec![4, 3, 1, 2];
        let mut vec5 = vec![3, 4, 2, 1, 5];
        let mut vec10 = vec![10, 6, 1, 4, 2, 3, 7, 9, 8, 5];

        set_median(&mut vec1);
        set_median(&mut vec2);
        set_median(&mut vec3);
        set_median(&mut vec4);
        set_median(&mut vec5);
        set_median(&mut vec10);

        assert_eq!(vec1, vec![1]);
        assert_eq!(vec2, vec![2, 1]);
        assert_eq!(vec3, vec![2, 1, 3]);
        assert_eq!(vec4, vec![3, 2, 1, 4]);
        assert_eq!(vec5, vec![3, 2, 1, 4, 5]);
        assert_eq!(vec10, vec![6, 2, 3, 4, 5, 1, 7, 8, 9, 10]);
    }

    #[test]
    fn algo_find_kth_partition_1() {
        let mut vec1 = vec![1];
        let mut vec2 = vec![2, 1];
        let mut vec3 = vec![2, 3, 1];
        let mut vec4 = vec![4, 3, 1, 2];
        let mut vec5 = vec![3, 4, 2, 1, 5];
        let mut vec10 = vec![10, 6, 1, 4, 2, 3, 7, 9, 8, 5];

        set_median(&mut vec1);
        vec1.swap(0, 0);
        let left_index = partition(&mut vec1, 0, 0);
        assert_eq!(vec1, vec![1]);
        assert_eq!(left_index, -1);

        set_median(&mut vec2);
        vec2.swap(0, 1);
        let left_index = partition(&mut vec2, 0, 1);
        assert_eq!(vec2, vec![1, 2]);
        assert_eq!(left_index, 0);

        set_median(&mut vec3);
        vec3.swap(0, 2);
        let left_index = partition(&mut vec3, 0, 2);
        assert_eq!(vec3, vec![1, 2, 3]);
        assert_eq!(left_index, 0);

        set_median(&mut vec4);
        vec4.swap(0, 3);
        let left_index = partition(&mut vec4, 0, 3);
        assert_eq!(vec4, vec![2, 1, 3, 4]);
        assert_eq!(left_index, 1);

        set_median(&mut vec5);
        vec5.swap(0, 4);
        let left_index = partition(&mut vec5, 0, 4);
        assert_eq!(vec5, vec![2, 1, 3, 4, 5]);
        assert_eq!(left_index, 1);

        set_median(&mut vec10);
        vec10.swap(0, 9);
        let left_index = partition(&mut vec10, 0, 9);
        assert_eq!(vec10, vec![2, 3, 4, 5, 1, 6, 7, 8, 9, 10]);
        assert_eq!(left_index, 4);
    }
}
