use std::collections::VecDeque;

#[derive(Debug)]
pub struct InternalTree<T: std::cmp::Ord> {
    // number of direct children of the current node
    degree: usize,

    // data stored in the current node
    payload: T,

    // children of the current node
    children_list: VecDeque<InternalTree<T>>,

    // indicates wether current node is a min heap-ordered tree or not
    min: bool,
}

impl<T: std::cmp::Ord> InternalTree<T> {
    // initializes an internal tree which is min or max heap ordered tree based on min parameter
    fn init(payload: T, min: bool) -> InternalTree<T> {
        InternalTree {
            degree: 0,
            payload,
            children_list: VecDeque::new(),
            min,
        }
    }

    // returns true if tree1.payload <= tree2.payload
    #[inline]
    fn is_smaller_or_equal(
        internal_tree_1: &InternalTree<T>,
        internal_tree_2: &InternalTree<T>,
    ) -> bool {
        let payload1 = internal_tree_1.peek_payload();
        let payload2 = internal_tree_2.peek_payload();
        payload1 <= payload2
    }
    // returns true if tree1.payload >= tree2.payload
    #[inline]
    fn is_greater_or_equal(
        internal_tree_1: &InternalTree<T>,
        internal_tree_2: &InternalTree<T>,
    ) -> bool {
        let payload1 = internal_tree_1.peek_payload();
        let payload2 = internal_tree_2.peek_payload();
        payload1 >= payload2
    }

    // returns true if tree1 has higher priority than tree2
    // it means:
    // if trees are min heap-ordered higher priority means smaller values
    // if trees are max heap-ordered higher priority means larger values
    fn has_higher_priority(
        internal_tree_1: &InternalTree<T>,
        internal_tree_2: &InternalTree<T>,
        is_min: bool,
    ) -> bool {
        if is_min {
            InternalTree::is_smaller_or_equal(&internal_tree_1, &internal_tree_2)
        } else {
            InternalTree::is_greater_or_equal(&internal_tree_1, &internal_tree_2)
        }
    }

    // merges two heap-ordered trees and returns the merged tree
    fn merge(
        mut internal_tree_1: InternalTree<T>,
        mut internal_tree_2: InternalTree<T>,
    ) -> InternalTree<T> {
        // make sure both tree are of the same kind
        if internal_tree_1.is_min() != internal_tree_2.is_min() {
            panic!("Both internal trees must be of same type. Both min or both max")
        }

        let trees_are_min = internal_tree_1.is_min();

        // tree with lower priority must be child of the tree with higher priority
        if InternalTree::has_higher_priority(&internal_tree_1, &internal_tree_2, trees_are_min) {
            internal_tree_1.add_child(internal_tree_2);

            internal_tree_1
        } else {
            internal_tree_2.add_child(internal_tree_1);

            internal_tree_2
        }
    }

    // add another internal tree as a child
    fn add_child(&mut self, internal_tree: InternalTree<T>) {
        self.children_list.push_back(internal_tree);
        self.degree += 1;
    }

    // returns degree of current tree
    fn degree(&self) -> usize {
        self.degree
    }

    // returns a reference to root payload of current tree
    fn peek_payload(&self) -> &T {
        &self.payload
    }

    // returns a reference to list of children of the current node
    fn children_list(&self) -> &VecDeque<InternalTree<T>> {
        &self.children_list
    }

    // returns true if tree is initialized as a min heap-ordered tree
    fn is_min(&self) -> bool {
        self.min
    }
}

impl<T> InternalTree<T>
where
    T: std::cmp::Ord + std::fmt::Display,
{
    // It's like preorder function of Binomial Heap
    pub fn preorder(internal_tree: &InternalTree<T>) -> String {
        return String::from(InternalTree::_preorder(&Some(internal_tree)).trim());
    }

    fn _preorder(node_opt: &Option<&InternalTree<T>>) -> String {
        let mut node_list = String::from("");

        match node_opt {
            None => node_list,
            Some(node) => {
                let payload = node.peek_payload();
                node_list.push_str(format!("{} ", payload).as_str());
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
        let it = InternalTree::init(1, true);

        assert_eq!(it.degree(), 0);
        assert_eq!(*it.peek_payload(), 1);
    }

    #[test]
    fn heap_fibonacci_internal_tree_is_smaller() {
        let it1 = InternalTree::init(0, true);
        let it2 = InternalTree::init(1, true);
        let it3 = InternalTree::init(0, true);

        assert_eq!(InternalTree::is_smaller_or_equal(&it1, &it2), true);
        assert_eq!(InternalTree::is_smaller_or_equal(&it1, &it3), true);
        assert_eq!(InternalTree::is_smaller_or_equal(&it2, &it1), false);
    }

    #[test]
    fn heap_fibonacci_internal_tree_add_child_1() {
        let mut it1 = InternalTree::init(0, true);
        let it2 = InternalTree::init(1, true);

        it1.add_child(it2);

        assert_eq!(it1.degree(), 1);
        assert_eq!(
            *it1.children_list.pop_back().unwrap().peek_payload(),
            1
        );
    }

    #[test]
    fn heap_fibonacci_internal_tree_add_child_2() {
        let it1 = InternalTree::init(0, true);
        let mut it2 = InternalTree::init(1, true);

        it2.add_child(it1);

        assert_eq!(it2.degree(), 1);
        assert_eq!(
            *it2.children_list.pop_back().unwrap().peek_payload(),
            0
        );
    }

    #[test]
    fn heap_fibonacci_internal_tree_merge_1() {
        let it1 = InternalTree::init(0, true);
        let it2 = InternalTree::init(1, true);

        let mut merged_tree = InternalTree::merge(it1, it2);

        assert_eq!(merged_tree.degree(), 1);
        assert_eq!(
            *merged_tree.children_list.pop_back().unwrap().peek_payload(),
            1
        );
    }

    #[test]
    fn heap_fibonacci_internal_tree_merge_2() {
        let it1 = InternalTree::init(0, true);
        let it2 = InternalTree::init(1, true);

        let mut merged_tree = InternalTree::merge(it2, it1);

        assert_eq!(merged_tree.degree(), 1);
        assert_eq!(
            *merged_tree.children_list.pop_back().unwrap().peek_payload(),
            1
        );
    }

    #[test]
    fn heap_fibonacci_internal_tree_merge_3() {
        let it1 = InternalTree::init(0, true);
        let it2 = InternalTree::init(1, true);
        let merged_tree_1 = InternalTree::merge(it2, it1);
        let it3 = InternalTree::init(2, true);
        let it4 = InternalTree::init(3, true);
        let merged_tree_2 = InternalTree::merge(it3, it4);

        let merged_tree = InternalTree::merge(merged_tree_1, merged_tree_2);

        assert_eq!(merged_tree.degree(), 2);
        assert_eq!(
            InternalTree::preorder(&merged_tree),
            String::from("0 1 2 3")
        );
    }

    #[test]
    #[should_panic(expected = "Both internal trees must be of same type. Both min or both max")]
    fn heap_fibonacci_internal_tree_panic_merge() {
        let it1 = InternalTree::init(0, true);
        let it2 = InternalTree::init(1, false);

        InternalTree::merge(it1, it2);
    }
}

// ------------- Fibonacci Heap -------------
/// A Fibonacci heap is a data structure for priority queue operations.
/// It has a better amortized running time than binary heap and binomial heap.
///
/// # Examples
/// ```
/// use rudac::heap::FibonacciHeap;
///
/// // initialize a fibonacci heap
/// let mut fibonacci_heap: FibonacciHeap<usize> = FibonacciHeap::init_min();
///
/// // push items into heap
/// fibonacci_heap.push(0);
/// fibonacci_heap.push(1);
/// fibonacci_heap.push(3);
///
/// // heap will have the shape:
/// //  min
/// //   |
/// //   0 <-> 1 <-> 2
/// assert_eq!(
///     FibonacciHeap::preorder(&fibonacci_heap),
///     String::from("Priority: 0\nTree 1: 1\nTree 2: 3\n")
/// )
/// ```
#[derive(Debug)]
pub struct FibonacciHeap<T: std::cmp::Ord> {
    // doubly linked list of internal trees
    children_list: VecDeque<InternalTree<T>>,

    // total number of items in the heap
    size: usize,

    // pointer to root containing the highest priority
    priority_pointer: Option<InternalTree<T>>,

    // indicates wether current heap is initialized as a min heap or not
    min: bool,
}

impl<T: std::cmp::Ord> FibonacciHeap<T> {
    // initializes a fibonacci heap
    fn init(min: bool) -> FibonacciHeap<T> {
        FibonacciHeap {
            children_list: VecDeque::new(),
            size: 0,
            priority_pointer: None,
            min,
        }
    }

    /// Initializes a min heap with the specified `payload`
    ///
    /// # Examples
    /// ```
    /// use rudac::heap::FibonacciHeap;
    ///
    /// let fibonacci_heap: FibonacciHeap<usize> = FibonacciHeap::init_min();
    ///
    /// assert_eq!(fibonacci_heap.is_min(), true);
    /// ```
    pub fn init_min() -> FibonacciHeap<T> {
        FibonacciHeap::init(true)
    }

    /// Initializes a max heap with the specified `payload`
    ///
    /// # Examples
    /// ```
    /// use rudac::heap::FibonacciHeap;
    ///
    /// let fibonacci_heap: FibonacciHeap<usize> = FibonacciHeap::init_max();
    ///
    /// assert_eq!(fibonacci_heap.is_max(), true);
    /// ```
    pub fn init_max() -> FibonacciHeap<T> {
        FibonacciHeap::init(false)
    }

    /// Pushes specified `payload` into heap
    ///
    /// # Arguments:
    /// * `payload`: data to be pushed into heap
    ///
    /// # Examples
    /// ```
    /// use rudac::heap::FibonacciHeap;
    ///
    /// let mut fibonacci_heap: FibonacciHeap<usize> = FibonacciHeap::init_min();
    ///
    /// // push items into heap
    /// fibonacci_heap.push(0);
    /// fibonacci_heap.push(1);
    /// fibonacci_heap.push(3);
    ///
    /// // heap will have the shape:
    /// //  min
    /// //   |
    /// //   0 <-> 1 <-> 2
    /// assert_eq!(
    ///     FibonacciHeap::preorder(&fibonacci_heap),
    ///     String::from("Priority: 0\nTree 1: 1\nTree 2: 3\n")
    /// )
    /// ```
    pub fn push(&mut self, payload: T) {
        // create a compatible root with current heap, containing the payload
        let new_node = InternalTree::init(payload, self.is_min());

        let heap_is_min = self.is_min();

        // if there is no priority node, assign the newly created node as priority node
        if self.priority_pointer.is_none() {
            self.priority_pointer = Some(new_node);
        } else {
            if InternalTree::has_higher_priority(
                // if new node has higher priority, it must become priority node
                &new_node,
                &self.priority_pointer.as_ref().unwrap(),
                heap_is_min,
            ) {
                // swap new node and priority node
                let temp = self.priority_pointer.take().unwrap();
                self.priority_pointer = Some(new_node);
                self.children_list.push_back(temp);
            } else {
                // if new node has lower priority, just add it to children list of the heap
                self.children_list.push_back(new_node);
            }
        }

        // account for newly added node
        self.size += 1;
    }

    /// Merges two fibonacci heaps and returns the merged fibonacci heap
    ///
    /// # Arguments:
    /// * `fibonacci_heap_1`: first fibonacci heap
    /// * `fibonacci_heap_2`: second fibonacci heap
    ///
    /// # Panics:
    /// * panics if two fibonacci heaps are not the same kind(ex. one is min heap and the other is max heap)
    ///
    /// # Examples
    /// ```
    /// use rudac::heap::FibonacciHeap;
    ///
    /// let mut fibonacci_heap_1: FibonacciHeap<usize> = FibonacciHeap::init_min();
    /// fibonacci_heap_1.push(0);
    /// fibonacci_heap_1.push(2);
    ///
    /// let mut fibonacci_heap_2: FibonacciHeap<usize> = FibonacciHeap::init_min();
    /// fibonacci_heap_2.push(1);
    /// fibonacci_heap_2.push(3);
    ///
    /// let merged_heap = FibonacciHeap::merge(fibonacci_heap_2, fibonacci_heap_1);
    ///
    /// assert_eq!(
    ///     FibonacciHeap::preorder(&merged_heap),
    ///     String::from("Priority: 0\nTree 1: 3\nTree 2: 2\nTree 3: 1\n")
    /// );
    /// ```
    pub fn merge(
        mut fibonacci_heap_1: FibonacciHeap<T>,
        mut fibonacci_heap_2: FibonacciHeap<T>,
    ) -> FibonacciHeap<T> {
        // if one heap is min and the other is max, panic!. merge is not possible
        if fibonacci_heap_1.is_min() != fibonacci_heap_2.is_min() {
            panic!("Two heaps must be of same type in order for merge to be possible")
        }

        // if either heaps are empty, return the other one as result
        if fibonacci_heap_1.is_empty() {
            return fibonacci_heap_2;
        } else if fibonacci_heap_2.is_empty() {
            return fibonacci_heap_1;
        }

        // concatenate children list of heap1 and heap2
        fibonacci_heap_1
            .children_list
            .append(&mut fibonacci_heap_2.children_list);

        let heap_is_min = fibonacci_heap_1.is_min();

        // update priority node in merged heap
        // if priority node in heap2 has higher priority than priority node in heap1, priority node in heap2 must become the new priority node of merged heap
        if InternalTree::has_higher_priority(
            &fibonacci_heap_2.priority_pointer.as_ref().unwrap(),
            &fibonacci_heap_1.priority_pointer.as_ref().unwrap(),
            heap_is_min,
        ) {
            // swap priority nodes of heap2 and heap1
            let temp = fibonacci_heap_1.priority_pointer.take().unwrap();
            fibonacci_heap_1.priority_pointer = fibonacci_heap_2.priority_pointer.take();
            fibonacci_heap_1.children_list.push_back(temp);
        } else {
            // if priority node of heap2 has lower priority then just add it to children list of heap1
            fibonacci_heap_1
                .children_list
                .push_back(fibonacci_heap_2.priority_pointer.unwrap());
        }

        // calculate size of merged heap
        fibonacci_heap_1.size += fibonacci_heap_2.size;

        // return merged heap
        fibonacci_heap_1
    }

    /// Pops and returns item with highest priority. Returns `None` if heap is empty. After pop, heap will be consolidated
    ///
    /// # Examples
    /// ```
    /// use rudac::heap::FibonacciHeap;
    ///
    /// let mut fibonacci_heap: FibonacciHeap<usize> = FibonacciHeap::init_min();
    /// fibonacci_heap.push(2);
    /// fibonacci_heap.push(3);
    /// fibonacci_heap.push(0);
    /// fibonacci_heap.push(1);
    ///
    /// // before pop
    /// assert_eq!(
    ///     FibonacciHeap::preorder(&fibonacci_heap),
    ///     String::from("Priority: 0\nTree 1: 3\nTree 2: 2\nTree 3: 1\n")    
    /// );
    ///
    /// assert_eq!(fibonacci_heap.pop(), Some(0));
    ///
    /// // heap trees are consolidated
    /// assert_eq!(
    ///     FibonacciHeap::preorder(&fibonacci_heap),
    ///     String::from("Priority: 1\nTree 1: 2 3\n")
    /// );
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        // extract node with highest priority from heap
        let priority_node = self.priority_pointer.take().unwrap();

        // account for deleted node
        self.size -= 1;

        // iterate over children of removed node and add them to children list of heap
        self.children_list.extend(priority_node.children_list);

        // extract payload of priority node
        let payload = priority_node.payload;

        // if there is nodes in heap, consolidate them
        if !self.is_empty() {
            // a temp priority node just for consolidate method to work
            self.priority_pointer = self.children_list.pop_front();

            self.consolidate();
        }

        // return payload with highest priority
        Some(payload)
    }

    // this method consolidate trees in fibonacci heap
    // until each tree in children list of the heap has a unique degree
    // ex after consolidate there can not be two trees with degree 0 like: 0 <-> 1
    //
    // after consolidate total number of trees in heap is gonna be at most log(n)
    fn consolidate(&mut self) {
        // there is nothing to consolidate
        if self.is_empty() {
            return;
        }
        // use a helper vector for consolidating
        // vector keeps track of degree of present trees
        // therefore we can make sure each degree is associated with a unique tree
        // array size will be log(heap size) with base 1.61803
        let array_size = ((self.size as f32).log(1.61803_f32) + 1.0) as usize;

        // helper vector for tracking current degrees present in consolidating process
        let mut a: Vec<Option<InternalTree<T>>> = Vec::with_capacity(array_size);

        // initialize consolidate array
        a.resize_with(array_size, || None);

        // add priority node to children list
        // because we have to iterate over all nodes
        self.children_list
            .push_front(self.priority_pointer.take().unwrap());

        // iterate over children and merge trees with same degrees
        for mut x in self.children_list.drain(..) {
            let mut d = x.degree(); // degree of current internal tree
            while a[d].is_some() {
                // iterate over consolidate array to find the place for x
                let y = a[d].take().unwrap(); // if there exists a tree with degree of x like y
                x = InternalTree::merge(x, y); // merge x and y and store merged tree in x
                d += 1; // degree of x is now d + 1 because it has y as its child
            }
            a[d] = Some(x); // finally when a degree is free(a[d]), means degree of x is unique. store it in consolidate array
        }

        // update priority pointer and children list
        let heap_is_min = self.is_min();

        // after consolidate, "a" has all the nodes in the heap
        // we have to find minimum between these nodes and add rest of them to children list of heap
        // so iterate over consolidate array

        let mut nodes = a.into_iter().filter_map(|x| x);
        let mut priority_pointer = nodes.next().unwrap();

        for mut node in nodes {
            if InternalTree::has_higher_priority(
                &node,
                &priority_pointer,
                heap_is_min,
            ) {
                // current tree in a has higher priority than latest found priority node, swap them
                std::mem::swap(&mut priority_pointer, &mut node);
            }
            self.children_list.push_back(node);
        }

        self.priority_pointer = Some(priority_pointer);
    }

    /// Returns a reference to item with highest priority
    ///
    /// # Examples
    /// ```
    /// use rudac::heap::FibonacciHeap;
    ///
    /// let mut fibonacci_heap: FibonacciHeap<usize> = FibonacciHeap::init_min();
    ///
    /// fibonacci_heap.push(0);
    ///
    /// assert_eq!(fibonacci_heap.peek(), Some(&0));
    /// ```
    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }

        let payload = self.priority_pointer.as_ref().unwrap().peek_payload();
        Some(payload)
    }

    /// Clears the heap and resets internal flags
    ///
    /// # Examples
    /// ```
    /// use rudac::heap::FibonacciHeap;
    ///
    /// let mut fibonacci_heap: FibonacciHeap<usize> = FibonacciHeap::init_min();
    /// fibonacci_heap.push(0);
    ///
    /// fibonacci_heap.clear();
    ///
    /// assert_eq!(fibonacci_heap.size(), 0);
    /// assert_eq!(fibonacci_heap.pop(), None);
    /// ```
    pub fn clear(&mut self) {
        self.children_list.clear();
        self.size = 0;
        self.priority_pointer = None;
    }

    /// Returns true if there are no more items in the heap
    ///
    /// # Examples
    /// ```
    /// use rudac::heap::FibonacciHeap;
    ///
    /// let mut fibonacci_heap: FibonacciHeap<usize> = FibonacciHeap::init_min();
    ///
    /// fibonacci_heap.push(0);
    /// assert_eq!(fibonacci_heap.is_empty(), false);
    ///
    /// fibonacci_heap.pop();
    /// assert_eq!(fibonacci_heap.is_empty(), true);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Returns number of items in heap
    ///
    /// # Examples
    /// ```
    /// use rudac::heap::FibonacciHeap;
    ///
    /// let mut fibonacci_heap: FibonacciHeap<usize> = FibonacciHeap::init_min();
    /// fibonacci_heap.push(0);
    /// fibonacci_heap.push(1);
    ///
    /// assert_eq!(fibonacci_heap.size(), 2);
    /// ```
    pub fn size(&self) -> usize {
        self.size
    }

    /// Returns true if the heap is initialized as a min heap
    ///
    /// # Examples
    /// ```
    /// use rudac::heap::FibonacciHeap;
    ///
    /// let mut fibonacci_heap: FibonacciHeap<usize> = FibonacciHeap::init_min();
    ///
    /// assert_eq!(fibonacci_heap.is_min(), true);
    /// ```
    pub fn is_min(&self) -> bool {
        self.min
    }

    /// Returns true if the heap is initialized as a max heap
    ///
    /// # Examples
    /// ```
    /// use rudac::heap::FibonacciHeap;
    ///
    /// let mut fibonacci_heap: FibonacciHeap<usize> = FibonacciHeap::init_max();
    ///
    /// assert_eq!(fibonacci_heap.is_max(), true);
    /// ```
    pub fn is_max(&self) -> bool {
        !self.is_min()
    }
}

impl<T> FibonacciHeap<T>
where
    T: std::cmp::Ord + std::fmt::Display,
{
    /// Returns the preorder representation of the heap. it has the form of:</br>
    /// Priority: *preorder representation of tree containing priority value*\n
    /// Tree i: *preorder representation of the internal tree of rank i*\n
    ///
    /// # Arguments:
    /// * `fibonacci_heap`: reference to a fibonacci heap
    ///
    /// # Examples
    /// ```
    /// use rudac::heap::FibonacciHeap;
    ///
    /// let mut fibonacci_heap: FibonacciHeap<usize> = FibonacciHeap::init_min();
    /// for i in 0..14 {
    ///     fibonacci_heap.push(i)
    /// }
    ///
    /// fibonacci_heap.pop();
    ///
    /// assert_eq!(
    ///     FibonacciHeap::preorder(&fibonacci_heap),
    ///     String::from("Priority: 1 2 3 4 5 6 7 8\nTree 1: 13\nTree 2: 9 10 11 12\n")
    /// );
    /// ```
    pub fn preorder(fibonacci_heap: &FibonacciHeap<T>) -> String {
        let mut node_list = String::from("");

        if !fibonacci_heap.priority_pointer.is_none() {
            node_list.push_str(
                format!(
                    "Priority: {}\n",
                    InternalTree::preorder(&fibonacci_heap.priority_pointer.as_ref().unwrap())
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
        let fh: FibonacciHeap<usize> = FibonacciHeap::init_min();

        assert!(fh.priority_pointer.is_none());
        assert_eq!(fh.size, 0);
        assert_eq!(fh.children_list.len(), 0);
    }

    #[test]
    fn heap_fibonacci_push_1() {
        let mut fh: FibonacciHeap<usize> = FibonacciHeap::init_min();

        fh.push(0);
        fh.push(1);
        fh.push(3);

        assert_eq!(fh.children_list.len(), 2);
        assert_eq!(
            *fh.priority_pointer
                .as_ref()
                .unwrap()
                .peek_payload(),
            0
        );

        assert_eq!(
            FibonacciHeap::preorder(&fh),
            String::from("Priority: 0\nTree 1: 1\nTree 2: 3\n")
        )
    }

    #[test]
    fn heap_fibonacci_push_2() {
        let mut fh: FibonacciHeap<usize> = FibonacciHeap::init_min();

        fh.push(3);
        fh.push(1);
        fh.push(0);

        assert_eq!(fh.children_list.len(), 2);
        assert_eq!(
            *fh.priority_pointer
                .as_ref()
                .unwrap()
                .peek_payload(),
            0
        );

        assert_eq!(
            FibonacciHeap::preorder(&fh),
            String::from("Priority: 0\nTree 1: 3\nTree 2: 1\n")
        )
    }

    #[test]
    fn heap_fibonacci_merge_1() {
        let mut fh1: FibonacciHeap<usize> = FibonacciHeap::init_min();
        fh1.push(0);

        let mut fh2: FibonacciHeap<usize> = FibonacciHeap::init_min();
        fh2.push(1);

        let merged_heap = FibonacciHeap::merge(fh1, fh2);

        assert_eq!(merged_heap.size, 2);
        assert_eq!(
            *merged_heap
                .priority_pointer
                .as_ref()
                .unwrap()
                .peek_payload(),
            0
        );
        assert_eq!(
            FibonacciHeap::preorder(&merged_heap),
            String::from("Priority: 0\nTree 1: 1\n")
        );
    }

    #[test]
    fn heap_fibonacci_merge_2() {
        let mut fh1: FibonacciHeap<usize> = FibonacciHeap::init_min();
        fh1.push(0);
        fh1.push(2);

        let mut fh2: FibonacciHeap<usize> = FibonacciHeap::init_min();
        fh2.push(1);
        fh2.push(3);

        let merged_heap = FibonacciHeap::merge(fh2, fh1);

        assert_eq!(merged_heap.size, 4);
        assert_eq!(
            *merged_heap
                .priority_pointer
                .as_ref()
                .unwrap()
                .peek_payload(),
            0
        );
        assert_eq!(
            FibonacciHeap::preorder(&merged_heap),
            String::from("Priority: 0\nTree 1: 3\nTree 2: 2\nTree 3: 1\n")
        );
    }

    #[test]
    fn heap_fibonacci_merge_3() {
        let mut fh1: FibonacciHeap<usize> = FibonacciHeap::init_min();
        fh1.push(0);
        fh1.push(2);

        let fh2: FibonacciHeap<usize> = FibonacciHeap::init_min();

        let merged_heap = FibonacciHeap::merge(fh2, fh1);

        assert_eq!(merged_heap.size, 2);
        assert_eq!(
            *merged_heap
                .priority_pointer
                .as_ref()
                .unwrap()
                .peek_payload(),
            0
        );
        assert_eq!(
            FibonacciHeap::preorder(&merged_heap),
            String::from("Priority: 0\nTree 1: 2\n")
        );
    }

    #[test]
    #[should_panic(expected = "Two heaps must be of same type in order for merge to be possible")]
    fn heap_fibonacci_panic_merge() {
        let fh1: FibonacciHeap<usize> = FibonacciHeap::init_min();

        let fh2: FibonacciHeap<usize> = FibonacciHeap::init_max();

        FibonacciHeap::merge(fh2, fh1);
    }

    #[test]
    fn heap_fibonacci_merge_after_consolidate() {
        let mut fh1: FibonacciHeap<usize> = FibonacciHeap::init_min();
        for i in 0..14 {
            fh1.push(i)
        }

        fh1.consolidate();

        let mut fh2: FibonacciHeap<usize> = FibonacciHeap::init_min();
        for i in 14..20 {
            fh2.push(i)
        }

        fh2.consolidate();

        let merged_heap = FibonacciHeap::merge(fh1, fh2);
        assert_eq!(merged_heap.size, 20);
        assert_eq!(FibonacciHeap::preorder(&merged_heap),String::from("Priority: 0 1 2 3 4 5 6 7\nTree 1: 12 13\nTree 2: 8 9 10 11\nTree 3: 18 19\nTree 4: 14 15 16 17\n"));
    }

    #[test]
    fn heap_fibonacci_consolidate_1() {
        let mut fh: FibonacciHeap<usize> = FibonacciHeap::init_min();
        fh.push(0);

        fh.consolidate();

        assert_eq!(FibonacciHeap::preorder(&fh), String::from("Priority: 0\n"));
    }

    #[test]
    fn heap_fibonacci_consolidate_2() {
        let mut fh: FibonacciHeap<usize> = FibonacciHeap::init_min();
        fh.push(1);
        fh.push(0);

        fh.consolidate();

        assert_eq!(
            FibonacciHeap::preorder(&fh),
            String::from("Priority: 0 1\n")
        );
    }

    #[test]
    fn heap_fibonacci_consolidate_3() {
        let mut fh: FibonacciHeap<usize> = FibonacciHeap::init_min();
        fh.push(1);
        fh.push(0);
        fh.push(2);

        fh.consolidate();

        assert_eq!(
            FibonacciHeap::preorder(&fh),
            String::from("Priority: 0 1\nTree 1: 2\n")
        );
    }

    #[test]
    fn heap_fibonacci_consolidate_4() {
        let mut fh: FibonacciHeap<usize> = FibonacciHeap::init_min();
        fh.push(1);
        fh.push(0);
        fh.push(2);
        fh.push(3);

        fh.consolidate();

        assert_eq!(
            FibonacciHeap::preorder(&fh),
            String::from("Priority: 0 1 2 3\n")
        );
    }

    #[test]
    fn heap_fibonacci_consolidate_5() {
        let mut fh: FibonacciHeap<usize> = FibonacciHeap::init_min();
        fh.push(1);
        fh.push(0);
        fh.push(3);
        fh.push(2);
        fh.push(4);

        fh.consolidate();

        assert_eq!(
            FibonacciHeap::preorder(&fh),
            String::from("Priority: 0 1 2 3\nTree 1: 4\n")
        );
    }

    #[test]
    fn heap_fibonacci_consolidate_6() {
        let mut fh: FibonacciHeap<usize> = FibonacciHeap::init_min();
        for i in 0..14 {
            fh.push(i)
        }

        fh.consolidate();

        assert_eq!(
            FibonacciHeap::preorder(&fh),
            String::from("Priority: 0 1 2 3 4 5 6 7\nTree 1: 12 13\nTree 2: 8 9 10 11\n")
        );
    }

    #[test]
    fn heap_fibonacci_pop_1() {
        let mut fh: FibonacciHeap<usize> = FibonacciHeap::init_min();
        fh.push(0);

        assert_eq!(fh.pop(), Some(0));

        assert_eq!(FibonacciHeap::preorder(&fh), String::from(""));
    }

    #[test]
    fn heap_fibonacci_pop_2() {
        let mut fh: FibonacciHeap<usize> = FibonacciHeap::init_min();
        fh.push(0);
        fh.push(1);

        assert_eq!(fh.pop(), Some(0));
        assert_eq!(fh.size(), 1);
        assert_eq!(FibonacciHeap::preorder(&fh), String::from("Priority: 1\n"));
    }

    #[test]
    fn heap_fibonacci_pop_3() {
        let mut fh: FibonacciHeap<usize> = FibonacciHeap::init_min();
        fh.push(2);
        fh.push(0);
        fh.push(1);

        assert_eq!(fh.pop(), Some(0));
        assert_eq!(fh.size(), 2);
        assert_eq!(
            FibonacciHeap::preorder(&fh),
            String::from("Priority: 1 2\n")
        );
    }

    #[test]
    fn heap_fibonacci_pop_4() {
        let mut fh: FibonacciHeap<usize> = FibonacciHeap::init_min();
        fh.push(2);
        fh.push(3);
        fh.push(0);
        fh.push(1);

        assert_eq!(fh.pop(), Some(0));
        assert_eq!(fh.size(), 3);
        assert_eq!(
            FibonacciHeap::preorder(&fh),
            String::from("Priority: 1\nTree 1: 2 3\n")
        );
    }

    #[test]
    fn heap_fibonacci_pop_multi_1() {
        let mut fh: FibonacciHeap<usize> = FibonacciHeap::init_min();
        fh.push(2);
        fh.push(3);
        fh.push(0);
        fh.push(1);

        assert_eq!(fh.pop(), Some(0));
        assert_eq!(fh.size(), 3);

        assert_eq!(fh.pop(), Some(1));
        assert_eq!(fh.size(), 2);
        assert_eq!(
            FibonacciHeap::preorder(&fh),
            String::from("Priority: 2 3\n")
        );

        assert_eq!(fh.pop(), Some(2));
        assert_eq!(fh.size(), 1);
        assert_eq!(FibonacciHeap::preorder(&fh), String::from("Priority: 3\n"));

        assert_eq!(fh.pop(), Some(3));
        assert_eq!(fh.size(), 0);
        assert_eq!(FibonacciHeap::preorder(&fh), String::from(""));

        assert_eq!(fh.pop(), None);
        assert_eq!(fh.size(), 0);
        assert_eq!(FibonacciHeap::preorder(&fh), String::from(""));
    }

    #[test]
    fn heap_fibonacci_pop_multi_2() {
        let mut fh: FibonacciHeap<usize> = FibonacciHeap::init_min();
        for i in 0..5 {
            fh.push(i)
        }
        assert_eq!(fh.pop(), Some(0));
        assert_eq!(fh.size(), 4);
        assert_eq!(
            FibonacciHeap::preorder(&fh),
            String::from("Priority: 1 2 3 4\n")
        );

        assert_eq!(fh.pop(), Some(1));
        assert_eq!(fh.size(), 3);
        assert_eq!(
            FibonacciHeap::preorder(&fh),
            String::from("Priority: 2\nTree 1: 3 4\n")
        );

        assert_eq!(fh.pop(), Some(2));
        assert_eq!(fh.size(), 2);
        assert_eq!(
            FibonacciHeap::preorder(&fh),
            String::from("Priority: 3 4\n")
        );

        assert_eq!(fh.pop(), Some(3));
        assert_eq!(fh.size(), 1);
        assert_eq!(FibonacciHeap::preorder(&fh), String::from("Priority: 4\n"));

        assert_eq!(fh.pop(), Some(4));
        assert_eq!(fh.size(), 0);
        assert_eq!(FibonacciHeap::preorder(&fh), String::from(""));
    }
}
