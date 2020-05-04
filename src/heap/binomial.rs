use crate::tree::BinomialTree;

/// A binomial heap is a data structure that acts as a priority queue but also allows pairs of heaps to be merged together
///
/// # Examples
/// ```
/// use rudac::heap::BinomialHeap;
///
/// // create a min heap
/// let mut binomial_heap = BinomialHeap::init_min(0);
///
/// // push items
/// binomial_heap.push(1);
/// binomial_heap.push(2);
/// binomial_heap.push(3);
///
/// // there are no binomial tree with rank 0
/// // there are no binomial tree with rank 1
/// // tree with rank 2 contains: 0
/// //                            | \
/// //                            1  2
/// //                               |
/// //                               3
/// assert_eq!(
///     BinomialHeap::preorder(&binomial_heap),
///     format!("Rank 0: \nRank 1: \nRank 2: 0 1 2 3\n")
/// );
/// ```
#[derive(Debug)]
pub struct BinomialHeap<T: std::cmp::Ord> {
    // stores binomial trees of different ranks
    // index of the vector represents the rank of the tree
    // ex. tree at index=2 has rank=2 thus has 4 nodes in it
    roots: Vec<Option<BinomialTree<T>>>,
    // index of the root with highest priority(candidate to be popped)
    candidate_root_index: usize,

    // number of items in the heap
    size: usize,

    // indicates wether current heap is a min heap or not
    min: bool,
}

impl<T: std::cmp::Ord> BinomialHeap<T> {
    // initializes binomial heap based on the type specified by `min` argument
    fn init(payload: T, min: bool) -> BinomialHeap<T> {
        // create a binomial tree with rank 0
        let root = Some(BinomialTree::init(payload, min));

        let mut roots = Vec::new();

        // push the binomial tree into heap
        roots.push(root);

        BinomialHeap {
            roots: roots,
            size: 1,
            candidate_root_index: 0,
            min,
        }
    }

    /// Initializes a min heap with the specified `payload`
    ///
    /// # Arguments:
    /// * `payload`: data to be pushed in the heap
    ///
    /// # Examples
    /// ```
    /// use rudac::heap::BinomialHeap;
    ///
    /// let binomial_heap = BinomialHeap::init_min("rudac is awesome");
    /// ```
    pub fn init_min(payload: T) -> BinomialHeap<T> {
        BinomialHeap::init(payload, true)
    }

    /// Initializes a max heap with the specified `payload`
    ///
    /// # Arguments:
    /// * `payload`: data to be pushed in the heap
    ///
    /// # Examples
    /// ```
    /// use rudac::heap::BinomialHeap;
    ///
    /// let binomial_heap = BinomialHeap::init_max("rudac is awesome");
    /// ```
    pub fn init_max(payload: T) -> BinomialHeap<T> {
        BinomialHeap::init(payload, false)
    }

    /// Merges two binomial heaps and returns the merged binomial heap
    ///
    /// # Arguments:
    /// * `binomial_heap_1`: first binomial heap
    /// * `binomial_heap_2`: second binomial heap
    ///
    /// # Panics:
    /// * panics if two binomial heaps are not the same kind(ex. one is min heap and the other is max heap)
    ///
    /// # Examples
    /// ```
    /// use rudac::heap::BinomialHeap;
    ///
    /// let binomial_heap_1 = BinomialHeap::init_min(0);
    /// let binomial_heap_2 = BinomialHeap::init_min(1);
    ///
    /// let merged_heap = BinomialHeap::merge(binomial_heap_1, binomial_heap_2);
    ///
    /// assert_eq!(BinomialHeap::preorder(&merged_heap), String::from("Rank 0: \nRank 1: 0 1\n"))
    /// ```
    pub fn merge(
        binomial_heap_1: BinomialHeap<T>,
        binomial_heap_2: BinomialHeap<T>,
    ) -> BinomialHeap<T> {
        // two binomial heaps must be of same kind in order for merge to be possible
        if binomial_heap_1.is_min() != binomial_heap_2.is_min() {
            panic!("Both binomial heaps must be of the same type(both min or both max)");
        }

        // for less trouble iterate over the smaller heap and insert its binomial trees into the second(larger) heap
        // this helps cause: there will be no index out of bound in the larger(when calling _push).
        // because every rank present in the smaller heap is also present in the larger heap.
        // thats why we pass the smaller heap as the first argument and the other one(larger one) as the second argument
        if binomial_heap_1.max_tree_rank() <= binomial_heap_2.max_tree_rank() {
            BinomialHeap::_merge(binomial_heap_1, binomial_heap_2)
        } else {
            BinomialHeap::_merge(binomial_heap_2, binomial_heap_1)
        }
    }

    // merges smaller heap(binomial_heap_1) with a larger heap(binomial_heap_2)
    fn _merge(
        mut binomial_heap_1: BinomialHeap<T>,
        mut binomial_heap_2: BinomialHeap<T>,
    ) -> BinomialHeap<T> {
        // size of heap2 will be heap2.size + heap1.size
        binomial_heap_2.set_size(binomial_heap_2.size() + binomial_heap_1.size());

        // iterate over binomial trees in heap1 and push them into heap 2
        for i in 0..binomial_heap_1.max_tree_rank() {
            match binomial_heap_1.roots[i].take() {
                Some(binomial_tree) => {
                    binomial_heap_2._push(binomial_tree);
                }
                None => (),
            }
        }

        binomial_heap_2
    }

    /// pushes specified `payload` into heap
    ///
    /// # Arguments:
    /// * `payload`: data to be pushed into heap
    ///
    /// # Examples
    /// ```
    /// use rudac::heap::BinomialHeap;
    ///
    /// let mut binomial_heap = BinomialHeap::init_min(0);
    /// binomial_heap.push(1);
    ///
    /// assert_eq!(BinomialHeap::preorder(&binomial_heap), String::from("Rank 0: \nRank 1: 0 1\n"))
    /// ```
    pub fn push(&mut self, payload: T) {
        // create a compatible binomial tree with rank 0 that is compatible with the current heap(hence the passing of self.is_min())
        let new_node = BinomialTree::init(payload, self.is_min());

        self._push(new_node);

        self.size += 1;
    }

    // pushes a binomial tree into heap
    fn _push(&mut self, mut new_node: BinomialTree<T>) {
        // maximum rank in the heap(end of roots vector)
        let max_rank = self.roots.len();

        // iteration for pushing must start at the rank of the new node
        let start_rank = new_node.rank();

        // it will iterate till it reaches maximum rank
        // if it passes the maximum rank(merging trees continue that far), a larger rank will be allocated
        for i in start_rank..max_rank {
            match self.roots[i].take() {
                Some(node) => {
                    // if there is already a binomial tree with rank i, merge the new node and the alredy existing binomial tree.
                    // it will create a binomial tree with a new_rank = old_rank + 1
                    // so continue the iteration
                    new_node = BinomialTree::merge(node, new_node);

                    // if iteration reaches the highest rank and there is still no place empty for the new node to be inserted,
                    // then allocate a new rank(max_rank + 1) and insert the new node there
                    if i == max_rank - 1 {
                        self.roots.push(Some(new_node));
                        break;
                    }
                }
                None => {
                    // when we are here, we found an empty place to insert our new node
                    self.roots[i] = Some(new_node);
                    break;
                }
            }
        }

        // update candidate index
        self.candidate_root_index = self.find_candidate_root_index();
    }

    /// Pops and returns item with highest priority. Returns `None` if heap is empty
    ///
    /// # Examples
    /// ```
    /// use rudac::heap::BinomialHeap;
    ///
    /// let mut binomial_heap = BinomialHeap::init_min(0);
    ///
    /// assert_eq!(binomial_heap.pop(), Some(0));
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        // find index of root with highest priority
        let candidate_index = self.find_candidate_root_index();

        // extract the node from heap
        let mut popped_node = self.roots[candidate_index].take().unwrap();

        // push children of the popped node into heap
        for i in 0..popped_node.children().len() {
            let child = popped_node.children_mut()[i].take().unwrap();

            self._push(child);
        }

        self.size -= 1;

        // return payload the popped node
        Some(popped_node.get_payload())
    }

    /// Returns a reference to item with highest priority
    /// 
    /// # Examples
    /// ```
    /// use rudac::heap::BinomialHeap;
    /// 
    /// let bh1 = BinomialHeap::init_min(0);
    /// let bh2 = BinomialHeap::init_min(1);
    /// let mut merged_heap = BinomialHeap::merge(bh1, bh2);
    /// 
    /// assert_eq!(*merged_heap.peek(), Some(0));
    /// merged_heap.pop();
    /// 
    /// assert_eq!(*merged_heap.peek(), Some(1));
    /// merged_heap.pop();
    /// 
    /// merged_heap.pop();
    /// assert_eq!(*merged_heap.peek(), None);
    /// ```
    pub fn peek(&self) -> &Option<T> {
        if self.is_empty() {
            return &None;
        }

        self.roots[self.candidate_root_index]
            .as_ref()
            .unwrap()
            .peek_payload()
    }

    /// Clears the heap and resets internal flags
    ///
    /// # Examples
    /// ```
    /// use rudac::heap::BinomialHeap;
    ///
    /// let mut binomial_heap = BinomialHeap::init_min(0);
    ///
    /// binomial_heap.clear();
    ///
    /// assert_eq!(binomial_heap.size(), 0);
    /// assert_eq!(binomial_heap.pop(), None);
    /// ```
    pub fn clear(&mut self) {
        self.roots.clear();
        self.size = 0;
    }

    /// Returns number of items in heap
    ///
    /// # Examples
    /// ```
    /// use rudac::heap::BinomialHeap;
    ///
    /// let mut binomial_heap = BinomialHeap::init_min(0);
    /// binomial_heap.push(1);
    ///
    /// assert_eq!(binomial_heap.size(), 2);
    /// ```
    pub fn size(&self) -> usize {
        self.size
    }

    // updates size of the heap
    fn set_size(&mut self, size: usize) {
        self.size = size;
    }

    /// Returns true if the current heap is a min heap
    ///
    /// # Examples
    /// ```
    /// use rudac::heap::BinomialHeap;
    ///
    /// let binomial_heap = BinomialHeap::init_min(0);
    ///
    /// assert_eq!(binomial_heap.is_min(), true);
    /// ```
    pub fn is_min(&self) -> bool {
        self.min
    }

    /// Returns true if the current heap is a max heap
    ///
    /// # Examples
    /// ```
    /// use rudac::heap::BinomialHeap;
    ///
    /// let binomial_heap = BinomialHeap::init_max(0);
    ///
    /// assert_eq!(binomial_heap.is_max(), true);
    /// ```
    pub fn is_max(&self) -> bool {
        !self.is_min()
    }

    /// Returns true if there are no more items in the heap
    ///
    /// # Examples
    /// ```
    /// use rudac::heap::BinomialHeap;
    ///
    /// let mut binomial_heap = BinomialHeap::init_min(0);
    /// assert_eq!(binomial_heap.is_empty(), false);
    ///
    /// binomial_heap.pop();
    /// assert_eq!(binomial_heap.is_empty(), true);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }

    fn max_tree_rank(&self) -> usize {
        self.roots.len()
    }

    // find index of root with highest priority(minimum root in min heap and maximum root in max heap)
    fn find_candidate_root_index(&self) -> usize {
        // candidate index to pop the item with largest priority
        let mut candidate_index = 0;

        // candidate node to be popped
        let mut candidate_node_option: &Option<BinomialTree<T>>;

        // find first rank which has an existing binomial tree in it.
        // root of the found binomial tree will become the candidate.
        // unless in the next iteration we find a root with larger priority, this root will be popped
        for i in 0..self.roots.len() {
            match &self.roots[i] {
                Some(_) => {
                    candidate_index = i; // found candidate index
                    break;
                }
                None => (),
            }
        }

        // initialize the candidate node to be popped
        candidate_node_option = &self.roots[candidate_index];

        // find index of the item with largest priority
        // iteration will start at the next index of the candidate index
        for i in candidate_index + 1..self.roots.len() {
            match (&self.roots[i], candidate_node_option) {
                (Some(node), Some(largest_priority_node)) => {
                    // in two cases candidate node will be replaced with the current node in the iteration
                    // 1- heap is a min heap and current node has a smaller root than the candidate
                    // 2- heap is a max heap and current node has a larger root than the candidate
                    if (self.is_min()
                        && BinomialTree::is_smaller_or_equall(&node, largest_priority_node))
                        || (self.is_max()
                            && BinomialTree::is_greater_or_equall(&node, largest_priority_node))
                    {
                        candidate_index = i; // update candidate index
                    }
                }
                _ => (),
            }
            candidate_node_option = &self.roots[candidate_index]; // update candidate node
        }

        candidate_index
    }
}

impl<T: std::cmp::Ord + std::fmt::Display> BinomialHeap<T> {
    /// Returns the preorder representation of the heap. it has the form of:</br>
    /// Rank i: *preorder representation of the binomial tree of rank i*\n
    ///
    /// # Arguments:
    /// * `binomial_heap`: reference to a binomial heap
    ///
    /// # Examples
    /// ```
    /// use rudac::heap::BinomialHeap;
    ///
    /// let mut binomial_heap = BinomialHeap::init_min(0);
    /// binomial_heap.push(1);
    /// binomial_heap.push(2);
    /// binomial_heap.push(3);
    /// binomial_heap.push(4);
    /// binomial_heap.push(5);
    /// binomial_heap.push(6);
    ///
    /// assert_eq!(BinomialHeap::preorder(&binomial_heap), "Rank 0: 6\nRank 1: 4 5\nRank 2: 0 1 2 3\n");
    /// ```
    pub fn preorder(binomial_heap: &BinomialHeap<T>) -> String {
        // output string that contains a list of nodes in a preorder fashion
        let mut node_list = String::from("");

        // iterate over all binomial trees in the heap
        for i in 0..binomial_heap.roots.len() {
            // add the payload of the current node
            node_list.push_str(format!("Rank {}: ", i).as_str());

            // visit all children of the current node from left to right
            match &binomial_heap.roots[i] {
                Some(binomial_tree) => {
                    node_list.push_str(BinomialTree::preorder(&binomial_tree).as_str())
                }
                None => (),
            }

            node_list.push_str("\n");
        }

        node_list
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heap_binomial_init_min() {
        let bh = BinomialHeap::init_min(0);

        assert_eq!(BinomialHeap::preorder(&bh), format!("Rank 0: 0\n"));
        assert_eq!(bh.is_min(), true);
        assert_eq!(bh.is_max(), false);
    }

    #[test]
    fn heap_binomial_init_max() {
        let bh = BinomialHeap::init_max(0);

        assert_eq!(BinomialHeap::preorder(&bh), format!("Rank 0: 0\n"));
        assert_eq!(bh.is_min(), false);
        assert_eq!(bh.is_max(), true);
    }

    #[test]
    fn heap_binomial_push_min_1() {
        let mut bh = BinomialHeap::init_min(0);

        bh.push(1);

        assert_eq!(
            BinomialHeap::preorder(&bh),
            format!("Rank 0: \nRank 1: 0 1\n")
        );
    }

    #[test]
    fn heap_binomial_push_max_1() {
        let mut bh = BinomialHeap::init_max(0);

        bh.push(1);

        assert_eq!(
            BinomialHeap::preorder(&bh),
            format!("Rank 0: \nRank 1: 1 0\n")
        );
    }

    #[test]
    fn heap_binomial_push_min_2() {
        let mut bh = BinomialHeap::init_min(0);

        bh.push(1);
        bh.push(2);

        assert_eq!(
            BinomialHeap::preorder(&bh),
            format!("Rank 0: 2\nRank 1: 0 1\n")
        );
    }

    #[test]
    fn heap_binomial_push_max_2() {
        let mut bh = BinomialHeap::init_max(0);

        bh.push(1);
        bh.push(2);

        assert_eq!(
            BinomialHeap::preorder(&bh),
            format!("Rank 0: 2\nRank 1: 1 0\n")
        );
    }

    #[test]
    fn heap_binomial_push_min_3() {
        let mut bh = BinomialHeap::init_min(0);

        bh.push(1);
        bh.push(2);
        bh.push(3);

        assert_eq!(
            BinomialHeap::preorder(&bh),
            format!("Rank 0: \nRank 1: \nRank 2: 0 1 2 3\n")
        );
    }

    #[test]
    fn heap_binomial_push_max_3() {
        let mut bh = BinomialHeap::init_max(0);

        bh.push(1);
        bh.push(2);
        bh.push(3);

        assert_eq!(
            BinomialHeap::preorder(&bh),
            format!("Rank 0: \nRank 1: \nRank 2: 3 2 1 0\n")
        );
    }

    #[test]
    fn heap_binomial_pop_min_1() {
        let mut bh = BinomialHeap::init_min(0);

        bh.push(1);
        bh.push(2);
        bh.push(3);

        let value = bh.pop();

        assert_eq!(
            BinomialHeap::preorder(&bh),
            format!("Rank 0: 1\nRank 1: 2 3\nRank 2: \n")
        );
        assert_eq!(value, Some(0))
    }

    #[test]
    fn heap_binomial_pop_max_1() {
        let mut bh = BinomialHeap::init_max(0);

        bh.push(1);
        bh.push(2);
        bh.push(3);

        let value = bh.pop();

        assert_eq!(
            BinomialHeap::preorder(&bh),
            format!("Rank 0: 2\nRank 1: 1 0\nRank 2: \n")
        );
        assert_eq!(value, Some(3))
    }

    #[test]
    fn heap_binomial_pop_min_2() {
        let mut bh = BinomialHeap::init_min(0);

        bh.push(1);
        bh.push(2);
        bh.push(3);

        bh.push(8);
        bh.push(9);

        bh.push(7);

        let mut value = bh.pop();

        assert_eq!(
            BinomialHeap::preorder(&bh),
            format!("Rank 0: \nRank 1: 2 3\nRank 2: 1 7 8 9\n")
        );
        assert_eq!(value, Some(0));

        value = bh.pop();
        assert_eq!(
            BinomialHeap::preorder(&bh),
            format!("Rank 0: 7\nRank 1: \nRank 2: 2 3 8 9\n")
        );
        assert_eq!(value, Some(1));

        value = bh.pop();
        assert_eq!(
            BinomialHeap::preorder(&bh),
            format!("Rank 0: \nRank 1: \nRank 2: 3 7 8 9\n")
        );
        assert_eq!(value, Some(2));

        value = bh.pop();
        assert_eq!(
            BinomialHeap::preorder(&bh),
            format!("Rank 0: 7\nRank 1: 8 9\nRank 2: \n")
        );
        assert_eq!(value, Some(3));

        value = bh.pop();
        assert_eq!(
            BinomialHeap::preorder(&bh),
            format!("Rank 0: \nRank 1: 8 9\nRank 2: \n")
        );
        assert_eq!(value, Some(7));

        value = bh.pop();
        assert_eq!(
            BinomialHeap::preorder(&bh),
            format!("Rank 0: 9\nRank 1: \nRank 2: \n")
        );
        assert_eq!(value, Some(8));

        value = bh.pop();
        assert_eq!(
            BinomialHeap::preorder(&bh),
            format!("Rank 0: \nRank 1: \nRank 2: \n")
        );
        assert_eq!(value, Some(9));

        value = bh.pop();
        assert_eq!(
            BinomialHeap::preorder(&bh),
            format!("Rank 0: \nRank 1: \nRank 2: \n")
        );
        assert_eq!(value, None);
    }

    #[test]
    fn heap_binomial_pop_max_2() {
        let mut bh = BinomialHeap::init_max(0);

        bh.push(1);
        bh.push(2);
        bh.push(3);
        bh.push(8);
        bh.push(9);
        bh.push(7);

        let mut value = bh.pop();

        assert_eq!(
            BinomialHeap::preorder(&bh),
            format!("Rank 0: \nRank 1: 8 7\nRank 2: 3 2 1 0\n")
        );
        assert_eq!(value, Some(9));

        value = bh.pop();
        assert_eq!(
            BinomialHeap::preorder(&bh),
            format!("Rank 0: 7\nRank 1: \nRank 2: 3 2 1 0\n")
        );
        assert_eq!(value, Some(8));

        value = bh.pop();
        assert_eq!(
            BinomialHeap::preorder(&bh),
            format!("Rank 0: \nRank 1: \nRank 2: 3 2 1 0\n")
        );
        assert_eq!(value, Some(7));

        value = bh.pop();
        assert_eq!(
            BinomialHeap::preorder(&bh),
            format!("Rank 0: 2\nRank 1: 1 0\nRank 2: \n")
        );
        assert_eq!(value, Some(3));

        value = bh.pop();
        assert_eq!(
            BinomialHeap::preorder(&bh),
            format!("Rank 0: \nRank 1: 1 0\nRank 2: \n")
        );
        assert_eq!(value, Some(2));

        value = bh.pop();
        assert_eq!(
            BinomialHeap::preorder(&bh),
            format!("Rank 0: 0\nRank 1: \nRank 2: \n")
        );
        assert_eq!(value, Some(1));

        value = bh.pop();
        assert_eq!(
            BinomialHeap::preorder(&bh),
            format!("Rank 0: \nRank 1: \nRank 2: \n")
        );
        assert_eq!(value, Some(0));

        value = bh.pop();
        assert_eq!(
            BinomialHeap::preorder(&bh),
            format!("Rank 0: \nRank 1: \nRank 2: \n")
        );
        assert_eq!(value, None);
    }

    #[test]
    fn heap_binomial_merge_min_1() {
        let bh1 = BinomialHeap::init_min(0);
        let bh2 = BinomialHeap::init_min(1);

        let merged_heap = BinomialHeap::merge(bh1, bh2);

        assert_eq!(
            BinomialHeap::preorder(&merged_heap),
            format!("Rank 0: \nRank 1: 0 1\n")
        );
    }

    #[test]
    fn heap_binomial_merge_max_1() {
        let bh1 = BinomialHeap::init_max(0);
        let bh2 = BinomialHeap::init_max(1);

        let merged_heap = BinomialHeap::merge(bh1, bh2);

        assert_eq!(
            BinomialHeap::preorder(&merged_heap),
            format!("Rank 0: \nRank 1: 1 0\n")
        );
    }

    #[test]
    fn heap_binomial_merge_min_2() {
        let bh1 = BinomialHeap::init_min(0);
        let bh2 = BinomialHeap::init_min(1);
        let merged_heap_1 = BinomialHeap::merge(bh1, bh2);

        let bh3 = BinomialHeap::init_min(2);
        let bh4 = BinomialHeap::init_min(3);
        let merged_heap_2 = BinomialHeap::merge(bh3, bh4);

        let merged_heap = BinomialHeap::merge(merged_heap_1, merged_heap_2);

        assert_eq!(
            BinomialHeap::preorder(&merged_heap),
            format!("Rank 0: \nRank 1: \nRank 2: 0 1 2 3\n")
        );
    }

    #[test]
    fn heap_binomial_merge_max_2() {
        let bh1 = BinomialHeap::init_max(0);
        let bh2 = BinomialHeap::init_max(1);
        let merged_heap_1 = BinomialHeap::merge(bh1, bh2);

        let bh3 = BinomialHeap::init_max(2);
        let bh4 = BinomialHeap::init_max(3);
        let merged_heap_2 = BinomialHeap::merge(bh3, bh4);

        let merged_heap = BinomialHeap::merge(merged_heap_1, merged_heap_2);

        assert_eq!(
            BinomialHeap::preorder(&merged_heap),
            format!("Rank 0: \nRank 1: \nRank 2: 3 2 1 0\n")
        );
    }

    #[test]
    fn heap_binomial_peek_min_1() {
        let bh1 = BinomialHeap::init_min(0);

        assert_eq!(*bh1.peek(), Some(0));
        assert_eq!(BinomialHeap::preorder(&bh1), format!("Rank 0: 0\n"));
    }

    #[test]
    fn heap_binomial_peek_min_2() {
        let bh1 = BinomialHeap::init_min(0);
        let bh2 = BinomialHeap::init_min(1);

        let merged_heap = BinomialHeap::merge(bh1, bh2);

        assert_eq!(*merged_heap.peek(), Some(0));
        assert_eq!(
            BinomialHeap::preorder(&merged_heap),
            format!("Rank 0: \nRank 1: 0 1\n")
        );
    }

    #[test]
    fn heap_binomial_peek_min_empty_heap() {
        let bh1 = BinomialHeap::init_min(0);
        let bh2 = BinomialHeap::init_min(1);

        let mut merged_heap = BinomialHeap::merge(bh1, bh2);

        merged_heap.pop();
        merged_heap.pop();
        assert_eq!(*merged_heap.peek(), None);
        assert_eq!(
            BinomialHeap::preorder(&merged_heap),
            format!("Rank 0: \nRank 1: \n")
        );
    }

    #[test]
    fn heap_binomial_peek_max_1() {
        let bh1 = BinomialHeap::init_max(0);

        assert_eq!(*bh1.peek(), Some(0));
        assert_eq!(BinomialHeap::preorder(&bh1), format!("Rank 0: 0\n"));
    }

    #[test]
    fn heap_binomial_peek_max_2() {
        let bh1 = BinomialHeap::init_max(0);
        let bh2 = BinomialHeap::init_max(1);

        let merged_heap = BinomialHeap::merge(bh1, bh2);

        assert_eq!(*merged_heap.peek(), Some(1));
        assert_eq!(
            BinomialHeap::preorder(&merged_heap),
            format!("Rank 0: \nRank 1: 1 0\n")
        );
    }

    #[test]
    fn heap_binomial_peek_max_empty_heap() {
        let bh1 = BinomialHeap::init_max(0);
        let bh2 = BinomialHeap::init_max(1);

        let mut merged_heap = BinomialHeap::merge(bh1, bh2);

        merged_heap.pop();
        merged_heap.pop();
        assert_eq!(*merged_heap.peek(), None);
        assert_eq!(
            BinomialHeap::preorder(&merged_heap),
            format!("Rank 0: \nRank 1: \n")
        );
    }

    #[test]
    #[should_panic(expected = "Both binomial heaps must be of the same type(both min or both max)")]
    fn heap_binomial_panic_merge() {
        let bh1 = BinomialHeap::init_min(0);
        let bh2 = BinomialHeap::init_max(1);
        BinomialHeap::merge(bh1, bh2);
    }
}
