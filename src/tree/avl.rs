use std::collections::VecDeque;

struct Node<K: std::cmp::Ord, V> {
    key: Option<K>,
    value: Option<V>,
    height: usize,
    size: usize,
    left_child: Option<Box<Node<K, V>>>,
    right_child: Option<Box<Node<K, V>>>,
}

impl<K: std::cmp::Ord, V> Node<K, V> {
    fn init(key: K, value: V, height: usize, size: usize) -> Node<K, V> {
        Node {
            key: Some(key),
            value: Some(value),
            height: height,
            size: size,
            left_child: None,
            right_child: None,
        }
    }

    fn key(&self) -> &K {
        &self.key.as_ref().unwrap()
    }

    fn get_key(&mut self) -> K {
        self.key.take().unwrap()
    }

    fn value(&self) -> &V {
        &self.value.as_ref().unwrap()
    }

    fn get_value(&mut self) -> V {
        self.value.take().unwrap()
    }

    fn update_height(&mut self) {
        self.height = (1 + Node::_max_height(&self.left_child, &self.right_child)) as usize;
    }

    fn update_size(&mut self) {
        self.size = 1 + Node::size(&self.left_child) + Node::size(&self.right_child);
    }

    fn _max_height(node1: &Option<Box<Node<K, V>>>, node2: &Option<Box<Node<K, V>>>) -> i64 {
        std::cmp::max(Node::height(node1), Node::height(node2))
    }

    fn height(node: &Option<Box<Node<K, V>>>) -> i64 {
        match node {
            Some(_node) => _node.height as i64,
            None => -1,
        }
    }

    fn size(node: &Option<Box<Node<K, V>>>) -> usize {
        match node {
            Some(_node) => _node.size,
            None => 0,
        }
    }

    fn balance_factor(node: &Box<Node<K, V>>) -> i64 {
        Node::height(&node.left_child) - Node::height(&node.right_child)
    }
}


pub struct AVL<K: std::cmp::Ord, V> {
    root: Option<Box<Node<K, V>>>,
}

impl<K: std::cmp::Ord, V> AVL<K, V> {
    pub fn init() -> AVL<K, V> {
        AVL { root: None }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn size(&self) -> usize {
        Node::size(&self.root)
    }

    pub fn contains(&self, key: &K) -> bool {
        !self.get(key).is_none()
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        AVL::_get(&self.root, key)
    }

    fn _get<'a>(node: &'a Option<Box<Node<K, V>>>, key: &K) -> Option<&'a V> {
        if node.is_none() {
            return None;
        }

        let node_ref = node.as_ref().unwrap();

        if *key < *node_ref.key() {
            AVL::_get(&node_ref.left_child, key)
        } else if *key > *node_ref.key() {
            AVL::_get(&node_ref.right_child, key)
        } else {
            return Some(node_ref.value());
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.root = AVL::_insert(self.root.take(), key, value);
    }

    fn _insert(node: Option<Box<Node<K, V>>>, key: K, value: V) -> Option<Box<Node<K, V>>> {
        if node.is_none() {
            return Some(Box::new(Node::init(key, value, 0, 1)));
        }

        let mut node_ref = node.unwrap();

        if key < *node_ref.key() {
            node_ref.left_child = AVL::_insert(node_ref.left_child, key, value);
        } else if key > *node_ref.key() {
            node_ref.right_child = AVL::_insert(node_ref.right_child, key, value);
        } else {
            node_ref.value = Some(value);
            return Some(node_ref);
        }

        node_ref.update_height();
        node_ref.update_size();

        Some(AVL::balance(node_ref))
    }

    fn balance(mut node: Box<Node<K, V>>) -> Box<Node<K, V>> {
        if Node::balance_factor(&node) < -1 {
            if Node::balance_factor(node.right_child.as_ref().unwrap()) > 0 {
                node.right_child = Some(AVL::rotate_right(node.right_child.unwrap()));
            }
            node = AVL::rotate_left(node);
        } else if Node::balance_factor(&node) > 1 {
            if Node::balance_factor(node.left_child.as_ref().unwrap()) < 0 {
                node.left_child = Some(AVL::rotate_left(node.left_child.unwrap()));
            }
            node = AVL::rotate_right(node);
        }
        node
    }

    fn rotate_right(mut node: Box<Node<K, V>>) -> Box<Node<K, V>> {
        let mut y = node.left_child.unwrap();
        node.left_child = y.right_child;
        y.size = node.size;
        node.update_height();
        node.update_size();

        y.right_child = Some(node);
        y.update_height();

        y
    }

    fn rotate_left(mut node: Box<Node<K, V>>) -> Box<Node<K, V>> {
        let mut y = node.right_child.unwrap();
        node.right_child = y.left_child;
        y.size = node.size;

        node.update_height();
        node.update_size();

        y.left_child = Some(node);
        y.update_height();

        y
    }

    pub fn delete(&mut self, key: &K) {
        if !self.is_empty() {
            self.root = AVL::_delete(self.root.take(), key);
        }
    }

    fn _delete(node: Option<Box<Node<K, V>>>, key: &K) -> Option<Box<Node<K, V>>> {
        match node {
            None => node,
            Some(mut _node) => {
                if *key < *_node.key() {
                    _node.left_child = AVL::_delete(_node.left_child.take(), key);
                } else if *key > *_node.key() {
                    _node.right_child = AVL::_delete(_node.right_child.take(), key);
                } else {
                    if _node.left_child.is_none() {
                        return _node.right_child;
                    } else if _node.right_child.is_none() {
                        return _node.left_child;
                    } else {
                        let mut y = _node;
                        _node = AVL::min(&mut y.right_child);
                        _node.right_child = AVL::_delete_min(y.right_child.unwrap());
                        _node.left_child = y.left_child;
                    }
                }

                _node.update_height();
                _node.update_size();
                Some(AVL::balance(_node))
            }
        }
    }

    fn min(node: &mut Option<Box<Node<K, V>>>) -> Box<Node<K, V>> {
        match node {
            Some(_node) => {
                if _node.left_child.is_none() {
                    Box::new(Node::init(_node.get_key(), _node.get_value(), 0, 1))
                } else {
                    AVL::min(&mut _node.left_child)
                }
            }
            None => panic!("Called min on None node"),
        }
    }

    pub fn delete_min(&mut self) {
        if !self.is_empty() {
            self.root = AVL::_delete_min(self.root.take().unwrap());
        }
    }

    fn _delete_min(mut node: Box<Node<K, V>>) -> Option<Box<Node<K, V>>> {
        if node.left_child.is_none() {
            return node.right_child.take();
        }

        node.left_child = AVL::_delete_min(node.left_child.unwrap());

        node.update_height();
        node.update_size();

        Some(AVL::balance(node))
    }

    pub fn delete_max(&mut self) {
        if !self.is_empty() {
            self.root = AVL::_delete_max(self.root.take().unwrap());
        }
    }

    fn _delete_max(mut node: Box<Node<K, V>>) -> Option<Box<Node<K, V>>> {
        if node.right_child.is_none() {
            return node.left_child.take();
        }

        node.right_child = AVL::_delete_max(node.right_child.unwrap());

        node.update_height();
        node.update_size();

        Some(AVL::balance(node))
    }

    pub fn floor(&self, key: &K) -> Option<&K> {
        AVL::_floor(&self.root, key)
    }

    fn _floor<'a>(node: &'a Option<Box<Node<K, V>>>, key: &K) -> Option<&'a K> {
        if node.is_none() {
            return None;
        }
        let node_ref = node.as_ref().unwrap();
        if *key == *node_ref.key() {
            return Some(node_ref.key());
        } else if *key < *node_ref.key() {
            return AVL::_floor(&node_ref.left_child, key);
        }
        let found_key = AVL::_floor(&node_ref.right_child, key);
        if !found_key.is_none() {
            return found_key;
        } else {
            return Some(node_ref.key());
        }
    }

    pub fn ceiling(&self, key: &K) -> Option<&K> {
        AVL::_ceiling(&self.root, key)
    }

    fn _ceiling<'a>(node: &'a Option<Box<Node<K, V>>>, key: &K) -> Option<&'a K> {
        if node.is_none() {
            return None;
        }
        let node_ref = node.as_ref().unwrap();
        if *key == *node_ref.key() {
            return Some(node_ref.key());
        } else if *key > *node_ref.key() {
            return AVL::_ceiling(&node_ref.right_child, key);
        }
        let found_key = AVL::_ceiling(&node_ref.left_child, key);
        if !found_key.is_none() {
            return found_key;
        } else {
            return Some(node_ref.key());
        }
    }

    pub fn select(&self, k: usize) -> Option<(&K, &V)> {
        if k > self.size() {
            panic!("K must be in range 0 <= k <= size - 1");
        }
        AVL::_select(&self.root, k)
    }

    fn _select(node: &Option<Box<Node<K, V>>>, k: usize) -> Option<(&K, &V)> {
        if node.is_none() {
            return None;
        }
        let node_ref = node.as_ref().unwrap();

        let t = Node::size(&node_ref.left_child);
        if t > k {
            return AVL::_select(&node_ref.left_child, k);
        } else if t < k {
            return AVL::_select(&node_ref.right_child, k - t - 1);
        } else {
            return Some((node_ref.key(), node_ref.value()));
        }
    }

    pub fn rank(&self, key: &K) -> usize {
        AVL::_rank(&self.root, key)
    }
    fn _rank(node: &Option<Box<Node<K, V>>>, key: &K) -> usize {
        if node.is_none() {
            return 0;
        }
        let node_ref = node.as_ref().unwrap();
        if *key < *node_ref.key() {
            AVL::_rank(&node_ref.left_child, key)
        } else if *key > *node_ref.key() {
            1 + Node::size(&node_ref.left_child) + AVL::_rank(&node_ref.right_child, key)
        } else {
            Node::size(&node_ref.left_child)
        }
    }

    pub fn keys(&self) -> Vec<&K> {
        let mut keys: Vec<&K> = Vec::new();

        AVL::_keys_in_order(&self.root, &mut keys);

        keys
    }

    fn _keys_in_order<'a>(node: &'a Option<Box<Node<K, V>>>, keys: &mut Vec<&'a K>) {
        if node.is_none() {
            return;
        }

        let node_ref = node.as_ref().unwrap();
        AVL::_keys_in_order(&node_ref.left_child, keys);
        keys.push(node_ref.key());
        AVL::_keys_in_order(&node_ref.right_child, keys);
    }

    pub fn keys_in_level_order(&self) -> Vec<&K> {
        let mut keys: Vec<&K> = Vec::new();

        AVL::_keys_in_level_order(&self.root, &mut keys);

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

    pub fn keys_between(&self, low_key: &K, high_key: &K) -> Vec<&K> {
        let mut keys: Vec<&K> = Vec::new();

        AVL::_keys_between(&self.root, low_key, high_key, &mut keys);

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
            AVL::_keys_between(&node_ref.left_child, low_key, high_key, keys);
        }
        if *low_key <= *node_ref.key() && *node_ref.key() <= *high_key {
            keys.push(node_ref.key());
        }
        if *high_key > *node_ref.key() {
            AVL::_keys_between(&node_ref.right_child, low_key, high_key, keys);
        }
    }

    pub fn size_between(&self, low_key: &K, high_key: &K) -> usize {
        if self.is_empty() {
            return 0;
        }
        if *low_key > *high_key {
            return 0;
        }

        return self.rank(high_key) - self.rank(low_key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_avl<K: std::cmp::Ord, V>(node: &Option<Box<Node<K, V>>>) -> bool {
        if node.is_none() {
            return true;
        }

        let node_ref = node.as_ref().unwrap();
        let balance_factor = Node::balance_factor(node_ref);

        if balance_factor < -1 || balance_factor > 1 {
            return false;
        }

        return is_avl(&node_ref.left_child) && is_avl(&node_ref.right_child);
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

    fn is_rank_consistent<K: std::cmp::Ord, V>(avl_tree: &AVL<K, V>) -> bool {
        for i in 0..Node::size(&avl_tree.root) {
            if i != avl_tree.rank(avl_tree.select(i).unwrap().0) {
                return false;
            }
        }

        for key in avl_tree.keys() {
            if *avl_tree.select(avl_tree.rank(key)).unwrap().0 != *key {
                return false;
            }
        }

        true
    }

    #[test]
    fn tree_avl_node_max_height() {
        assert_eq!(Node::<usize, usize>::_max_height(&None, &None), -1);
        assert_eq!(
            Node::<usize, usize>::_max_height(&Some(Box::new(Node::init(1, 1, 0, 0))), &None),
            0
        );
        assert_eq!(
            Node::<usize, usize>::_max_height(
                &Some(Box::new(Node::init(1, 1, 1, 0))),
                &Some(Box::new(Node::init(1, 1, 2, 0)))
            ),
            2
        );
    }

    #[test]
    fn tree_avl_node_update_height() {
        let mut root = Node::init(1, 1, 10, 0);
        let mut left = Node::init(1, 1, 20, 0);
        let mut right = Node::init(1, 1, 30, 0);

        left.update_height();
        right.update_height();

        root.left_child = Some(Box::new(left));
        root.right_child = Some(Box::new(right));

        root.update_height();

        assert_eq!(root.height, 1);
        assert_eq!(root.left_child.unwrap().height, 0);
        assert_eq!(root.right_child.unwrap().height, 0);
    }

    #[test]
    fn tree_avl_init() {
        let avl_tree = AVL::<usize, usize>::init();

        assert!(is_avl(&avl_tree.root));
        assert!(is_bst(&avl_tree.root, None, None));
        assert!(is_size_consistent(&avl_tree.root));
        assert!(is_rank_consistent(&avl_tree));
    }

    #[test]
    fn tree_avl_insert_1() {
        let mut avl_tree = AVL::<usize, usize>::init();

        avl_tree.insert(1, 1);

        assert!(is_avl(&avl_tree.root));
        assert!(is_bst(&avl_tree.root, None, None));
        assert!(is_size_consistent(&avl_tree.root));
        assert!(is_rank_consistent(&avl_tree));
    }

    #[test]
    fn tree_avl_insert_2() {
        let mut avl_tree = AVL::<usize, usize>::init();

        avl_tree.insert(4, 1);
        avl_tree.insert(3, 1);
        avl_tree.insert(2, 1);
        avl_tree.insert(1, 1);

        assert!(is_avl(&avl_tree.root));
        assert!(is_bst(&avl_tree.root, None, None));
        assert!(is_size_consistent(&avl_tree.root));
        assert!(is_rank_consistent(&avl_tree));
    }

    #[test]
    fn tree_avl_insert_3() {
        let mut avl_tree = AVL::<usize, usize>::init();

        for i in (0..100).rev() {
            avl_tree.insert(i, i);
        }

        assert!(is_avl(&avl_tree.root));
        assert!(is_bst(&avl_tree.root, None, None));
        assert!(is_size_consistent(&avl_tree.root));
        assert!(is_rank_consistent(&avl_tree));
    }

    #[test]
    fn tree_avl_delete_1() {
        let mut avl_tree = AVL::<usize, usize>::init();

        avl_tree.delete(&1);

        assert!(is_avl(&avl_tree.root));
        assert!(is_bst(&avl_tree.root, None, None));
        assert!(is_size_consistent(&avl_tree.root));
        assert!(is_rank_consistent(&avl_tree));
    }

    #[test]
    fn tree_avl_delete_2() {
        let mut avl_tree = AVL::<usize, usize>::init();

        avl_tree.insert(4, 1);
        avl_tree.insert(3, 1);
        avl_tree.insert(2, 1);
        avl_tree.insert(1, 1);

        avl_tree.delete(&1);
        assert_eq!(avl_tree.get(&1), None);

        avl_tree.delete(&3);
        assert_eq!(avl_tree.get(&3), None);

        avl_tree.delete(&2);
        assert_eq!(avl_tree.get(&2), None);

        avl_tree.delete(&4);
        assert_eq!(avl_tree.get(&4), None);

        assert!(is_avl(&avl_tree.root));
        assert!(is_bst(&avl_tree.root, None, None));
        assert!(is_size_consistent(&avl_tree.root));
        assert!(is_rank_consistent(&avl_tree));
    }

    #[test]
    fn tree_avl_is_empty() {
        let mut avl_tree = AVL::<usize, usize>::init();

        assert!(avl_tree.is_empty());

        avl_tree.insert(1, 1);

        assert!(!avl_tree.is_empty());
    }

    #[test]
    fn tree_avl_size_1() {
        let mut avl_tree = AVL::<usize, usize>::init();
        assert_eq!(avl_tree.size(), 0);

        avl_tree.insert(1, 1);
        assert_eq!(avl_tree.size(), 1);

        avl_tree.insert(2, 1);
        assert_eq!(avl_tree.size(), 2);

        avl_tree.insert(3, 1);
        assert_eq!(avl_tree.size(), 3);

        avl_tree.insert(4, 1);
        assert_eq!(avl_tree.size(), 4);
    }

    #[test]
    fn tree_avl_contains_1() {
        let mut avl_tree = AVL::<usize, usize>::init();
        assert!(!avl_tree.contains(&1));

        avl_tree.insert(1, 2);
        assert!(avl_tree.contains(&1));

        avl_tree.insert(1, 3);
        assert!(avl_tree.contains(&1));
    }

    #[test]
    fn tree_avl_get_1() {
        let mut avl_tree = AVL::<usize, usize>::init();
        assert_eq!(avl_tree.get(&1), None);

        avl_tree.insert(1, 2);
        assert_eq!(*avl_tree.get(&1).unwrap(), 2);

        assert!(is_avl(&avl_tree.root));
        assert!(is_bst(&avl_tree.root, None, None));
        assert!(is_size_consistent(&avl_tree.root));
        assert!(is_rank_consistent(&avl_tree));
    }

    #[test]
    fn tree_avl_get_2() {
        let mut avl_tree = AVL::<usize, usize>::init();

        for i in (0..100).rev() {
            avl_tree.insert(i, i);
        }

        for i in (0..100).rev() {
            assert_eq!(*avl_tree.get(&i).unwrap(), i);
        }

        assert!(is_avl(&avl_tree.root));
        assert!(is_bst(&avl_tree.root, None, None));
        assert!(is_size_consistent(&avl_tree.root));
        assert!(is_rank_consistent(&avl_tree));
    }

    #[test]
    fn tree_avl_get_3() {
        let mut avl_tree = AVL::<usize, usize>::init();

        for i in (0..100).rev() {
            avl_tree.insert(i, i);
        }

        for i in (0..100).rev() {
            assert_eq!(*avl_tree.get(&i).unwrap(), i);
        }

        for i in (0..100).rev() {
            avl_tree.insert(i, i + 1);
        }

        for i in (0..100).rev() {
            assert_eq!(*avl_tree.get(&i).unwrap(), i + 1);
        }

        assert!(is_avl(&avl_tree.root));
        assert!(is_bst(&avl_tree.root, None, None));
        assert!(is_size_consistent(&avl_tree.root));
        assert!(is_rank_consistent(&avl_tree));
    }

    #[test]
    fn tree_avl_delete_min_1() {
        let mut avl_tree = AVL::<usize, usize>::init();

        avl_tree.delete_min();

        assert!(is_avl(&avl_tree.root));
        assert!(is_bst(&avl_tree.root, None, None));
        assert!(is_size_consistent(&avl_tree.root));
        assert!(is_rank_consistent(&avl_tree));
    }

    #[test]
    fn tree_avl_delete_min_2() {
        let mut avl_tree = AVL::<usize, usize>::init();

        for i in (0..100).rev() {
            avl_tree.insert(i, i);
        }

        for i in 0..100 {
            avl_tree.delete_min();
            assert_eq!(avl_tree.get(&i), None);
        }

        assert!(is_avl(&avl_tree.root));
        assert!(is_bst(&avl_tree.root, None, None));
        assert!(is_size_consistent(&avl_tree.root));
        assert!(is_rank_consistent(&avl_tree));
    }

    #[test]
    fn tree_avl_delete_max_1() {
        let mut avl_tree = AVL::<usize, usize>::init();

        avl_tree.delete_max();

        assert!(is_avl(&avl_tree.root));
        assert!(is_bst(&avl_tree.root, None, None));
        assert!(is_size_consistent(&avl_tree.root));
        assert!(is_rank_consistent(&avl_tree));
    }

    #[test]
    fn tree_avl_delete_max_2() {
        let mut avl_tree = AVL::<usize, usize>::init();

        for i in (0..100).rev() {
            avl_tree.insert(i, i);
        }

        for i in (0..100).rev() {
            avl_tree.delete_max();
            assert_eq!(avl_tree.get(&i), None);
        }

        assert!(is_avl(&avl_tree.root));
        assert!(is_bst(&avl_tree.root, None, None));
        assert!(is_size_consistent(&avl_tree.root));
        assert!(is_rank_consistent(&avl_tree));
    }

    #[test]
    fn tree_avl_floor_1() {
        let mut avl_tree = AVL::<usize, usize>::init();

        for i in (0..100).step_by(2) {
            avl_tree.insert(i, i);
        }

        for i in (1..100).step_by(2) {
            assert_eq!(*avl_tree.floor(&i).unwrap(), i - 1);
        }

        assert!(is_avl(&avl_tree.root));
        assert!(is_bst(&avl_tree.root, None, None));
        assert!(is_size_consistent(&avl_tree.root));
        assert!(is_rank_consistent(&avl_tree));
    }

    #[test]
    fn tree_avl_ceiling_1() {
        let mut avl_tree = AVL::<usize, usize>::init();

        for i in (0..100).step_by(2) {
            avl_tree.insert(i, i);
        }

        for i in (1..99).step_by(2) {
            assert_eq!(*avl_tree.ceiling(&i).unwrap(), i + 1);
        }

        assert!(is_avl(&avl_tree.root));
        assert!(is_bst(&avl_tree.root, None, None));
        assert!(is_size_consistent(&avl_tree.root));
        assert!(is_rank_consistent(&avl_tree));
    }

    #[test]
    fn tree_avl_select_1() {
        let mut avl_tree = AVL::<usize, usize>::init();

        for i in (0..100).rev() {
            avl_tree.insert(i, i);
        }

        for i in 1..100 {
            let result = avl_tree.select(i).unwrap();
            assert_eq!((*result.0, *result.1), (i, i));
        }

        assert!(is_avl(&avl_tree.root));
        assert!(is_bst(&avl_tree.root, None, None));
        assert!(is_size_consistent(&avl_tree.root));
        assert!(is_rank_consistent(&avl_tree));
    }

    #[test]
    fn tree_avl_rank_1() {
        let mut avl_tree = AVL::<usize, usize>::init();

        for i in (1..100).rev() {
            avl_tree.insert(i, i);
        }

        for i in 1..100 {
            assert_eq!(avl_tree.rank(&i), i - 1);
        }

        assert!(is_avl(&avl_tree.root));
        assert!(is_bst(&avl_tree.root, None, None));
        assert!(is_size_consistent(&avl_tree.root));
        assert!(is_rank_consistent(&avl_tree));
    }

    #[test]
    fn tree_avl_keys_1() {
        let mut avl_tree = AVL::<usize, usize>::init();

        for i in (1..100).rev() {
            avl_tree.insert(i, i);
        }

        let mut i = 1;
        for key in avl_tree.keys() {
            assert!(*key == i);
            i += 1;
        }

        assert!(is_avl(&avl_tree.root));
        assert!(is_bst(&avl_tree.root, None, None));
        assert!(is_size_consistent(&avl_tree.root));
        assert!(is_rank_consistent(&avl_tree));
    }

    #[test]
    fn tree_avl_keys_between_1() {
        let mut avl_tree = AVL::<usize, usize>::init();

        for i in (1..100).rev() {
            avl_tree.insert(i, i);
        }

        for i in 1..100 {
            let mut j = i;

            for key in avl_tree.keys_between(&i, &99) {
                assert_eq!(*key, j);
                j += 1;
            }
        }

        assert!(is_avl(&avl_tree.root));
        assert!(is_bst(&avl_tree.root, None, None));
        assert!(is_size_consistent(&avl_tree.root));
        assert!(is_rank_consistent(&avl_tree));
    }

    #[test]
    fn tree_avl_size_between_1() {
        let mut avl_tree = AVL::<usize, usize>::init();

        for i in (1..100).rev() {
            avl_tree.insert(i, i);
        }

        for i in 1..100 {
            assert_eq!(avl_tree.size_between(&i, &100), 100 - i);
        }

        assert!(is_avl(&avl_tree.root));
        assert!(is_bst(&avl_tree.root, None, None));
        assert!(is_size_consistent(&avl_tree.root));
        assert!(is_rank_consistent(&avl_tree));
    }
}
