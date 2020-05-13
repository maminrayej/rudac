pub struct MinMax<T: std::cmp::Ord> {
    tree: Vec<T>,
}

impl<T: std::cmp::Ord + std::fmt::Debug> MinMax<T> {
    pub fn init() -> MinMax<T> {
        MinMax { tree: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> MinMax<T> {
        MinMax {
            tree: Vec::with_capacity(capacity),
        }
    }

    pub fn build_heap(arr: Vec<T>) -> MinMax<T> {
        let mut minmax_heap = MinMax { tree: arr };

        let upper_bound = minmax_heap.size() / 2;

        for i in (0..upper_bound).rev() {
            minmax_heap.push_down(i);
        }

        minmax_heap
    }

    fn push_down(&mut self, index: usize) {
        if is_on_min_level(index) {
            self.push_down_min(index);
        } else {
            self.push_down_max(index);
        }
    }

    fn push_down_min(&mut self, mut index: usize) {
        while has_child(index, self.size()) {
            let (smallest_index, is_grandchild) = self.smallest_child_or_grandchild(index);
            if is_grandchild {
                if self.tree[smallest_index] < self.tree[index] {
                    self.tree.swap(smallest_index, index);

                    if self.tree[smallest_index] > self.tree[parent(smallest_index)] {
                        self.tree.swap(smallest_index, parent(smallest_index));
                    }
                }
            } else {
                if self.tree[smallest_index] < self.tree[index] {
                    self.tree.swap(index, smallest_index);
                }
            }

            index = smallest_index;
        }
    }

    fn push_down_max(&mut self, mut index: usize) {
        while has_child(index, self.size()) {
            let (greatest_index, is_grandchild) = self.greatest_child_or_grandchild(index);
            if is_grandchild {
                if self.tree[greatest_index] > self.tree[index] {
                    self.tree.swap(index, greatest_index);

                    if self.tree[greatest_index] < self.tree[parent(greatest_index)] {
                        self.tree.swap(greatest_index, parent(greatest_index));
                    }
                }
            } else {
                if self.tree[greatest_index] > self.tree[index] {
                    self.tree.swap(index, greatest_index);
                }
            }
            index = greatest_index;
        }
    }

    fn smallest_child_or_grandchild(&self, index: usize) -> (usize, bool) {
        // check left sub tree
        if has_left_child(index, self.size()) {
            let left_child_index = left_child(index);

            let mut smallest_index = left_child_index;
            let mut is_grandchild = false;

            if self.tree[left_child_index] < self.tree[smallest_index] {
                smallest_index = left_child_index;
                is_grandchild = false;
            }

            // check grandchildren of left sub tree
            if has_left_child(left_child_index, self.size()) {
                let left_grandchild_index = left_child(left_child_index);
                if self.tree[left_grandchild_index] < self.tree[smallest_index] {
                    smallest_index = left_grandchild_index;
                    is_grandchild = true;
                }
            }
            if has_right_child(left_child_index, self.size()) {
                let right_grandchild_index = right_child(left_child_index);
                if self.tree[right_grandchild_index] < self.tree[smallest_index] {
                    smallest_index = right_grandchild_index;
                    is_grandchild = true;
                }
            }

            // check right sub tree
            if has_right_child(index, self.size()) {
                let right_child_index = right_child(index);
                if self.tree[right_child_index] < self.tree[smallest_index] {
                    smallest_index = right_child_index;
                    is_grandchild = false;
                }

                // check grandchildren of right sub tree
                if has_left_child(right_child_index, self.size()) {
                    let left_grandchild_index = left_child(right_child_index);
                    if self.tree[left_grandchild_index] < self.tree[smallest_index] {
                        smallest_index = left_grandchild_index;
                        is_grandchild = true;
                    }
                }

                if has_right_child(right_child_index, self.size()) {
                    let right_grandchild_index = right_child(right_child_index);
                    if self.tree[right_grandchild_index] < self.tree[smallest_index] {
                        smallest_index = right_grandchild_index;
                        is_grandchild = true;
                    }
                }
            }
            (smallest_index, is_grandchild)
        } else {
            (index, false)
        }
    }

    fn greatest_child_or_grandchild(&self, index: usize) -> (usize, bool) {
        // check left sub tree
        if has_left_child(index, self.size()) {
            let left_child_index = left_child(index);

            let mut greatest_index = left_child_index;
            let mut is_grandchild = false;

            if self.tree[left_child_index] > self.tree[greatest_index] {
                greatest_index = left_child_index;
                is_grandchild = false;
            }

            // check grandchildren of left sub tree
            if has_left_child(left_child_index, self.size()) {
                let left_grandchild_index = left_child(left_child_index);
                if self.tree[left_grandchild_index] > self.tree[greatest_index] {
                    greatest_index = left_grandchild_index;
                    is_grandchild = true;
                }
            }
            if has_right_child(left_child_index, self.size()) {
                let right_grandchild_index = right_child(left_child_index);
                if self.tree[right_grandchild_index] > self.tree[greatest_index] {
                    greatest_index = right_grandchild_index;
                    is_grandchild = true;
                }
            }

            // check right sub tree
            if has_right_child(index, self.size()) {
                let right_child_index = right_child(index);
                if self.tree[right_child_index] > self.tree[greatest_index] {
                    greatest_index = right_child_index;
                    is_grandchild = false;
                }

                // check grandchildren of right sub tree
                if has_left_child(right_child_index, self.size()) {
                    let left_grandchild_index = left_child(right_child_index);
                    if self.tree[left_grandchild_index] > self.tree[greatest_index] {
                        greatest_index = left_grandchild_index;
                        is_grandchild = true;
                    }
                }
                if has_right_child(right_child_index, self.size()) {
                    let right_grandchild_index = right_child(right_child_index);
                    if self.tree[right_grandchild_index] > self.tree[greatest_index] {
                        greatest_index = right_grandchild_index;
                        is_grandchild = true;
                    }
                }
            }
            (greatest_index, is_grandchild)
        } else {
            (index, false)
        }
    }

    pub fn push(&mut self, item: T) {
        self.tree.push(item);

        self.push_up(self.tree.len() - 1);
    }

    fn push_up(&mut self, index: usize) {
        if index != 0 {
            if is_on_min_level(index) {
                if self.tree[index] > self.tree[parent(index)] {
                    self.tree.swap(index, parent(index));
                    self.push_up_max(parent(index));
                } else {
                    self.push_up_min(index);
                }
            } else {
                if self.tree[index] < self.tree[parent(index)] {
                    self.tree.swap(index, parent(index));
                    self.push_up_min(parent(index));
                } else {
                    self.push_up_max(index);
                }
            }
        }
    }

    fn push_up_min(&mut self, mut index: usize) {
        while has_grandparent(index) && self.tree[index] < self.tree[grandparent(index)] {
            self.tree.swap(index, grandparent(index));

            index = grandparent(index);
        }
    }

    fn push_up_max(&mut self, mut index: usize) {
        while has_grandparent(index) && self.tree[index] > self.tree[grandparent(index)] {
            self.tree.swap(index, grandparent(index));

            index = grandparent(index);
        }
    }

    pub fn peek_min(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }

        Some(&self.tree[0])
    }

    pub fn peek_max(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }

        match self.size() {
            0 => None,
            1 => Some(&self.tree[0]),
            2 => Some(&self.tree[1]),
            _ => {
                if self.tree[1] > self.tree[2] {
                    Some(&self.tree[1])
                } else {
                    Some(&self.tree[2])
                }
            }
        }
    }

    pub fn pop_min(&mut self) -> Option<T> {
        match self.size() {
            0 => None,
            1 => Some(self.tree.pop().unwrap()),
            _ => {
                let mut last_item = self.tree.pop().unwrap();

                std::mem::swap(&mut last_item, &mut self.tree[0]);
                self.push_down(0);
                Some(last_item)
            }
        }
    }

    pub fn pop_max(&mut self) -> Option<T> {
        match self.size() {
            0 => None,
            1 | 2 => Some(self.tree.pop().unwrap()),
            _ => {
                let mut last_item: T;

                if self.tree[1] > self.tree[2] {
                    last_item = self.tree.pop().unwrap();
                    std::mem::swap(&mut last_item, &mut self.tree[1]);
                    self.push_down(1);
                } else {
                    last_item = self.tree.pop().unwrap();
                    std::mem::swap(&mut last_item, &mut self.tree[2]);
                    self.push_down(2);
                }

                Some(last_item)
            }
        }
    }

    pub fn push_pop_min(&mut self, mut item: T) -> Option<T> {
        if self.is_empty() || item < self.tree[0] {
            return Some(item);
        }

        std::mem::swap(&mut item, &mut self.tree[0]);

        self.push_down(0);

        return Some(item);
    }

    pub fn push_pop_max(&mut self, mut item: T) -> Option<T> {
        match self.size() {
            0 => Some(item),
            _ => {
                let max_index = self.find_max_index();

                if item > self.tree[max_index] {
                    Some(item)
                } else {
                    std::mem::swap(&mut item, &mut self.tree[max_index]);

                    if self.tree[max_index] < self.tree[0] {
                        self.tree.swap(max_index, 0);
                    }

                    self.push_down(max_index);

                    Some(item)
                }
            }
        }
    }

    pub fn replace_min(&mut self, mut item: T) -> Option<T> {
        if self.is_empty() {
            self.push(item);
            return None;
        }

        std::mem::swap(&mut item, &mut self.tree[0]);

        self.push_down(0);

        Some(item)
    }

    pub fn replace_max(&mut self, mut item: T) -> Option<T> {
        if self.is_empty() {
            self.push(item);
            return None;
        }

        let max_index = self.find_max_index();

        std::mem::swap(&mut item, &mut self.tree[max_index]);

        if self.tree[max_index] < self.tree[0] {
            self.tree.swap(max_index, 0);
        }

        self.push_down(max_index);

        Some(item)
    }
    fn find_max_index(&self) -> usize {
        match self.size() {
            0 => panic!("There is no item is the heap."),
            1 => 0,
            2 => 1,
            _ => {
                if self.tree[1] > self.tree[2] {
                    1
                } else {
                    2
                }
            }
        }
    }

    pub fn reserve(&mut self, additional: usize) {
        self.tree.reserve(additional);
    }

    pub fn reserve_exact(&mut self, additional: usize) {
        self.tree.reserve_exact(additional);
    }

    pub fn shrink_to_fit(&mut self) {
        self.tree.shrink_to_fit();
    }

    pub fn into_vec(self) -> Vec<T> {
        self.tree
    }

    pub fn size(&self) -> usize {
        self.tree.len()
    }

    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }

    pub fn clear(&mut self) {
        self.tree.clear()
    }

    pub fn capacity(&self) -> usize {
        self.tree.capacity()
    }
}

fn is_on_min_level(index: usize) -> bool {
    (((index + 1) as f32).log(2.0) as usize) % 2 == 0
}

fn has_grandparent(index: usize) -> bool {
    index > 2
}

// fn has_parent(index: usize) -> bool {
//     index > 0
// }

fn has_left_child(index: usize, size: usize) -> bool {
    (2 * (index + 1)) - 1 < size
}

fn has_right_child(index: usize, size: usize) -> bool {
    (2 * (index + 1)) < size
}

fn has_child(index: usize, size: usize) -> bool {
    has_left_child(index, size) || has_left_child(index, size)
}

fn left_child(index: usize) -> usize {
    (2 * (index + 1)) - 1
}

fn right_child(index: usize) -> usize {
    2 * (index + 1)
}

pub fn parent(index: usize) -> usize {
    (index - 1) / 2
}

pub fn grandparent(index: usize) -> usize {
    parent(parent(index))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heap_minmax_tree_is_on_min_level() {
        assert_eq!(is_on_min_level(0), true);

        assert_eq!(is_on_min_level(1), false);
        assert_eq!(is_on_min_level(2), false);

        assert_eq!(is_on_min_level(3), true);
        assert_eq!(is_on_min_level(4), true);
        assert_eq!(is_on_min_level(5), true);
        assert_eq!(is_on_min_level(6), true);

        assert_eq!(is_on_min_level(7), false);
    }

    #[test]
    fn heap_minmax_tree_has_grandparent() {
        assert_eq!(has_grandparent(0), false);
        assert_eq!(has_grandparent(1), false);
        assert_eq!(has_grandparent(2), false);
        assert_eq!(has_grandparent(3), true);
    }

    #[test]
    fn heap_minmax_tree_parent() {
        assert_eq!(parent(1), 0);
        assert_eq!(parent(2), 0);
        assert_eq!(parent(3), 1);
        assert_eq!(parent(4), 1);
        assert_eq!(parent(5), 2);
        assert_eq!(parent(6), 2);
        assert_eq!(parent(7), 3);
        assert_eq!(parent(8), 3);
        assert_eq!(parent(9), 4);
        assert_eq!(parent(10), 4);
        assert_eq!(parent(11), 5);
        assert_eq!(parent(12), 5);
        assert_eq!(parent(13), 6);
        assert_eq!(parent(14), 6);
    }

    #[test]
    fn heap_minmax_tree_grandparent() {
        assert_eq!(grandparent(3), 0);
        assert_eq!(grandparent(4), 0);
        assert_eq!(grandparent(5), 0);
        assert_eq!(grandparent(6), 0);
        assert_eq!(grandparent(7), 1);
        assert_eq!(grandparent(8), 1);
        assert_eq!(grandparent(9), 1);
        assert_eq!(grandparent(10), 1);
        assert_eq!(grandparent(11), 2);
        assert_eq!(grandparent(12), 2);
        assert_eq!(grandparent(13), 2);
        assert_eq!(grandparent(14), 2);
    }

    #[test]
    fn heap_minmax_tree_has_left_child() {
        assert_eq!(has_left_child(0, 6), true);
        assert_eq!(has_left_child(1, 6), true);
        assert_eq!(has_left_child(2, 6), true);
        assert_eq!(has_left_child(3, 6), false);
        assert_eq!(has_left_child(4, 6), false);
        assert_eq!(has_left_child(5, 6), false);
    }

    #[test]
    fn heap_minmax_tree_has_right_child() {
        assert_eq!(has_right_child(0, 6), true);
        assert_eq!(has_right_child(1, 6), true);
        assert_eq!(has_right_child(2, 6), false);
        assert_eq!(has_right_child(3, 6), false);
        assert_eq!(has_right_child(4, 6), false);
        assert_eq!(has_right_child(5, 6), false);
    }

    #[test]
    fn heap_minmax_tree_has_child() {
        assert_eq!(has_child(0, 6), true);
        assert_eq!(has_child(1, 6), true);
        assert_eq!(has_child(2, 6), true);
        assert_eq!(has_child(3, 6), false);
        assert_eq!(has_child(4, 6), false);
        assert_eq!(has_child(5, 6), false);
    }

    #[test]
    fn heap_minmax_tree_left_child() {
        assert_eq!(left_child(0), 1);
        assert_eq!(left_child(1), 3);
        assert_eq!(left_child(2), 5);
        assert_eq!(left_child(3), 7);
        assert_eq!(left_child(4), 9);
        assert_eq!(left_child(5), 11);
    }

    #[test]
    fn heap_minmax_tree_right_child() {
        assert_eq!(right_child(0), 2);
        assert_eq!(right_child(1), 4);
        assert_eq!(right_child(2), 6);
        assert_eq!(right_child(3), 8);
        assert_eq!(right_child(4), 10);
        assert_eq!(right_child(5), 12);
    }

    #[test]
    fn heap_minmax_init() {
        let minmax: MinMax<usize> = MinMax::init();

        assert_eq!(minmax.size(), 0);
        assert_eq!(minmax.is_empty(), true);
        assert_eq!(minmax.capacity(), 0);
    }

    #[test]
    fn heap_minmax_with_capacity() {
        let minmax: MinMax<usize> = MinMax::with_capacity(10);

        assert_eq!(minmax.is_empty(), true);
        assert_eq!(minmax.size(), 0);
        assert_eq!(minmax.capacity(), 10);
    }

    #[test]
    fn heap_minmax_smallest_child_or_grandchild_1() {
        let mut minmax: MinMax<usize> = MinMax::init();

        minmax.tree = vec![0, 5, 4, 3, 4, 1];

        assert_eq!(minmax.smallest_child_or_grandchild(0), (5, true));
        assert_eq!(minmax.smallest_child_or_grandchild(1), (3, false));
        assert_eq!(minmax.smallest_child_or_grandchild(2), (5, false));
        assert_eq!(minmax.smallest_child_or_grandchild(3), (3, false));
        assert_eq!(minmax.smallest_child_or_grandchild(4), (4, false));
        assert_eq!(minmax.smallest_child_or_grandchild(5), (5, false));
    }

    #[test]
    fn heap_minmax_greatest_child_or_grandchild_1() {
        let mut minmax: MinMax<usize> = MinMax::init();

        minmax.tree = vec![0, 5, 4, 3, 4, 1];

        assert_eq!(minmax.greatest_child_or_grandchild(0), (1, false));
        assert_eq!(minmax.greatest_child_or_grandchild(1), (4, false));
        assert_eq!(minmax.greatest_child_or_grandchild(2), (5, false));
        assert_eq!(minmax.greatest_child_or_grandchild(3), (3, false));
        assert_eq!(minmax.greatest_child_or_grandchild(4), (4, false));
        assert_eq!(minmax.greatest_child_or_grandchild(5), (5, false));
    }

    #[test]
    fn heap_minmax_push_down_1() {
        let mut minmax: MinMax<usize> = MinMax::init();

        minmax.tree = vec![0, 5, 4, 3, 4, 1];

        minmax.tree[0] = 6;
        minmax.push_down(0);
        assert_eq!(
            format!("{:?}", minmax.tree),
            String::from("[1, 5, 6, 3, 4, 4]")
        );

        minmax.tree[0] = 7;
        minmax.push_down(0);
        assert_eq!(
            format!("{:?}", minmax.tree),
            String::from("[3, 7, 6, 5, 4, 4]")
        );
    }

    #[test]
    fn heap_minmax_build_heap_1() {
        let minmax = MinMax::build_heap(vec![1, 3, 0, 6, 8, 2, 4]);

        assert_eq!(minmax.size(), 7);
        assert_eq!(
            format!("{:?}", minmax.tree),
            String::from("[0, 8, 4, 6, 3, 2, 1]")
        );
    }

    #[test]
    fn heap_minmax_build_heap_2() {
        let minmax = MinMax::build_heap(vec![9, 8, 2, 3, 4, 5, 11, 6, 7, 0]);

        assert_eq!(minmax.size(), 10);
        assert_eq!(
            format!("{:?}", minmax.tree),
            String::from("[0, 9, 11, 3, 4, 5, 2, 6, 7, 8]")
        );
    }

    #[test]
    fn heap_minmax_peek_min_1() {
        let minmax: MinMax<usize> = MinMax::init();

        assert_eq!(minmax.peek_min(), None);
    }

    #[test]
    fn heap_minmax_peek_min_2() {
        let mut minmax: MinMax<usize> = MinMax::init();

        minmax.push(0);

        assert_eq!(*minmax.peek_min().unwrap(), 0);
    }

    #[test]
    fn heap_minmax_peek_min_3() {
        let mut minmax: MinMax<usize> = MinMax::init();

        minmax.push(10);
        minmax.push(0);

        assert_eq!(*minmax.peek_min().unwrap(), 0);
    }

    #[test]
    fn heap_minmax_peek_max_1() {
        let minmax: MinMax<usize> = MinMax::init();

        assert_eq!(minmax.peek_max(), None);
    }

    #[test]
    fn heap_minmax_peek_max_2() {
        let mut minmax: MinMax<usize> = MinMax::init();

        minmax.push(0);

        assert_eq!(*minmax.peek_max().unwrap(), 0);
    }

    #[test]
    fn heap_minmax_peek_max_3() {
        let mut minmax: MinMax<usize> = MinMax::init();

        minmax.push(10);
        minmax.push(0);

        assert_eq!(*minmax.peek_max().unwrap(), 10);
    }

    #[test]
    fn heap_minmax_peek_max_4() {
        let mut minmax: MinMax<usize> = MinMax::init();

        minmax.push(10);
        minmax.push(0);
        minmax.push(20);

        assert_eq!(*minmax.peek_max().unwrap(), 20);
    }

    #[test]
    fn heap_minmax_push_1() {
        let mut minmax: MinMax<usize> = MinMax::init();

        minmax.push(10);

        assert_eq!(*minmax.peek_min().unwrap(), 10);
        assert_eq!(*minmax.peek_max().unwrap(), 10);
    }

    #[test]
    fn heap_minmax_push_2() {
        let mut minmax: MinMax<usize> = MinMax::init();

        minmax.push(10);
        minmax.push(5);
        minmax.push(1);
        minmax.push(4);
        minmax.push(3);
        minmax.push(12);

        assert_eq!(*minmax.peek_min().unwrap(), 1);
        assert_eq!(*minmax.peek_max().unwrap(), 12);
    }

    #[test]
    fn heap_minmax_pop_min() {
        let mut minmax = MinMax::build_heap(vec![9, 8, 2, 3, 4, 5, 11, 6, 7, 0]);

        assert_eq!(*minmax.peek_min().unwrap(), 0);

        minmax.pop_min();
        assert_eq!(*minmax.peek_min().unwrap(), 2);

        minmax.pop_min();
        assert_eq!(*minmax.peek_min().unwrap(), 3);

        minmax.pop_min();
        assert_eq!(*minmax.peek_min().unwrap(), 4);

        minmax.pop_min();
        assert_eq!(*minmax.peek_min().unwrap(), 5);

        minmax.pop_min();
        assert_eq!(*minmax.peek_min().unwrap(), 6);

        minmax.pop_min();
        assert_eq!(*minmax.peek_min().unwrap(), 7);

        minmax.pop_min();
        assert_eq!(*minmax.peek_min().unwrap(), 8);

        minmax.pop_min();
        assert_eq!(*minmax.peek_min().unwrap(), 9);

        minmax.pop_min();
        assert_eq!(*minmax.peek_min().unwrap(), 11);

        minmax.pop_min();
        assert_eq!(minmax.peek_min(), None);
    }

    #[test]
    fn heap_minmax_pop_max() {
        let mut minmax = MinMax::build_heap(vec![9, 8, 2, 3, 4, 5, 11, 6, 7, 0]);

        assert_eq!(*minmax.peek_max().unwrap(), 11);

        minmax.pop_max();
        assert_eq!(*minmax.peek_max().unwrap(), 9);

        minmax.pop_max();
        assert_eq!(*minmax.peek_max().unwrap(), 8);

        minmax.pop_max();
        assert_eq!(*minmax.peek_max().unwrap(), 7);

        minmax.pop_max();
        assert_eq!(*minmax.peek_max().unwrap(), 6);

        minmax.pop_max();
        assert_eq!(*minmax.peek_max().unwrap(), 5);

        minmax.pop_max();
        assert_eq!(*minmax.peek_max().unwrap(), 4);

        minmax.pop_max();
        assert_eq!(*minmax.peek_max().unwrap(), 3);

        minmax.pop_max();
        assert_eq!(*minmax.peek_max().unwrap(), 2);

        minmax.pop_max();
        assert_eq!(*minmax.peek_max().unwrap(), 0);

        minmax.pop_max();
        assert_eq!(minmax.peek_max(), None);
    }

    #[test]
    fn heap_minmax_push_pop_min_1() {
        let mut minmax = MinMax::init();

        assert_eq!(minmax.push_pop_min(1).unwrap(), 1);
    }

    #[test]
    fn heap_minmax_push_pop_min_2() {
        let mut minmax = MinMax::init();
        minmax.push(2);

        assert_eq!(minmax.push_pop_min(1).unwrap(), 1);
    }

    #[test]
    fn heap_minmax_push_pop_min_3() {
        let mut minmax = MinMax::build_heap(vec![9, 8, 2, 3, 4, 5, 11, 6, 7, 0]);

        assert_eq!(minmax.push_pop_min(13).unwrap(), 0);
        assert_eq!(*minmax.peek_min().unwrap(), 2);
        assert_eq!(*minmax.peek_max().unwrap(), 13);
    }

    #[test]
    fn heap_minmax_push_pop_max_1() {
        let mut minmax = MinMax::init();

        assert_eq!(minmax.push_pop_max(1).unwrap(), 1);
    }

    #[test]
    fn heap_minmax_push_pop_max_2() {
        let mut minmax = MinMax::init();
        minmax.push(2);

        assert_eq!(minmax.push_pop_max(3).unwrap(), 3);
    }

    #[test]
    fn heap_minmax_push_pop_max_3() {
        let mut minmax = MinMax::build_heap(vec![9, 8, 2, 3, 4, 5, 11, 6, 7, 1]);

        assert_eq!(minmax.push_pop_max(0).unwrap(), 11);
        assert_eq!(*minmax.peek_min().unwrap(), 0);
        assert_eq!(*minmax.peek_max().unwrap(), 9);
    }

    #[test]
    fn heap_minmax_replace_min_1() {
        let mut minmax: MinMax<usize> = MinMax::build_heap(vec![]);

        assert_eq!(minmax.replace_min(0), None);
        assert_eq!(*minmax.peek_min().unwrap(), 0);
        assert_eq!(*minmax.peek_max().unwrap(), 0);
    }

    #[test]
    fn heap_minmax_replace_min_2() {
        let mut minmax: MinMax<usize> = MinMax::build_heap(vec![3, 2, 1]);

        assert_eq!(minmax.replace_min(4).unwrap(), 1);
        assert_eq!(*minmax.peek_min().unwrap(), 2);
        assert_eq!(*minmax.peek_max().unwrap(), 4);
    }

    #[test]
    fn heap_minmax_replace_max_1() {
        let mut minmax: MinMax<usize> = MinMax::build_heap(vec![]);

        assert_eq!(minmax.replace_max(0), None);
        assert_eq!(*minmax.peek_min().unwrap(), 0);
        assert_eq!(*minmax.peek_max().unwrap(), 0);
    }

    #[test]
    fn heap_minmax_replace_max_2() {
        let mut minmax: MinMax<usize> = MinMax::build_heap(vec![3, 2, 1]);

        assert_eq!(minmax.replace_max(4).unwrap(), 3);
        assert_eq!(*minmax.peek_min().unwrap(), 1);
        assert_eq!(*minmax.peek_max().unwrap(), 4);
    }

    #[test]
    fn heap_minmax_replace_max_3() {
        let mut minmax: MinMax<usize> = MinMax::build_heap(vec![3, 2, 1]);

        assert_eq!(minmax.replace_max(0).unwrap(), 3);
        assert_eq!(*minmax.peek_min().unwrap(), 0);
        assert_eq!(*minmax.peek_max().unwrap(), 2);
    }
}
