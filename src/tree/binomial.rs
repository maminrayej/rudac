/// A binomial tree of rank(order) k is a general tree with a recursive definition
///
/// B<sub>k</sub>:
/// * k = 0: consists of only one node which is the root
/// * k > 0: consists of one root and children of binomial trees of degress {B<sub>0</sub>, B<sub>1</sub>, ... , B<sub>k-1</sub>}
///
/// # Examples
/// ```
/// use rudac::tree::BinomialTree;
///
/// // create two min binomial trees of rank 0
/// let bt1 = BinomialTree::init_min(0);
/// let bt2 = BinomialTree::init_min(1);
///
/// // merge two trees and get a binomial tree of rank 1
/// let merged_tree = BinomialTree::merge(bt1, bt2);
///
/// // preorder traveral of the heap is equal to 0 1
/// assert_eq!(BinomialTree::preorder(&merged_tree), String::from("0 1"));
/// assert_eq!(merged_tree.rank(), 1);
/// ```
///
#[derive(Debug)]
pub struct BinomialTree<T: std::cmp::Ord> {
    // rank of the tree
    rank: usize,

    // children of the current node
    children: Vec<Option<BinomialTree<T>>>,

    // contents of the node
    payload: Option<T>,

    // indicates wether the binomial tree is a min or max one
    min: bool,
}

impl<T: std::cmp::Ord> BinomialTree<T> {
    /// Creates a min binomial tree with rank 0 which holds the `payload`.
    /// in this binomial tree each node is smaller than its children
    ///
    /// # Arguments
    /// * `payload` - data stored inside the node
    ///
    /// # Examples
    /// ```
    /// use rudac::tree::BinomialTree;
    ///
    /// let bt1 = BinomialTree::init_min(0);
    /// let bt2 = BinomialTree::init_min(1);
    ///
    /// let merged_tree = BinomialTree::merge(bt1, bt2);
    ///
    /// assert_eq!(BinomialTree::preorder(&merged_tree), String::from("0 1"))
    /// ```
    pub fn init_min(payload: T) -> BinomialTree<T> {
        BinomialTree::init(payload, true)
    }

    /// Creates a max binomial tree with rank 0 which holds the `payload`.
    /// in this binomial tree each node is greater than its children
    ///
    /// # Arguments
    /// * `payload` - data stored inside the node
    ///
    /// # Examples
    /// ```
    /// use rudac::tree::BinomialTree;
    ///
    /// let bt1 = BinomialTree::init_max(0);
    /// let bt2 = BinomialTree::init_max(1);
    ///
    /// let merged_tree = BinomialTree::merge(bt1, bt2);
    ///
    /// assert_eq!(BinomialTree::preorder(&merged_tree), String::from("1 0"))
    /// ```
    pub fn init_max(payload: T) -> BinomialTree<T> {
        BinomialTree::init(payload, false)
    }

    // initializes the min or max binomial tree based on `min` argument
    fn init(payload: T, min: bool) -> BinomialTree<T> {
        BinomialTree {
            rank: 0,
            children: Vec::new(),
            payload: Some(payload),
            min,
        }
    }

    // adds a binomial tree as the rightmost child of the current binomial tree
    // this method is called from `merge` method thus compatablity between these two trees has been checked
    // if you call this function directly you sould check compatability between ranks and types of these two trees
    fn add(&mut self, binomial_tree: BinomialTree<T>) {
        // add the binomial tree as the rightmost child
        self.children.push(Some(binomial_tree));

        // merged tree of two binomial trees of rank k has rank k + 1
        self.rank += 1;
    }

    /// Merges two binomial trees
    ///
    /// # Arguments
    /// * `binomial_tree_1` - first binomial tree
    /// * `binomial_tree_2` - second binomial tree
    ///
    /// # Panics
    /// * panics if rank of two binomial trees are not the same
    ///
    /// # Examples
    /// ```
    /// use rudac::tree::BinomialTree;
    ///
    /// // create two binomial trees of rank 0
    /// let bt1 = BinomialTree::init_min("ru");
    /// let bt2 = BinomialTree::init_min("dac");
    ///
    /// // merge two trees and get a binomial tree of rank 1
    /// let merged_tree = BinomialTree::merge(bt1, bt2);
    ///
    /// // preorder traveral of the heap is equal to "dac" "ru"
    /// assert_eq!(BinomialTree::preorder(&merged_tree), "dac ru");
    /// assert_eq!(merged_tree.rank(), 1);
    /// ```
    pub fn merge(
        mut binomial_tree_1: BinomialTree<T>,
        mut binomial_tree_2: BinomialTree<T>,
    ) -> BinomialTree<T> {
        // rank compatability check
        if binomial_tree_1.rank() != binomial_tree_2.rank {
            panic!("Binomial tree ranks must be the same when merging");
        }

        // type compatability check
        if binomial_tree_1.is_min() != binomial_tree_2.is_min() {
            panic!("Both binomial trees must be of the same type(both min or both max)");
        }

        // trees_are_min indicates wether the comparison is between two min trees or two max trees
        let trees_are_min = binomial_tree_1.is_min();

        // if two trees are min binomial trees the minimum must become the new root
        // if two trees are max binomial trees the maximum must become the new root
        if (trees_are_min && BinomialTree::is_smaller_or_equall(&binomial_tree_1, &binomial_tree_2))
            || (!trees_are_min
                && BinomialTree::is_smaller_or_equall(&binomial_tree_2, &binomial_tree_1))
        {
            binomial_tree_1.add(binomial_tree_2);
            return binomial_tree_1;
        } else {
            binomial_tree_2.add(binomial_tree_1);
            return binomial_tree_2;
        }
    }

    /// Returns rank of the binomial tree
    ///
    /// # Examples
    /// ```
    /// use rudac::tree::BinomialTree;
    ///
    /// let bt = BinomialTree::init_min("rudac");
    ///
    /// assert_eq!(bt.rank(), 0);
    /// ```
    pub fn rank(&self) -> usize {
        self.rank
    }

    /// Returns true if binomial tree is initialized as a min binomial tree
    ///
    /// # Examples
    /// ```
    /// use rudac::tree::BinomialTree;
    ///
    /// let bt1 = BinomialTree::init_min("rudac");
    /// let bt2 = BinomialTree::init_max("rudac");
    ///
    /// assert_eq!(bt1.is_min(), true);
    /// assert_eq!(bt2.is_min(), false);
    /// ```
    pub fn is_min(&self) -> bool {
        self.min
    }

    /// Extracts and returns the payload from the node and replaces it with None
    ///
    /// # Panics
    /// * panics if payload is None
    ///
    /// # Examples
    /// ```
    /// use rudac::tree::BinomialTree;
    ///
    /// let mut bt = BinomialTree::init_min("rudac");
    ///
    /// assert_eq!(bt.get_payload(), "rudac");
    /// ```
    pub fn get_payload(&mut self) -> T {
        if self.payload == None {
            panic!("Payload is None");
        }

        self.payload.take().unwrap()
    }

    /// Returns a refrence to payload
    ///
    /// # Examples
    /// ```
    /// use rudac::tree::BinomialTree;
    ///
    /// let mut bt = BinomialTree::init_min("rudac");
    ///
    /// assert_eq!(*bt.peek_payload(), Some("rudac"));
    /// ```
    pub fn peek_payload(&self) -> &Option<T> {
        &self.payload
    }

    /// Compares payloads which reside in roots of two binomial trees `first` and `other`.
    /// Returns True if payload of `first` is smaller or equall than payload of `other`
    ///
    /// # Examples
    /// ```
    /// use rudac::tree::BinomialTree;
    ///
    /// let bt1 = BinomialTree::init_min(0);
    /// let bt2 = BinomialTree::init_min(1);
    ///
    /// assert_eq!(true, BinomialTree::is_smaller_or_equall(&bt1, &bt2));
    /// assert_eq!(false, BinomialTree::is_smaller_or_equall(&bt2, &bt1));
    /// ```
    pub fn is_smaller_or_equall(first: &BinomialTree<T>, other: &BinomialTree<T>) -> bool {
        match (first.peek_payload(), other.peek_payload()) {
            (Some(payload1), Some(payload2)) => payload1 <= payload2,
            _ => panic!("Payloads can not be None"), // if one of the payloads or both of them are None
        }
    }

    /// Returns a mutable reference to vector of children
    pub fn children_mut(&mut self) -> &mut Vec<Option<BinomialTree<T>>> {
        &mut self.children
    }

    /// Returns a immutable reference to vector of childre
    pub fn children(&self) -> &Vec<Option<BinomialTree<T>>> {
        &self.children
    }
}

impl<T: std::cmp::Ord + std::fmt::Display> BinomialTree<T> {
    /// Returns the preorder representation of the heap
    ///
    /// # Arguments
    /// * `root`: root of the binomial tree
    ///
    /// # Examples
    /// ```
    /// use rudac::tree::BinomialTree;
    ///
    /// let bt1 = BinomialTree::init_min(0);
    /// let bt2 = BinomialTree::init_min(1);
    /// let merged_tree_1 = BinomialTree::merge(bt1, bt2);
    ///
    /// let bt3 = BinomialTree::init_min(2);
    /// let bt4 = BinomialTree::init_min(3);
    /// let merged_tree_2 = BinomialTree::merge(bt3, bt4);
    ///
    /// let merged_tree = BinomialTree::merge(merged_tree_1, merged_tree_2);
    ///
    /// assert_eq!(
    ///     BinomialTree::preorder(&merged_tree),
    ///     String::from("0 1 2 3")
    /// );
    /// ```
    pub fn preorder(root: &BinomialTree<T>) -> String {
        return String::from(BinomialTree::_pre_visit(&Some(root)).trim());
    }

    // traverses the tree in a preorder fashion
    fn _pre_visit(node: &Option<&BinomialTree<T>>) -> String {
        // list of nodes visited starting from the current node
        let mut node_list = String::from("");

        match node {
            None => node_list, // if node is None then it does not have any nodes to visit. return ""
            Some(data) => {
                // visit the node
                match data.peek_payload() {
                    Some(value) => node_list.push_str(format!("{} ", value).as_str()), // add the payload representation
                    None => (), // if payload of the node is empty there is nothing to display in this node
                }
                //visit children from left to right
                for i in 0..data.children.len() {
                    match &data.children[i] {
                        Some(data) => {
                            // store node lists returned by preorder traversal of each child
                            node_list.push_str(BinomialTree::_pre_visit(&Some(&data)).as_str())
                        }
                        None => (), // if child is None don't visit it
                    }
                }

                node_list
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binomial_tree_create() {
        let bt = BinomialTree::init_min(0);

        assert_eq!(*bt.peek_payload(), Some(0));
        assert_eq!(bt.rank, 0);
        assert_eq!(bt.children.len(), 0);
        assert_eq!(BinomialTree::preorder(&bt), String::from("0"));
    }

    #[test]
    fn binomial_tree_merge_rank_0() {
        let bt1 = BinomialTree::init_min(0);
        let bt2 = BinomialTree::init_min(1);

        let merged_tree = BinomialTree::merge(bt1, bt2);

        assert_eq!(BinomialTree::preorder(&merged_tree), String::from("0 1"));

        assert_eq!(merged_tree.rank(), 1);
    }

    #[test]
    fn binomial_tree_merge_rank_0_ref() {
        let bt1 = BinomialTree::init_min(String::from("a"));
        let bt2 = BinomialTree::init_min(String::from("b"));

        let merged_tree = BinomialTree::merge(bt1, bt2);

        assert_eq!(BinomialTree::preorder(&merged_tree), String::from("a b"));

        assert_eq!(merged_tree.rank(), 1);
    }

    #[test]
    fn binomial_tree_merge_rank_1() {
        let bt1 = BinomialTree::init_min(0);
        let bt2 = BinomialTree::init_min(1);

        let merged_tree_1 = BinomialTree::merge(bt1, bt2);
        let bt3 = BinomialTree::init_min(2);
        let bt4 = BinomialTree::init_min(3);

        let merged_tree_2 = BinomialTree::merge(bt3, bt4);

        let merged_tree = BinomialTree::merge(merged_tree_1, merged_tree_2);

        assert_eq!(
            BinomialTree::preorder(&merged_tree),
            String::from("0 1 2 3")
        );
        assert_eq!(merged_tree.rank(), 2);
    }

    #[test]
    fn binomial_tree_merge_rank_2() {
        let bt1 = BinomialTree::init_min(0);
        let bt2 = BinomialTree::init_min(1);

        let merged_tree_1 = BinomialTree::merge(bt1, bt2);
        let bt3 = BinomialTree::init_min(2);
        let bt4 = BinomialTree::init_min(3);

        let merged_tree_2 = BinomialTree::merge(bt3, bt4);

        let merged_tree_final_1 = BinomialTree::merge(merged_tree_1, merged_tree_2);

        let bt1 = BinomialTree::init_min(1);
        let bt2 = BinomialTree::init_min(2);

        let merged_tree_1 = BinomialTree::merge(bt1, bt2);
        let bt3 = BinomialTree::init_min(5);
        let bt4 = BinomialTree::init_min(6);

        let merged_tree_2 = BinomialTree::merge(bt3, bt4);

        let merged_tree_final_2 = BinomialTree::merge(merged_tree_1, merged_tree_2);

        let merged_tree = BinomialTree::merge(merged_tree_final_1, merged_tree_final_2);

        assert_eq!(
            BinomialTree::preorder(&merged_tree),
            String::from("0 1 2 3 1 2 5 6")
        );
        assert_eq!(merged_tree.rank(), 3);
    }

    #[test]
    fn binomial_tree_merge_rank_2_max() {
        let bt1 = BinomialTree::init_max(0);
        let bt2 = BinomialTree::init_max(1);

        let merged_tree_1 = BinomialTree::merge(bt1, bt2);
        let bt3 = BinomialTree::init_max(2);
        let bt4 = BinomialTree::init_max(3);

        let merged_tree_2 = BinomialTree::merge(bt3, bt4);

        let merged_tree_final_1 = BinomialTree::merge(merged_tree_1, merged_tree_2);

        let bt1 = BinomialTree::init_max(1);
        let bt2 = BinomialTree::init_max(2);

        let merged_tree_1 = BinomialTree::merge(bt1, bt2);
        let bt3 = BinomialTree::init_max(5);
        let bt4 = BinomialTree::init_max(6);

        let merged_tree_2 = BinomialTree::merge(bt3, bt4);

        let merged_tree_final_2 = BinomialTree::merge(merged_tree_1, merged_tree_2);

        let merged_tree = BinomialTree::merge(merged_tree_final_1, merged_tree_final_2);

        assert_eq!(
            BinomialTree::preorder(&merged_tree),
            String::from("6 5 2 1 3 2 1 0")
        );
        assert_eq!(merged_tree.rank(), 3);
    }

    #[test]
    #[should_panic(expected = "Both binomial trees must be of the same type(both min or both max)")]
    fn binomial_tree_merge_min_max() {
        let min_bt = BinomialTree::init_min(0);
        let max_bt = BinomialTree::init_max(0);

        BinomialTree::merge(min_bt, max_bt);
    }

    #[test]
    #[should_panic(expected = "Binomial tree ranks must be the same when merging")]
    fn binomial_tree_merge_different_ranks() {
        let bt1 = BinomialTree::init_min(0);
        let bt2 = BinomialTree::init_min(0);
        let bt3 = BinomialTree::init_min(0);

        let merged_tree = BinomialTree::merge(bt1, bt2);

        BinomialTree::merge(bt3, merged_tree);
    }

    #[test]
    #[should_panic(expected = "Payload is None")]
    fn binomial_tree_get_payload_panic() {
        let mut bt1 = BinomialTree::init_min(0);

        bt1.get_payload();
        bt1.get_payload();
    }
}
