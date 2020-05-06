use std::collections::LinkedList;

#[derive(Debug)]
pub struct InternalTree<T: std::cmp::Ord> {
    degree: usize,
    payload: Option<T>,
    children_list: LinkedList<InternalTree<T>>,
}

impl<T: std::cmp::Ord> InternalTree<T> {
    pub fn init(payload: T) -> InternalTree<T> {
        InternalTree {
            degree: 0,
            payload: Some(payload),
            children_list: LinkedList::new(),
        }
    }

    pub fn is_smaller_or_equal(
        internal_tree_1: &InternalTree<T>,
        internal_tree_2: &InternalTree<T>,
    ) -> bool {
        match (
            internal_tree_1.peek_payload(),
            internal_tree_2.peek_payload(),
        ) {
            (Some(payload1), Some(payload2)) => payload1 <= payload2,
            _ => panic!("Payloads can not be empty"),
        }
    }

    pub fn merge(
        mut internal_tree_1: InternalTree<T>,
        mut internal_tree_2: InternalTree<T>,
    ) -> InternalTree<T> {
        if InternalTree::is_smaller_or_equal(&internal_tree_1, &internal_tree_2) {
            internal_tree_1.add_child(internal_tree_2);

            internal_tree_1
        } else {
            internal_tree_2.add_child(internal_tree_1);

            internal_tree_2
        }
    }

    fn add_child(&mut self, internal_tree: InternalTree<T>) {
        self.children_list.push_back(internal_tree);
        self.degree += 1;
    }

    pub fn degree(&self) -> usize {
        self.degree
    }

    pub fn peek_payload(&self) -> &Option<T> {
        &self.payload
    }

    pub fn get_payload(&mut self) -> T {
        if self.payload.is_none() {
            panic!("Payload is None");
        }

        self.payload.take().unwrap()
    }

    pub fn children_list_mut(&mut self) -> &mut LinkedList<InternalTree<T>> {
        &mut self.children_list
    }

    pub fn children_list(&self) -> &LinkedList<InternalTree<T>> {
        &self.children_list
    }
}

impl<T> InternalTree<T>
where
    T: std::cmp::Ord + std::fmt::Display,
{
    pub fn preorder(internal_tree: &InternalTree<T>) -> String {
        return String::from(InternalTree::_preorder(&Some(internal_tree)).trim());
    }

    fn _preorder(node_opt: &Option<&InternalTree<T>>) -> String {
        let mut node_list = String::from("");

        match node_opt {
            None => node_list,
            Some(node) => {
                match node.peek_payload() {
                    Some(value) => node_list.push_str(format!("{} ", value).as_str()),
                    None => (),
                }
                for item in node.children_list() {
                    node_list
                        .push_str(format!("{}", InternalTree::_preorder(&Some(&item))).as_str());
                }
                node_list
            }
        }
    }
}

#[cfg(test)]
mod internal_tree_tests {
    use super::*;

    #[test]
    fn heap_fibonacci_internal_tree_init() {
        let it = InternalTree::init(1);

        assert_eq!(it.degree(), 0);
        assert_eq!(*it.peek_payload(), Some(1));
    }

    #[test]
    fn heap_fibonacci_internal_tree_is_smaller() {
        let it1 = InternalTree::init(0);
        let it2 = InternalTree::init(1);
        let it3 = InternalTree::init(0);

        assert_eq!(InternalTree::is_smaller_or_equal(&it1, &it2), true);
        assert_eq!(InternalTree::is_smaller_or_equal(&it1, &it3), true);
        assert_eq!(InternalTree::is_smaller_or_equal(&it2, &it1), false);
    }

    #[test]
    fn heap_fibonacci_internal_tree_add_child_1() {
        let mut it1 = InternalTree::init(0);
        let it2 = InternalTree::init(1);

        it1.add_child(it2);

        assert_eq!(it1.degree(), 1);
        assert_eq!(
            *it1.children_list.pop_back().unwrap().peek_payload(),
            Some(1)
        );
    }

    #[test]
    fn heap_fibonacci_internal_tree_add_child_2() {
        let it1 = InternalTree::init(0);
        let mut it2 = InternalTree::init(1);

        it2.add_child(it1);

        assert_eq!(it2.degree(), 1);
        assert_eq!(
            *it2.children_list.pop_back().unwrap().peek_payload(),
            Some(0)
        );
    }

    #[test]
    fn heap_fibonacci_internal_tree_merge_1() {
        let it1 = InternalTree::init(0);
        let it2 = InternalTree::init(1);

        let mut merged_tree = InternalTree::merge(it1, it2);

        assert_eq!(merged_tree.degree(), 1);
        assert_eq!(
            *merged_tree.children_list.pop_back().unwrap().peek_payload(),
            Some(1)
        );
    }

    #[test]
    fn heap_fibonacci_internal_tree_merge_2() {
        let it1 = InternalTree::init(0);
        let it2 = InternalTree::init(1);

        let mut merged_tree = InternalTree::merge(it2, it1);

        assert_eq!(merged_tree.degree(), 1);
        assert_eq!(
            *merged_tree.children_list.pop_back().unwrap().peek_payload(),
            Some(1)
        );
    }

    #[test]
    fn heap_fibonacci_internal_tree_merge_3() {
        let it1 = InternalTree::init(0);
        let it2 = InternalTree::init(1);
        let merged_tree_1 = InternalTree::merge(it2, it1);
        let it3 = InternalTree::init(2);
        let it4 = InternalTree::init(3);
        let merged_tree_2 = InternalTree::merge(it3, it4);

        let merged_tree = InternalTree::merge(merged_tree_1, merged_tree_2);

        assert_eq!(merged_tree.degree(), 2);
        assert_eq!(
            InternalTree::preorder(&merged_tree),
            String::from("0 1 2 3")
        );
    }
}

// ------------- Fibonacci Heap -------------
#[derive(Debug)]
pub struct FibonacciHeap<T: std::cmp::Ord> {
    children_list: LinkedList<InternalTree<T>>,
    size: usize,
    min_pointer: Option<InternalTree<T>>,
}

impl<T: std::cmp::Ord> FibonacciHeap<T> {
    pub fn init() -> FibonacciHeap<T> {
        FibonacciHeap {
            children_list: LinkedList::new(),
            size: 0,
            min_pointer: None,
        }
    }

    pub fn push(&mut self, payload: T) {
        let new_node = InternalTree::init(payload);

        if self.min_pointer.is_none() {
            self.min_pointer = Some(new_node);
        } else {
            if InternalTree::is_smaller_or_equal(&new_node, &self.min_pointer.as_ref().unwrap()) {
                let temp = self.min_pointer.take().unwrap();
                self.min_pointer = Some(new_node);
                self.children_list.push_back(temp);
            } else {
                self.children_list.push_back(new_node);
            }
        }

        self.size += 1;
    }

    pub fn merge(
        mut fibonacci_heap_1: FibonacciHeap<T>,
        mut fibonacci_heap_2: FibonacciHeap<T>,
    ) -> FibonacciHeap<T> {
        fibonacci_heap_1
            .children_list
            .append(&mut fibonacci_heap_2.children_list);

        if InternalTree::is_smaller_or_equal(
            &fibonacci_heap_2.min_pointer.as_ref().unwrap(),
            &fibonacci_heap_1.min_pointer.as_ref().unwrap(),
        ) {
            let temp = fibonacci_heap_1.min_pointer.take().unwrap();
            fibonacci_heap_1.min_pointer = fibonacci_heap_2.min_pointer.take();
            fibonacci_heap_1.children_list.push_back(temp);

            fibonacci_heap_1.size += fibonacci_heap_2.size;
        } else {
            fibonacci_heap_1.push(fibonacci_heap_2.min_pointer.unwrap().get_payload());

            fibonacci_heap_1.size += fibonacci_heap_2.size - 1;
        }

        fibonacci_heap_1
    }
}

impl<T> FibonacciHeap<T>
where
    T: std::cmp::Ord + std::fmt::Display,
{
    pub fn preorder(fibonacci_heap: &FibonacciHeap<T>) -> String {
        let mut node_list = String::from("");

        if !fibonacci_heap.min_pointer.is_none() {
            node_list.push_str(
                format!(
                    "Min: {}\n",
                    InternalTree::preorder(&fibonacci_heap.min_pointer.as_ref().unwrap())
                )
                .as_str(),
            );
        }

        for (index, internal_tree) in fibonacci_heap.children_list.iter().enumerate() {
            node_list.push_str(format!("Tree {}: ", index + 1).as_str());
            node_list.push_str(InternalTree::preorder(&internal_tree).as_str());

            node_list.push_str("\n");
        }

        node_list
    }
}

#[cfg(test)]
mod fibonacci_heap_tests {
    use super::*;

    #[test]
    fn heap_fibonacci_init() {
        let fh: FibonacciHeap<usize> = FibonacciHeap::init();

        assert!(fh.min_pointer.is_none());
        assert_eq!(fh.size, 0);
        assert_eq!(fh.children_list.len(), 0);
    }

    #[test]
    fn heap_fibonacci_push_1() {
        let mut fh: FibonacciHeap<usize> = FibonacciHeap::init();

        fh.push(0);
        fh.push(1);
        fh.push(3);

        assert_eq!(fh.children_list.len(), 2);
        assert_eq!(fh.min_pointer.as_ref().unwrap().peek_payload().unwrap(), 0);

        assert_eq!(
            FibonacciHeap::preorder(&fh),
            String::from("Min: 0\nTree 1: 1\nTree 2: 3\n")
        )
    }

    #[test]
    fn heap_fibonacci_push_2() {
        let mut fh: FibonacciHeap<usize> = FibonacciHeap::init();

        fh.push(3);
        fh.push(1);
        fh.push(0);

        assert_eq!(fh.children_list.len(), 2);
        assert_eq!(fh.min_pointer.as_ref().unwrap().peek_payload().unwrap(), 0);

        assert_eq!(
            FibonacciHeap::preorder(&fh),
            String::from("Min: 0\nTree 1: 3\nTree 2: 1\n")
        )
    }

    #[test]
    fn heap_fibonacci_merge_1() {
        let mut fh1: FibonacciHeap<usize> = FibonacciHeap::init();
        fh1.push(0);

        let mut fh2: FibonacciHeap<usize> = FibonacciHeap::init();
        fh2.push(1);

        let merged_heap = FibonacciHeap::merge(fh1, fh2);

        assert_eq!(merged_heap.size, 2);
        assert_eq!(
            merged_heap
                .min_pointer
                .as_ref()
                .unwrap()
                .peek_payload()
                .unwrap(),
            0
        );
        assert_eq!(
            FibonacciHeap::preorder(&merged_heap),
            String::from("Min: 0\nTree 1: 1\n")
        );
    }

    #[test]
    fn heap_fibonacci_merge_2() {
        let mut fh1: FibonacciHeap<usize> = FibonacciHeap::init();
        fh1.push(0);
        fh1.push(2);

        let mut fh2: FibonacciHeap<usize> = FibonacciHeap::init();
        fh2.push(1);
        fh2.push(3);

        let merged_heap = FibonacciHeap::merge(fh2, fh1);

        assert_eq!(merged_heap.size, 4);
        assert_eq!(
            merged_heap
                .min_pointer
                .as_ref()
                .unwrap()
                .peek_payload()
                .unwrap(),
            0
        );
        assert_eq!(
            FibonacciHeap::preorder(&merged_heap),
            String::from("Min: 0\nTree 1: 3\nTree 2: 2\nTree 3: 1\n")
        );
    }
}
