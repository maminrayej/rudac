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

    fn get_key(&mut self) -> K {
        self.key.take().unwrap()
    }

    fn value(&self) -> &V {
        &self.value.as_ref().unwrap()
    }

    fn get_value(&mut self) -> V {
        self.value.take().unwrap()
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

pub struct RedBlack<K: std::cmp::Ord, V> {
    root: Option<Box<Node<K, V>>>,
}

impl<K: std::cmp::Ord, V> RedBlack<K, V> {
    pub fn size(&self) -> usize {
        Node::size(&self.root)
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

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

    pub fn contains(&self, key: &K) -> bool {
        !self.get(key).is_none()
    }

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
        if Node::is_red(&node_ref.left_child)
            && Node::is_red(&node_ref.left_child.as_ref().unwrap().left_child)
        {
            node_ref = RedBlack::rotate_right(node_ref);
        }
        if Node::is_red(&node_ref.left_child) && Node::is_red(&node_ref.right_child) {
            RedBlack::flip_colors(&mut node_ref);
        }

        node_ref.update_size();

        Some(node_ref)
    }

    pub fn delete(&mut self, key: &K) {

        
    }

    fn _delete(node: Option<Box<Node<K,V>>>, key: &K) -> Option<Box<Node<K,V>>> {
        None
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

        if Node::is_red(&node.right_child.as_ref().unwrap().left_child) {
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

        if Node::is_red(&node.left_child.as_ref().unwrap().left_child) {
            node = RedBlack::rotate_right(node);
            RedBlack::flip_colors(&mut node);
        }

        node
    }

    fn balance(mut node: Box<Node<K, V>>) -> Box<Node<K, V>> {
        if Node::is_red(&node.right_child) {
            node = RedBlack::rotate_left(node);
        }
        if Node::is_red(&node.left_child)
            && Node::is_red(&node.left_child.as_ref().unwrap().left_child)
        {
            node = RedBlack::rotate_right(node);
        }
        if Node::is_red(&node.left_child) && Node::is_red(&node.right_child) {
            RedBlack::flip_colors(&mut node);
        }

        node.update_size();

        node
    }
}
