use std::collections::VecDeque;

const RED: bool = true;
const BLACK: bool = false;

struct Node<K: std::cmp::Ord, V> {
    key: Option<K>,
    value: Option<V>,
    color: bool,
    size: usize,
    left_child: Option<Box<Node<K, V>>>,
    right_child: Option<Box<Node<K, V>>>,
}

impl<K: std::cmp::Ord, V> Node<K, V> {
    fn init(key: K, value: V, color: bool, size: usize) -> Node<K, V> {
        Node {
            key: Some(key),
            value: Some(value),
            color: color,
            size: size,
            left_child: None,
            right_child: None,
        }
    }

    fn key(&self) -> &K {
        &self.key.as_ref().unwrap()
    }
    fn key_mut(&mut self) -> &mut Option<K> {
        &mut self.key
    }

    fn get_key(&mut self) -> K {
        self.key.take().unwrap()
    }

    fn value(&self) -> &V {
        &self.value.as_ref().unwrap()
    }

    fn value_mut(&mut self) -> &mut Option<V> {
        &mut self.value
    }

    fn get_value(&mut self) -> V {
        self.value.take().unwrap()
    }

    fn left_child(&self) -> &Box<Node<K, V>> {
        self.left_child.as_ref().unwrap()
    }

    fn right_child(&self) -> &Box<Node<K, V>> {
        self.right_child.as_ref().unwrap()
    }

    fn is_red(node: &Option<Box<Node<K, V>>>) -> bool {
        if node.is_none() {
            return false;
        }
        node.as_ref().unwrap().color == RED
    }

    fn size(node: &Option<Box<Node<K, V>>>) -> usize {
        if node.is_none() {
            return 0;
        }
        node.as_ref().unwrap().size
    }

    fn update_size(&mut self) {
        self.size = Node::size(&self.left_child) + Node::size(&self.right_child) + 1;
    }
}

/// A Red Black tree is a self-balancing binary search tree.
/// Red Black Trees provide faster insertion and removal operations than AVL trees
/// 
/// # Examples
/// ```
/// use rudac::tree::RedBlack;
/// 
/// // initialize a Red Black tree with keys of type usize and values of type String
/// let mut rb_tree = RedBlack::<usize, String>::init();
/// 
/// // insert items into tree
/// rb_tree.insert(1, String::from("rudac"));
/// rb_tree.insert(2, String::from("is"));
/// rb_tree.insert(3, String::from("awesome"));
/// rb_tree.insert(4, String::from("!"));
/// 
/// // lookup for items
/// assert_eq!(*rb_tree.get(&1).unwrap(), String::from("rudac"));
/// assert_eq!(*rb_tree.get(&2).unwrap(), String::from("is"));
/// assert_eq!(*rb_tree.get(&3).unwrap(), String::from("awesome"));
/// assert_eq!(*rb_tree.get(&4).unwrap(), String::from("!"));
/// 
/// // delete items from tree
/// rb_tree.delete(&4);
/// assert_eq!(rb_tree.get(&4), None);
/// ```
pub struct RedBlack<K: std::cmp::Ord, V> {
    root: Option<Box<Node<K, V>>>,
}

impl<K: std::cmp::Ord, V> RedBlack<K, V> {
    /// Initializes an empty Red Black tree
    /// 
    /// # Examples
    /// ```
    /// use rudac::tree::RedBlack;
    /// 
    /// let usize_to_string = RedBlack::<usize, String>::init();
    /// 
    /// let string_to_usize = RedBlack::<String, usize>::init();
    /// 
    /// let string_to_string = RedBlack::<String, String>::init();
    /// ```
    pub fn init() -> RedBlack<K, V> {
        RedBlack { root: None }
    }

    /// Returns total number of nodes in the tree
    /// 
    /// # Examples
    /// ```
    /// use rudac::tree::RedBlack;
    /// 
    /// let mut rb_tree = RedBlack::<usize,usize>::init();
    /// assert_eq!(rb_tree.size(), 0);
    /// 
    /// rb_tree.insert(1,1);
    /// rb_tree.insert(2,4);
    /// rb_tree.insert(3,8);
    /// assert_eq!(rb_tree.size(), 3);
    /// ```
    pub fn size(&self) -> usize {
        Node::size(&self.root)
    }

    /// Returns `true` if tree is empty and `false` otherwise
    /// 
    /// # Examples
    /// ```
    /// use rudac::tree::RedBlack;
    /// 
    /// let mut rb_tree = RedBlack::<usize,usize>::init();
    /// assert_eq!(rb_tree.is_empty(), true);
    /// 
    /// rb_tree.insert(1,1);
    /// assert_eq!(rb_tree.is_empty(), false);
    /// 
    /// rb_tree.delete(&1);
    /// assert_eq!(rb_tree.is_empty(), true);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    /// Returns a reference to value associated with specified `key` in tree, `None` otherwise
    /// # Arguments
    /// * `key`: key to be searched in the tree
    /// 
    /// # Examples
    /// ```
    /// use rudac::tree::RedBlack;
    /// 
    /// let mut rb_tree = RedBlack::<usize,usize>::init();
    /// 
    /// rb_tree.insert(1,10);
    /// assert_eq!(*rb_tree.get(&1).unwrap(), 10);
    /// 
    /// rb_tree.delete(&1);
    /// assert_eq!(rb_tree.get(&1), None);
    /// ```
    pub fn get(&self, key: &K) -> Option<&V> {
        RedBlack::_get(&self.root, key)
    }

    fn _get<'a>(mut node: &'a Option<Box<Node<K, V>>>, key: &K) -> Option<&'a V> {
        while !node.is_none() {
            let node_ref = node.as_ref().unwrap();
            if key < node_ref.key() {
                node = &node_ref.left_child
            } else if key > node_ref.key() {
                node = &node_ref.right_child
            } else {
                return Some(node_ref.value());
            }
        }
        None
    }

    /// Returns `true` if tree contains the specified `key`, false otherwise
    /// 
    /// # Arguments
    /// * `key`: key to be searched in the tree
    /// 
    /// # Examples
    /// ```
    /// use rudac::tree::RedBlack;
    /// 
    /// let mut rb_tree = RedBlack::<usize,usize>::init();
    /// 
    /// rb_tree.insert(1,10);
    /// assert_eq!(rb_tree.contains(&1), true);
    /// 
    /// rb_tree.delete(&1);
    /// assert_eq!(rb_tree.contains(&1), false);
    /// ```
    pub fn contains(&self, key: &K) -> bool {
        !self.get(key).is_none()
    }

    /// Insert a node which contains the specified `key` and `value` into the tree.
    /// if `key` already exists, this method will replace `value` as the new value of the node
    /// 
    /// # Arguments
    /// * `key`: key of the new node
    /// * `value`: value associated with the `key`
    /// 
    /// # Examples
    /// ```
    /// use rudac::tree::RedBlack;
    /// 
    /// let mut rb_tree = RedBlack::<usize,usize>::init();
    /// 
    /// rb_tree.insert(1,10);
    /// rb_tree.insert(2,20);
    /// rb_tree.insert(3,30);
    /// rb_tree.insert(4,40);
    /// assert_eq!(*rb_tree.get(&1).unwrap(), 10);
    /// 
    /// rb_tree.insert(1,11);
    /// assert_eq!(*rb_tree.get(&1).unwrap(), 11);
    /// ```
    pub fn insert(&mut self, key: K, value: V) {
        let mut root = RedBlack::_insert(self.root.take(), key, value).unwrap();

        root.color = BLACK;

        self.root = Some(root);
    }

    fn _insert(node: Option<Box<Node<K, V>>>, key: K, value: V) -> Option<Box<Node<K, V>>> {
        if node.is_none() {
            return Some(Box::new(Node::init(key, value, RED, 1)));
        }

        let mut node_ref = node.unwrap();

        if key < *node_ref.key() {
            node_ref.left_child = RedBlack::_insert(node_ref.left_child, key, value);
        } else if key > *node_ref.key() {
            node_ref.right_child = RedBlack::_insert(node_ref.right_child, key, value);
        } else {
            node_ref.value = Some(value);
        }

        // balance the tree
        if Node::is_red(&node_ref.right_child) && !Node::is_red(&node_ref.left_child) {
            node_ref = RedBlack::rotate_left(node_ref);
        }
        if Node::is_red(&node_ref.left_child) && Node::is_red(&node_ref.left_child().left_child) {
            node_ref = RedBlack::rotate_right(node_ref);
        }
        if Node::is_red(&node_ref.left_child) && Node::is_red(&node_ref.right_child) {
            RedBlack::flip_colors(&mut node_ref);
        }

        node_ref.update_size();

        Some(node_ref)
    }

    /// Deletes node with smallest key from the tree
    /// 
    /// # Examples
    /// ```
    /// use rudac::tree::RedBlack;
    /// 
    /// let mut rb_tree = RedBlack::<usize,usize>::init();
    /// 
    /// rb_tree.insert(1,10);
    /// rb_tree.insert(2,20);
    /// rb_tree.insert(3,30);
    /// rb_tree.insert(4,40);
    /// 
    /// rb_tree.delete_min();
    /// assert_eq!(rb_tree.get(&1), None);
    /// 
    /// 
    /// rb_tree.delete_min();
    /// assert_eq!(rb_tree.get(&2), None);
    /// ```
    pub fn delete_min(&mut self) {
        if self.root.is_none() {
            return;
        }

        let mut root_ref = self.root.take().unwrap();

        if !Node::is_red(&root_ref.left_child) && !Node::is_red(&root_ref.right_child) {
            root_ref.color = RED;
        }

        let mut root = RedBlack::_delete_min(Some(root_ref));

        if !root.is_none() {
            root_ref = root.unwrap();
            root_ref.color = BLACK;
            root = Some(root_ref)
        }

        self.root = root;
    }

    fn _delete_min(node: Option<Box<Node<K, V>>>) -> Option<Box<Node<K, V>>> {
        if node.as_ref().unwrap().left_child.is_none() {
            return None;
        }

        let mut node_ref = node.unwrap();

        if !Node::is_red(&node_ref.left_child) && !Node::is_red(&node_ref.left_child().left_child) {
            node_ref = RedBlack::move_red_left(node_ref);
        }

        node_ref.left_child = RedBlack::_delete_min(node_ref.left_child);

        Some(RedBlack::balance(node_ref))
    }

    /// Deletes node with largest key from the tree
    /// 
    /// # Examples
    /// ```
    /// use rudac::tree::RedBlack;
    /// 
    /// let mut rb_tree = RedBlack::<usize,usize>::init();
    /// 
    /// rb_tree.insert(1,10);
    /// rb_tree.insert(2,20);
    /// rb_tree.insert(3,30);
    /// rb_tree.insert(4,40);
    /// 
    /// rb_tree.delete_max();
    /// assert_eq!(rb_tree.get(&4), None);
    /// 
    /// 
    /// rb_tree.delete_max();
    /// assert_eq!(rb_tree.get(&3), None);
    /// ```
    pub fn delete_max(&mut self) {
        if self.root.is_none() {
            return;
        }

        let mut root_ref = self.root.take().unwrap();

        if !Node::is_red(&root_ref.left_child) && !Node::is_red(&root_ref.right_child) {
            root_ref.color = RED;
        }

        let mut root = RedBlack::_delete_max(Some(root_ref));

        if !root.is_none() {
            root_ref = root.unwrap();
            root_ref.color = BLACK;
            root = Some(root_ref)
        }

        self.root = root;
    }

    fn _delete_max(node: Option<Box<Node<K, V>>>) -> Option<Box<Node<K, V>>> {
        if node.is_none() {
            return None;
        }

        let mut node_ref = node.unwrap();

        if Node::is_red(&node_ref.left_child) {
            node_ref = RedBlack::rotate_right(node_ref);
        }

        if node_ref.right_child.is_none() {
            return None;
        }

        if !Node::is_red(&node_ref.right_child) && !Node::is_red(&node_ref.right_child().left_child)
        {
            node_ref = RedBlack::move_red_right(node_ref);
        }

        node_ref.right_child = RedBlack::_delete_max(node_ref.right_child);

        Some(RedBlack::balance(node_ref))
    }

    /// Deletes the node containing the specified `key`
    /// 
    /// # Arguments
    /// * `key`: key of the node to be deleted from the tree
    /// 
    /// # Examples
    /// ```
    /// use rudac::tree::RedBlack;
    /// 
    /// let mut rb_tree = RedBlack::<usize,usize>::init();
    /// 
    /// rb_tree.insert(1,10);
    /// rb_tree.insert(2,20);
    /// rb_tree.insert(3,30);
    /// rb_tree.insert(4,40);
    /// 
    /// rb_tree.delete(&1);
    /// assert_eq!(rb_tree.get(&1), None);
    /// ```
    pub fn delete(&mut self, key: &K) {
        if self.root.is_none() || !self.contains(key) {
            return;
        }

        let mut root_ref = self.root.take().unwrap();

        if !Node::is_red(&root_ref.left_child) && !Node::is_red(&root_ref.right_child) {
            root_ref.color = RED;
        }

        let mut root = RedBlack::_delete(Some(root_ref), key);

        if !root.is_none() {
            root_ref = root.unwrap();
            root_ref.color = BLACK;
            root = Some(root_ref)
        }

        self.root = root;
    }

    fn _delete(node: Option<Box<Node<K, V>>>, key: &K) -> Option<Box<Node<K, V>>> {
        if node.is_none() {
            return None;
        }

        let mut node_ref = node.unwrap();

        if *key < *node_ref.key() {
            if !Node::is_red(&node_ref.left_child)
                && Node::is_red(&node_ref.left_child().left_child)
            {
                node_ref = RedBlack::move_red_left(node_ref);
            }
            node_ref.left_child = RedBlack::_delete(node_ref.left_child, key);
        } else {
            if Node::is_red(&node_ref.left_child) {
                node_ref = RedBlack::rotate_right(node_ref);
            }
            if *key == *node_ref.key() && node_ref.right_child.is_none() {
                return None;
            }
            if !Node::is_red(&node_ref.right_child)
                && !Node::is_red(&node_ref.right_child().left_child)
            {
                node_ref = RedBlack::move_red_right(node_ref);
            }
            if *key == *node_ref.key() {
                let mut x = RedBlack::_min(&mut node_ref.right_child);
                // swap keys
                std::mem::swap(x.key_mut(), node_ref.key_mut());

                // swap values
                std::mem::swap(x.value_mut(), node_ref.value_mut());

                node_ref.right_child = RedBlack::_delete_min(node_ref.right_child);
            } else {
                node_ref.right_child = RedBlack::_delete(node_ref.right_child, key);
            }
        }

        Some(RedBlack::balance(node_ref))
    }

    fn _min(node: &mut Option<Box<Node<K, V>>>) -> Box<Node<K, V>> {
        match node {
            None => panic!("Called min on None node"),
            Some(_node) => {
                if _node.left_child.is_none() {
                    return Box::new(Node::init(_node.get_key(), _node.get_value(), RED, 1));
                } else {
                    return RedBlack::_min(&mut _node.left_child);
                }
            }
        }
    }

    /// Returns the height of the tree.
    /// An empty tree has height -1 and a tree with one node has height 0
    pub fn height(&self) -> i64 {
        RedBlack::_height(&self.root)
    }

    fn _height(node: &Option<Box<Node<K, V>>>) -> i64 {
        if node.is_none() {
            return -1;
        }

        let node_ref = node.as_ref().unwrap();

        return 1 + std::cmp::max(
            RedBlack::_height(&node_ref.left_child),
            RedBlack::_height(&node_ref.right_child),
        );
    }

    /// Returns the largest key in the tree less than or equal to `key`
    /// 
    /// # Arguments
    /// * `key`: key to be searched for
    /// 
    /// # Examples:
    /// ```
    /// use rudac::tree::RedBlack;
    /// 
    /// let mut rb_tree = RedBlack::<usize,usize>::init();
    /// 
    /// rb_tree.insert(1,10);
    /// rb_tree.insert(3,20);
    /// rb_tree.insert(5,30);
    /// rb_tree.insert(7,40);
    /// 
    /// assert_eq!(*rb_tree.floor(&2).unwrap(), 1);
    /// assert_eq!(rb_tree.floor(&0), None);
    /// ```
    pub fn floor(&self, key: &K) -> Option<&K> {
        RedBlack::_floor(&self.root, key)
    }

    fn _floor<'a>(node: &'a Option<Box<Node<K, V>>>, key: &K) -> Option<&'a K> {
        if node.is_none() {
            return None;
        }

        let node_ref = node.as_ref().unwrap();

        if *key == *node_ref.key() {
            return Some(node_ref.key());
        }
        if *key < *node_ref.key() {
            return RedBlack::_floor(&node_ref.left_child, key);
        }

        let found_key = RedBlack::_floor(&node_ref.right_child, key);

        if found_key.is_some() {
            return found_key;
        } else {
            return Some(node_ref.key());
        }
    }

    /// Returns the smallest key in the tree greater than or equal to `key`
    /// 
    /// # Arguments
    /// * `key`: key to be searched for
    /// 
    /// # Examples:
    /// ```
    /// use rudac::tree::RedBlack;
    /// 
    /// let mut rb_tree = RedBlack::<usize,usize>::init();
    /// 
    /// rb_tree.insert(1,10);
    /// rb_tree.insert(3,20);
    /// rb_tree.insert(5,30);
    /// rb_tree.insert(7,40);
    /// 
    /// assert_eq!(*rb_tree.ceiling(&6).unwrap(), 7);
    /// assert_eq!(rb_tree.ceiling(&8), None);
    /// ```
    pub fn ceiling(&self, key: &K) -> Option<&K> {
        RedBlack::_ceiling(&self.root, key)
    }

    fn _ceiling<'a>(node: &'a Option<Box<Node<K, V>>>, key: &K) -> Option<&'a K> {
        if node.is_none() {
            return None;
        }

        let node_ref = node.as_ref().unwrap();

        if *key == *node_ref.key() {
            return Some(node_ref.key());
        }
        if *key > *node_ref.key() {
            return RedBlack::_ceiling(&node_ref.right_child, key);
        }

        let found_key = RedBlack::_ceiling(&node_ref.left_child, key);

        if found_key.is_some() {
            return found_key;
        } else {
            return Some(node_ref.key());
        }
    }

    /// Returns the kth smallest key and its associated value in the tree
    /// 
    /// # Arguments
    /// * `k`: the order statistic
    /// 
    /// # Panics
    /// * panics if k is not in range: 0 <= k <= size - 1
    /// 
    /// # Examples
    /// ```
    /// use rudac::tree::RedBlack;
    /// 
    /// let mut rb_tree = RedBlack::<usize,usize>::init();
    /// 
    /// rb_tree.insert(1,10);
    /// rb_tree.insert(3,20);
    /// rb_tree.insert(5,30);
    /// rb_tree.insert(7,40);
    /// 
    /// let (key, value) = rb_tree.select(1).unwrap();
    /// assert_eq!(*key, 3);
    /// assert_eq!(*value, 20);
    /// ```
    pub fn select(&self, k: usize) -> Option<(&K, &V)> {
        if k > self.size() {
            panic!("K must be in range 0 <= k <= size - 1");
        }
        RedBlack::_select(&self.root, k)
    }

    fn _select(node: &Option<Box<Node<K, V>>>, k: usize) -> Option<(&K, &V)> {
        if node.is_none() {
            return None;
        }

        let node_ref = node.as_ref().unwrap();

        let left_size = Node::size(&node_ref.left_child);

        if left_size > k {
            return RedBlack::_select(&node_ref.left_child, k);
        } else if left_size < k {
            return RedBlack::_select(&node_ref.right_child, k - left_size - 1);
        } else {
            return Some((node_ref.key(), node_ref.value()));
        }
    }

    /// Returns the smallest key and its associated value in the tree
    /// 
    /// # Examples
    /// ```
    /// use rudac::tree::RedBlack;
    /// 
    /// let mut rb_tree = RedBlack::<usize,usize>::init();
    /// 
    /// rb_tree.insert(1,10);
    /// rb_tree.insert(3,20);
    /// rb_tree.insert(5,30);
    /// rb_tree.insert(7,40);
    /// 
    /// let (key, value) = rb_tree.min().unwrap();
    /// assert_eq!(*key, 1);
    /// assert_eq!(*value, 10);
    /// 
    /// rb_tree.delete_min();
    /// 
    /// let (key, value) = rb_tree.min().unwrap();
    /// assert_eq!(*key, 3);
    /// assert_eq!(*value, 20);
    /// ```
    pub fn min(&self) -> Option<(&K, &V)> {
        self.select(0)
    }

    /// Returns the largest key and its associated value in the tree
    /// 
    /// # Examples
    /// ```
    /// use rudac::tree::RedBlack;
    /// 
    /// let mut rb_tree = RedBlack::<usize,usize>::init();
    /// 
    /// rb_tree.insert(1,10);
    /// rb_tree.insert(3,20);
    /// rb_tree.insert(5,30);
    /// rb_tree.insert(7,40);
    /// 
    /// let (key, value) = rb_tree.max().unwrap();
    /// assert_eq!(*key, 7);
    /// assert_eq!(*value, 40);
    /// 
    /// rb_tree.delete_max();
    /// 
    /// let (key, value) = rb_tree.max().unwrap();
    /// assert_eq!(*key, 5);
    /// assert_eq!(*value, 30);
    /// ```
    pub fn max(&self) -> Option<(&K, &V)> {
        self.select(self.size()-1)
    }

    /// Returns the number of keys in the symbol table strictly less than `key`
    /// 
    /// # Arguments
    /// * `key`: key to be searched for
    /// 
    /// # Examples
    /// ```
    /// use rudac::tree::RedBlack;
    /// 
    /// let mut rb_tree = RedBlack::<usize, usize>::init();
    /// 
    /// for i in 1..100 {
    ///     rb_tree.insert(i, i);
    /// }
    /// 
    /// assert_eq!(rb_tree.rank(&99), 98);
    /// ```
    pub fn rank(&self, key: &K) -> usize {
        RedBlack::_rank(&self.root, key)
    }

    fn _rank(node: &Option<Box<Node<K, V>>>, key: &K) -> usize {
        if node.is_none() {
            return 0;
        }

        let node_ref = node.as_ref().unwrap();

        if *key < *node_ref.key() {
            return RedBlack::_rank(&node_ref.left_child, key);
        } else if *key > *node_ref.key() {
            return 1
                + Node::size(&node_ref.left_child)
                + RedBlack::_rank(&node_ref.right_child, key);
        } else {
            return Node::size(&node_ref.left_child);
        }
    }

    /// Returns all keys in the tree following an in-order traversal. 
    /// Therefore keys are sorted from smallest to largest
    /// 
    /// # Examples
    /// ```
    /// use rudac::tree::RedBlack;
    /// 
    /// let mut rb_tree = RedBlack::<usize, usize>::init();
    /// 
    /// for i in (1..100).rev() {
    ///     rb_tree.insert(i, i);
    /// }
    /// 
    /// let mut i = 1;
    /// // keys are sorted: [1, 2, 3,..., 99]
    /// for key in rb_tree.keys() {
    ///     assert!(*key == i);
    ///     i += 1;
    /// } 
    /// ```
    pub fn keys(&self) -> Vec<&K> {
        let mut keys: Vec<&K> = Vec::new();

        RedBlack::_keys_in_order(&self.root, &mut keys);

        keys
    }

    fn _keys_in_order<'a>(node: &'a Option<Box<Node<K, V>>>, keys: &mut Vec<&'a K>) {
        if node.is_none() {
            return;
        }

        let node_ref = node.as_ref().unwrap();
        RedBlack::_keys_in_order(&node_ref.left_child, keys);
        keys.push(node_ref.key());
        RedBlack::_keys_in_order(&node_ref.right_child, keys);
    }

    /// Returns all keys in the tree following a level-order traversal
    pub fn keys_in_level_order(&self) -> Vec<&K> {
        let mut keys: Vec<&K> = Vec::new();

        RedBlack::_keys_in_level_order(&self.root, &mut keys);

        keys
    }

    fn _keys_in_level_order<'a>(node: &'a Option<Box<Node<K, V>>>, keys: &mut Vec<&'a K>) {
        if node.is_none() {
            return;
        }

        let mut queue = VecDeque::<&Option<Box<Node<K, V>>>>::with_capacity(Node::size(node));
        queue.push_back(node);

        while !queue.is_empty() {
            let current_node = queue.pop_front().unwrap().as_ref().unwrap();
            keys.push(current_node.key());

            if !current_node.left_child.is_none() {
                queue.push_back(&current_node.left_child);
            }
            if !current_node.right_child.is_none() {
                queue.push_back(&current_node.right_child);
            }
        }
    }

    /// Returns all keys in the symbol table between `low_key`(inclusive) and `high_key`(exclusive)
    /// 
    /// # Arguments
    /// * `low_key`: lowest key of the range
    /// * `high_key`: highest key of the range
    /// 
    /// # Examples
    /// ```
    /// use rudac::tree::RedBlack;
    /// 
    /// let mut rb_tree = RedBlack::<usize, usize>::init();
    /// 
    /// for i in (1..100).rev() {
    ///     rb_tree.insert(i, i);
    /// }
    /// 
    /// let keys = rb_tree.keys_between(&1, &99);
    /// 
    /// assert_eq!(keys.len(), 98); 
    /// ```
    pub fn keys_between(&self, low_key: &K, high_key: &K) -> Vec<&K> {
        let mut keys: Vec<&K> = Vec::new();

        RedBlack::_keys_between(&self.root, low_key, high_key, &mut keys);

        keys
    }

    fn _keys_between<'a>(
        node: &'a Option<Box<Node<K, V>>>,
        low_key: &K,
        high_key: &K,
        keys: &mut Vec<&'a K>,
    ) {
        if node.is_none() {
            return;
        }

        let node_ref = node.as_ref().unwrap();
        if *low_key < *node_ref.key() {
            RedBlack::_keys_between(&node_ref.left_child, low_key, high_key, keys);
        }
        if *low_key <= *node_ref.key() && *node_ref.key() < *high_key {
            keys.push(node_ref.key());
        }
        if *high_key > *node_ref.key() {
            RedBlack::_keys_between(&node_ref.right_child, low_key, high_key, keys);
        }
    }

    /// Returns the number of keys in the tree between `low_key`(inclusive) and `high_key`(exclusive)
    /// 
    /// # Arguments
    /// * `low_key`: lowest key of the range
    /// * `high_key`: highest key of the range
    /// 
    /// # Examples
    /// ```
    /// use rudac::tree::RedBlack;
    /// 
    /// let mut rb_tree = RedBlack::<usize, usize>::init();
    /// 
    /// for i in (1..100).rev() {
    ///     rb_tree.insert(i, i);
    /// }
    /// 
    /// let keys = rb_tree.size_between(&1, &99);
    /// 
    /// assert_eq!(keys, 98); 
    /// ```
    pub fn size_between(&self, low_key: &K, high_key: &K) -> usize {
        if self.is_empty() {
            return 0;
        }
        if *low_key > *high_key {
            return 0;
        }

        return self.rank(high_key) - self.rank(low_key);
    }

    fn rotate_left(mut node: Box<Node<K, V>>) -> Box<Node<K, V>> {
        let mut y = node.right_child.unwrap();
        node.right_child = y.left_child;

        // update colors
        y.color = node.color;
        node.color = RED;

        // update y size
        y.size = node.size;

        // update node size
        node.update_size();

        // add node as left child of y
        y.left_child = Some(node);
        y
    }

    fn rotate_right(mut node: Box<Node<K, V>>) -> Box<Node<K, V>> {
        let mut y = node.left_child.unwrap();
        node.left_child = y.right_child;

        // update colors
        y.color = node.color;
        node.color = RED;

        // update y size
        y.size = node.size;

        // update node size
        node.update_size();

        // add node as right child of y
        y.right_child = Some(node);
        y
    }

    fn flip_colors(node: &mut Box<Node<K, V>>) {
        node.color = !node.color;
        // flip left child color
        let mut left_child = node.left_child.take().unwrap();
        left_child.color = !left_child.color;

        // flip right child color
        let mut right_child = node.right_child.take().unwrap();
        right_child.color = !right_child.color;

        node.left_child = Some(left_child);
        node.right_child = Some(right_child);
    }

    fn move_red_left(mut node: Box<Node<K, V>>) -> Box<Node<K, V>> {
        RedBlack::flip_colors(&mut node);

        if Node::is_red(&node.right_child().left_child) {
            let mut right_child = node.right_child.take().unwrap();
            right_child = RedBlack::rotate_right(right_child);

            node.right_child = Some(right_child);

            node = RedBlack::rotate_left(node);
            RedBlack::flip_colors(&mut node);
        }

        node
    }

    fn move_red_right(mut node: Box<Node<K, V>>) -> Box<Node<K, V>> {
        RedBlack::flip_colors(&mut node);

        if Node::is_red(&node.left_child().left_child) {
            node = RedBlack::rotate_right(node);
            RedBlack::flip_colors(&mut node);
        }

        node
    }

    fn balance(mut node: Box<Node<K, V>>) -> Box<Node<K, V>> {
        if Node::is_red(&node.right_child) {
            node = RedBlack::rotate_left(node);
        }
        if Node::is_red(&node.left_child) && Node::is_red(&node.left_child().left_child) {
            node = RedBlack::rotate_right(node);
        }
        if Node::is_red(&node.left_child) && Node::is_red(&node.right_child) {
            RedBlack::flip_colors(&mut node);
        }

        node.update_size();

        node
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_23<K: std::cmp::Ord, V>(node: &Option<Box<Node<K, V>>>, is_root: bool) -> bool {
        if node.is_none() {
            return true;
        }
        let node_ref = node.as_ref().unwrap();

        if Node::is_red(&node_ref.right_child) {
            return false;
        }
        if !is_root && Node::is_red(node) && Node::is_red(&node_ref.left_child) {
            return false;
        }

        return is_23(&node_ref.left_child, false) && is_23(&node_ref.right_child, false);
    }

    fn is_bst<K: std::cmp::Ord, V>(
        node: &Option<Box<Node<K, V>>>,
        min: Option<&K>,
        max: Option<&K>,
    ) -> bool {
        if node.is_none() {
            return true;
        }

        let node_ref = node.as_ref().unwrap();
        if !min.is_none() && *node_ref.key() <= **min.as_ref().unwrap() {
            return false;
        }
        if !max.is_none() && *node_ref.key() >= **max.as_ref().unwrap() {
            return false;
        }

        return is_bst(&node_ref.left_child, min, Some(node_ref.key()))
            && is_bst(&node_ref.right_child, Some(node_ref.key()), max);
    }

    fn is_size_consistent<K: std::cmp::Ord, V>(node: &Option<Box<Node<K, V>>>) -> bool {
        if node.is_none() {
            return true;
        }
        let node_ref = node.as_ref().unwrap();

        if Node::size(node)
            != Node::size(&node_ref.left_child) + Node::size(&node_ref.right_child) + 1
        {
            return false;
        }

        return is_size_consistent(&node_ref.left_child)
            && is_size_consistent(&node_ref.right_child);
    }

    fn is_rank_consistent<K: std::cmp::Ord, V>(rb_tree: &RedBlack<K, V>) -> bool {
        for i in 0..Node::size(&rb_tree.root) {
            if i != rb_tree.rank(rb_tree.select(i).unwrap().0) {
                return false;
            }
        }

        for key in rb_tree.keys() {
            if *rb_tree.select(rb_tree.rank(key)).unwrap().0 != *key {
                return false;
            }
        }

        true
    }

    #[test]
    fn tree_rb_init() {
        let rb_tree = RedBlack::<usize, usize>::init();

        assert!(is_23(&rb_tree.root, true));
        assert!(is_bst(&rb_tree.root, None, None));
        assert!(is_size_consistent(&rb_tree.root));
        assert!(is_rank_consistent(&rb_tree));
    }

    #[test]
    fn tree_rb_insert_1() {
        let mut rb_tree = RedBlack::<usize, usize>::init();

        rb_tree.insert(1, 1);

        assert!(is_23(&rb_tree.root, true));
        assert!(is_bst(&rb_tree.root, None, None));
        assert!(is_size_consistent(&rb_tree.root));
        assert!(is_rank_consistent(&rb_tree));
    }

    #[test]
    fn tree_rb_insert_2() {
        let mut rb_tree = RedBlack::<usize, usize>::init();

        rb_tree.insert(4, 1);
        rb_tree.insert(3, 1);
        rb_tree.insert(2, 1);
        rb_tree.insert(1, 1);

        assert!(is_23(&rb_tree.root, true));
        assert!(is_bst(&rb_tree.root, None, None));
        assert!(is_size_consistent(&rb_tree.root));
        assert!(is_rank_consistent(&rb_tree));
    }

    #[test]
    fn tree_rb_insert_3() {
        let mut rb_tree = RedBlack::<usize, usize>::init();

        for i in (0..100).rev() {
            rb_tree.insert(i, i);
        }

        assert!(is_23(&rb_tree.root, true));
        assert!(is_bst(&rb_tree.root, None, None));
        assert!(is_size_consistent(&rb_tree.root));
        assert!(is_rank_consistent(&rb_tree));
    }

    #[test]
    fn tree_rb_delete_1() {
        let mut rb_tree = RedBlack::<usize, usize>::init();

        rb_tree.delete(&1);

        assert!(is_23(&rb_tree.root, true));
        assert!(is_bst(&rb_tree.root, None, None));
        assert!(is_size_consistent(&rb_tree.root));
        assert!(is_rank_consistent(&rb_tree));
    }

    #[test]
    fn tree_rb_delete_2() {
        let mut rb_tree = RedBlack::<usize, usize>::init();

        rb_tree.insert(4, 1);
        rb_tree.insert(3, 1);
        rb_tree.insert(2, 1);
        rb_tree.insert(1, 1);

        rb_tree.delete(&1);
        assert_eq!(rb_tree.get(&1), None);

        rb_tree.delete(&3);
        assert_eq!(rb_tree.get(&3), None);

        rb_tree.delete(&2);
        assert_eq!(rb_tree.get(&2), None);

        rb_tree.delete(&4);
        assert_eq!(rb_tree.get(&4), None);

        assert!(is_23(&rb_tree.root, true));
        assert!(is_bst(&rb_tree.root, None, None));
        assert!(is_size_consistent(&rb_tree.root));
        assert!(is_rank_consistent(&rb_tree));
    }

    #[test]
    fn tree_rb_is_empty() {
        let mut rb_tree = RedBlack::<usize, usize>::init();

        assert!(rb_tree.is_empty());

        rb_tree.insert(1, 1);

        assert!(!rb_tree.is_empty());
    }

    #[test]
    fn tree_rb_size_1() {
        let mut rb_tree = RedBlack::<usize, usize>::init();
        assert_eq!(rb_tree.size(), 0);

        rb_tree.insert(1, 1);
        assert_eq!(rb_tree.size(), 1);

        rb_tree.insert(2, 1);
        assert_eq!(rb_tree.size(), 2);

        rb_tree.insert(3, 1);
        assert_eq!(rb_tree.size(), 3);

        rb_tree.insert(4, 1);
        assert_eq!(rb_tree.size(), 4);
    }

    #[test]
    fn tree_rb_contains_1() {
        let mut rb_tree = RedBlack::<usize, usize>::init();
        assert!(!rb_tree.contains(&1));

        rb_tree.insert(1, 2);
        assert!(rb_tree.contains(&1));

        rb_tree.insert(1, 3);
        assert!(rb_tree.contains(&1));
    }

    #[test]
    fn tree_rb_get_1() {
        let mut rb_tree = RedBlack::<usize, usize>::init();
        assert_eq!(rb_tree.get(&1), None);

        rb_tree.insert(1, 2);
        assert_eq!(*rb_tree.get(&1).unwrap(), 2);

        assert!(is_23(&rb_tree.root, true));
        assert!(is_bst(&rb_tree.root, None, None));
        assert!(is_size_consistent(&rb_tree.root));
        assert!(is_rank_consistent(&rb_tree));
    }

    #[test]
    fn tree_rb_get_2() {
        let mut rb_tree = RedBlack::<usize, usize>::init();

        for i in (0..100).rev() {
            rb_tree.insert(i, i);
        }

        for i in (0..100).rev() {
            assert_eq!(*rb_tree.get(&i).unwrap(), i);
        }

        assert!(is_23(&rb_tree.root, true));
        assert!(is_bst(&rb_tree.root, None, None));
        assert!(is_size_consistent(&rb_tree.root));
        assert!(is_rank_consistent(&rb_tree));
    }

    #[test]
    fn tree_rb_get_3() {
        let mut rb_tree = RedBlack::<usize, usize>::init();

        for i in (0..100).rev() {
            rb_tree.insert(i, i);
        }

        for i in (0..100).rev() {
            assert_eq!(*rb_tree.get(&i).unwrap(), i);
        }

        for i in (0..100).rev() {
            rb_tree.insert(i, i + 1);
        }

        for i in (0..100).rev() {
            assert_eq!(*rb_tree.get(&i).unwrap(), i + 1);
        }

        assert!(is_23(&rb_tree.root, true));
        assert!(is_bst(&rb_tree.root, None, None));
        assert!(is_size_consistent(&rb_tree.root));
        assert!(is_rank_consistent(&rb_tree));
    }

    #[test]
    fn tree_rb_delete_min_1() {
        let mut rb_tree = RedBlack::<usize, usize>::init();

        rb_tree.delete_min();

        assert!(is_23(&rb_tree.root, true));
        assert!(is_bst(&rb_tree.root, None, None));
        assert!(is_size_consistent(&rb_tree.root));
        assert!(is_rank_consistent(&rb_tree));
    }

    #[test]
    fn tree_rb_delete_min_2() {
        let mut rb_tree = RedBlack::<usize, usize>::init();

        for i in (0..100).rev() {
            rb_tree.insert(i, i);
        }

        for i in 0..100 {
            rb_tree.delete_min();
            assert_eq!(rb_tree.get(&i), None);
        }

        assert!(is_23(&rb_tree.root, true));
        assert!(is_bst(&rb_tree.root, None, None));
        assert!(is_size_consistent(&rb_tree.root));
        assert!(is_rank_consistent(&rb_tree));
    }

    #[test]
    fn tree_rb_delete_max_1() {
        let mut rb_tree = RedBlack::<usize, usize>::init();

        rb_tree.delete_max();

        assert!(is_23(&rb_tree.root, true));
        assert!(is_bst(&rb_tree.root, None, None));
        assert!(is_size_consistent(&rb_tree.root));
        assert!(is_rank_consistent(&rb_tree));
    }

    #[test]
    fn tree_rb_delete_max_2() {
        let mut rb_tree = RedBlack::<usize, usize>::init();

        for i in (0..100).rev() {
            rb_tree.insert(i, i);
        }

        for i in (0..100).rev() {
            rb_tree.delete_max();
            assert_eq!(rb_tree.get(&i), None);
        }

        assert!(is_23(&rb_tree.root, true));
        assert!(is_bst(&rb_tree.root, None, None));
        assert!(is_size_consistent(&rb_tree.root));
        assert!(is_rank_consistent(&rb_tree));
    }

    #[test]
    fn tree_rb_floor_1() {
        let mut rb_tree = RedBlack::<usize, usize>::init();

        for i in (0..100).step_by(2) {
            rb_tree.insert(i, i);
        }

        for i in (1..100).step_by(2) {
            assert_eq!(*rb_tree.floor(&i).unwrap(), i - 1);
        }

        assert!(is_23(&rb_tree.root, true));
        assert!(is_bst(&rb_tree.root, None, None));
        assert!(is_size_consistent(&rb_tree.root));
        assert!(is_rank_consistent(&rb_tree));
    }

    #[test]
    fn tree_rb_ceiling_1() {
        let mut rb_tree = RedBlack::<usize, usize>::init();

        for i in (0..100).step_by(2) {
            rb_tree.insert(i, i);
        }

        for i in (1..99).step_by(2) {
            assert_eq!(*rb_tree.ceiling(&i).unwrap(), i + 1);
        }

        assert!(is_23(&rb_tree.root, true));
        assert!(is_bst(&rb_tree.root, None, None));
        assert!(is_size_consistent(&rb_tree.root));
        assert!(is_rank_consistent(&rb_tree));
    }

    #[test]
    fn tree_rb_select_1() {
        let mut rb_tree = RedBlack::<usize, usize>::init();

        for i in (0..100).rev() {
            rb_tree.insert(i, i);
        }

        for i in 0..100 {
            let result = rb_tree.select(i).unwrap();
            assert_eq!((*result.0, *result.1), (i, i));
        }

        assert!(is_23(&rb_tree.root, true));
        assert!(is_bst(&rb_tree.root, None, None));
        assert!(is_size_consistent(&rb_tree.root));
        assert!(is_rank_consistent(&rb_tree));
    }

    #[test]
    fn tree_rb_rank_1() {
        let mut rb_tree = RedBlack::<usize, usize>::init();

        for i in (1..100).rev() {
            rb_tree.insert(i, i);
        }

        for i in 1..100 {
            assert_eq!(rb_tree.rank(&i), i - 1);
        }

        assert!(is_23(&rb_tree.root, true));
        assert!(is_bst(&rb_tree.root, None, None));
        assert!(is_size_consistent(&rb_tree.root));
        assert!(is_rank_consistent(&rb_tree));
    }

    #[test]
    fn tree_rb_keys_1() {
        let mut rb_tree = RedBlack::<usize, usize>::init();

        for i in (1..100).rev() {
            rb_tree.insert(i, i);
        }

        let mut i = 1;
        for key in rb_tree.keys() {
            assert!(*key == i);
            i += 1;
        }

        assert!(is_23(&rb_tree.root, true));
        assert!(is_bst(&rb_tree.root, None, None));
        assert!(is_size_consistent(&rb_tree.root));
        assert!(is_rank_consistent(&rb_tree));
    }

    #[test]
    fn tree_rb_keys_between_1() {
        let mut rb_tree = RedBlack::<usize, usize>::init();

        for i in (1..100).rev() {
            rb_tree.insert(i, i);
        }

        for i in 1..100 {
            assert_eq!(rb_tree.keys_between(&i, &99).len(), 99 - i);
        }

        assert!(is_23(&rb_tree.root, true));
        assert!(is_bst(&rb_tree.root, None, None));
        assert!(is_size_consistent(&rb_tree.root));
        assert!(is_rank_consistent(&rb_tree));
    }

    #[test]
    fn tree_rb_size_between_1() {
        let mut rb_tree = RedBlack::<usize, usize>::init();

        for i in (1..100).rev() {
            rb_tree.insert(i, i);
        }

        for i in 1..100 {
            assert_eq!(rb_tree.size_between(&i, &100), 100 - i);
        }

        assert!(is_23(&rb_tree.root, true));
        assert!(is_bst(&rb_tree.root, None, None));
        assert!(is_size_consistent(&rb_tree.root));
        assert!(is_rank_consistent(&rb_tree));
    }
}
