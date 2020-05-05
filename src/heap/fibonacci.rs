use std::collections::LinkedList;

#[derive(Debug)]
struct InternalTree<T: std::cmp::Ord> {
    degree: usize,
    payload: Option<T>,
    children_list: LinkedList<InternalTree<T>>,
}

impl<T: std::cmp::Ord> InternalTree<T> {
    fn init(payload: T) -> InternalTree<T> {
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

    fn merge(
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
        self.children_list.push_front(internal_tree);
        self.degree += 1;
    }

    fn degree(&self) -> usize {
        self.degree
    }

    fn peek_payload(&self) -> &Option<T> {
        &self.payload
    }

    fn children_list_mut(&mut self) -> &mut LinkedList<InternalTree<T>> {
        &mut self.children_list
    }

    fn children_list(&self) -> & LinkedList<InternalTree<T>> {
        &self.children_list
    }
}

impl<T> InternalTree<T>
where
    T: std::cmp::Ord + std::fmt::Display,
{
    fn preorder(internal_tree: &InternalTree<T>) -> String {
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
                    if !item.peek_payload().is_none() {
                        node_list.push_str(format!("{} ", InternalTree::_preorder(&Some(&item))).as_str());
                    }
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

        println!("{}", InternalTree::preorder(&merged_tree));
    }
}

// ------------- Fibonacci Heap -------------
#[derive(Debug)]
pub struct FibonacciHeap<T: std::cmp::Ord> {
    children_list: LinkedList<InternalTree<T>>,
    size: usize,
}

impl<T: std::cmp::Ord> FibonacciHeap<T> {
    pub fn init() -> FibonacciHeap<T> {
        FibonacciHeap {
            children_list: LinkedList::new(),
            size: 0,
        }
    }
}
