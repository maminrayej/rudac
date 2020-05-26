use crate::util::Interval;
use std::cmp::Ord;
use std::ops::Bound;
use std::ops::Bound::*;
use std::rc::Rc;

struct Node<T: Ord> {
    interval: Option<Interval<T>>,
    max: Option<Rc<Bound<T>>>,
    height: usize,
    size: usize,
    left_child: Option<Box<Node<T>>>,
    right_child: Option<Box<Node<T>>>,
}

impl<T: Ord> Node<T> {
    fn init(interval: Interval<T>, max: Rc<Bound<T>>, height: usize, size: usize) -> Node<T> {
        Node {
            interval: Some(interval),
            max: Some(max),
            height: height,
            size: size,
            left_child: None,
            right_child: None,
        }
    }

    fn interval(&self) -> &Interval<T> {
        &self.interval.as_ref().unwrap()
    }

    fn get_interval(&mut self) -> Interval<T> {
        self.interval.take().unwrap()
    }

    fn get_max(&self) -> Rc<Bound<T>> {
        Rc::clone(&self.max.as_ref().unwrap())
    }

    fn update_height(&mut self) {
        self.height = (1 + Node::_max_height(&self.left_child, &self.right_child)) as usize;
    }

    fn update_size(&mut self) {
        self.size = 1 + Node::size(&self.left_child) + Node::size(&self.right_child);
    }

    fn update_max(&mut self) {
        let max = match (&self.left_child, &self.right_child) {
            (Some(_left_child), Some(_right_child)) => Node::find_max(
                self.interval().get_high(),
                Node::find_max(_left_child.get_max(), _right_child.get_max()),
            ),
            (Some(_left_child), None) => {
                Node::find_max(self.interval().get_high(), _left_child.get_max())
            }
            (None, Some(_right_child)) => {
                Node::find_max(self.interval().get_high(), _right_child.get_max())
            }
            (None, None) => self.interval().get_high(),
        };

        self.max = Some(Rc::clone(&max));
    }

    fn find_max(bound1: Rc<Bound<T>>, bound2: Rc<Bound<T>>) -> Rc<Bound<T>> {
        match (bound1.as_ref(), bound2.as_ref()) {
            (Included(_val1), Included(_val2))
            | (Included(_val1), Excluded(_val2))
            | (Excluded(_val1), Excluded(_val2)) => {
                if _val1 >= _val2 {
                    bound1
                } else {
                    bound2
                }
            }
            (Excluded(_val1), Included(_val2)) => {
                if _val1 > _val2 {
                    bound1
                } else {
                    bound2
                }
            }
            (Unbounded, _) => bound1,
            (_, Unbounded) => bound2,
        }
    }

    fn is_ge(bound1: Rc<Bound<T>>, bound2: Rc<Bound<T>>) -> bool {
        match (bound1.as_ref(), bound2.as_ref()) {
            (Included(_val1), Included(_val2)) => _val1 >= _val2,
            (Included(_val1), Excluded(_val2)) => _val1 > _val2,
            (Excluded(_val1), Included(_val2)) => _val1 > _val2,
            (Excluded(_val1), Excluded(_val2)) => _val1 > _val2,

            (Unbounded, Included(_val2)) => true,
            (Unbounded, Excluded(_val2)) => true,
            (Included(_val1), Unbounded) => true,
            (Excluded(_val1), Unbounded) => true,

            (Unbounded, Unbounded) => true,
        }
    }

    fn _max_height(node1: &Option<Box<Node<T>>>, node2: &Option<Box<Node<T>>>) -> i64 {
        std::cmp::max(Node::height(node1), Node::height(node2))
    }

    fn height(node: &Option<Box<Node<T>>>) -> i64 {
        match node {
            Some(_node) => _node.height as i64,
            None => -1,
        }
    }

    fn size(node: &Option<Box<Node<T>>>) -> usize {
        match node {
            Some(_node) => _node.size,
            None => 0,
        }
    }

    fn balance_factor(node: &Box<Node<T>>) -> i64 {
        Node::height(&node.left_child) - Node::height(&node.right_child)
    }
}

pub struct IntervalTree<T: Ord> {
    root: Option<Box<Node<T>>>,
}

impl<T: Ord> IntervalTree<T> {
    pub fn init() -> IntervalTree<T> {
        IntervalTree { root: None }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn size(&self) -> usize {
        Node::size(&self.root)
    }
    pub fn height(&self) -> i64 {
        Node::height(&self.root)
    }

    pub fn overlaps(&self, interval: &Interval<T>) -> bool {
        self.find_overlap(interval).is_some()
    }

    pub fn find_overlap(&self, interval: &Interval<T>) -> Option<Interval<T>> {
        IntervalTree::_find_overlap(&self.root, interval)
    }

    fn _find_overlap(node: &Option<Box<Node<T>>>, interval: &Interval<T>) -> Option<Interval<T>> {
        if node.is_none() {
            return None;
        }
        let mut current = node;
        while current.is_some() {
            let node_ref = current.as_ref().unwrap();
            if Interval::overlaps(node_ref.interval(), interval) {
                break;
            }

            if node_ref.left_child.is_some()
                && Node::is_ge(
                    node_ref.left_child.as_ref().unwrap().get_max(),
                    interval.get_low(),
                )
            {
                current = &node_ref.left_child;
            } else {
                current = &node_ref.right_child;
            }
        }

        if current.is_none() {
            None
        } else {
            Some(current.as_ref().unwrap().interval().duplicate())
        }
    }

    pub fn find_overlaps(&self, interval: &Interval<T>) -> Vec<Interval<T>> {
        let mut overlaps = Vec::<Interval<T>>::new();

        IntervalTree::_find_overlaps(&self.root, interval, &mut overlaps);

        overlaps
    }

    fn _find_overlaps(
        node: &Option<Box<Node<T>>>,
        interval: &Interval<T>,
        overlaps: &mut Vec<Interval<T>>,
    ) {
        if node.is_none() {
            return;
        }
        let node_ref = node.as_ref().unwrap();
        if Interval::overlaps(node_ref.interval(), interval) {
            overlaps.push(node_ref.interval().duplicate());
        }

        if node_ref.left_child.is_some()
            && Node::is_ge(
                node_ref.left_child.as_ref().unwrap().get_max(),
                interval.get_low(),
            )
        {
            IntervalTree::_find_overlaps(&node_ref.left_child, interval, overlaps);
        }
        IntervalTree::_find_overlaps(&node_ref.right_child, interval, overlaps);
    }

    pub fn insert(&mut self, interval: Interval<T>) {
        let max = interval.get_high();

        self.root = IntervalTree::_insert(self.root.take(), interval, max);
    }

    fn _insert(
        node: Option<Box<Node<T>>>,
        interval: Interval<T>,
        max: Rc<Bound<T>>,
    ) -> Option<Box<Node<T>>> {
        if node.is_none() {
            return Some(Box::new(Node::init(interval, max, 0, 1)));
        }

        let mut node_ref = node.unwrap();

        if interval < *node_ref.interval() {
            node_ref.left_child = IntervalTree::_insert(node_ref.left_child, interval, max);
        } else if interval > *node_ref.interval() {
            node_ref.right_child = IntervalTree::_insert(node_ref.right_child, interval, max);
        } else {
            return Some(node_ref);
        }

        node_ref.update_height();
        node_ref.update_size();
        node_ref.update_max();

        Some(IntervalTree::balance(node_ref))
    }

    fn balance(mut node: Box<Node<T>>) -> Box<Node<T>> {
        if Node::balance_factor(&node) < -1 {
            if Node::balance_factor(node.right_child.as_ref().unwrap()) > 0 {
                node.right_child = Some(IntervalTree::rotate_right(node.right_child.unwrap()));
            }
            node = IntervalTree::rotate_left(node);
        } else if Node::balance_factor(&node) > 1 {
            if Node::balance_factor(node.left_child.as_ref().unwrap()) < 0 {
                node.left_child = Some(IntervalTree::rotate_left(node.left_child.unwrap()));
            }
            node = IntervalTree::rotate_right(node);
        }
        node
    }

    fn rotate_right(mut node: Box<Node<T>>) -> Box<Node<T>> {
        let mut y = node.left_child.unwrap();
        node.left_child = y.right_child;
        y.size = node.size;
        node.update_height();
        node.update_size();
        node.update_max();

        y.right_child = Some(node);
        y.update_height();
        y.update_max();

        y
    }

    fn rotate_left(mut node: Box<Node<T>>) -> Box<Node<T>> {
        let mut y = node.right_child.unwrap();
        node.right_child = y.left_child;
        y.size = node.size;

        node.update_height();
        node.update_size();
        node.update_max();

        y.left_child = Some(node);
        y.update_height();
        y.update_max();

        y
    }

    pub fn delete(&mut self, interval: &Interval<T>) {
        if !self.is_empty() {
            self.root = IntervalTree::_delete(self.root.take(), interval);
        }
    }

    fn _delete(node: Option<Box<Node<T>>>, interval: &Interval<T>) -> Option<Box<Node<T>>> {
        match node {
            None => node,
            Some(mut _node) => {
                if *interval < *_node.interval() {
                    _node.left_child = IntervalTree::_delete(_node.left_child.take(), interval);
                } else if *interval > *_node.interval() {
                    _node.right_child = IntervalTree::_delete(_node.right_child.take(), interval);
                } else {
                    if _node.left_child.is_none() {
                        return _node.right_child;
                    } else if _node.right_child.is_none() {
                        return _node.left_child;
                    } else {
                        let mut y = _node;
                        _node = IntervalTree::_min(&mut y.right_child);
                        _node.right_child = IntervalTree::_delete_min(y.right_child.unwrap());
                        _node.left_child = y.left_child;
                    }
                }

                _node.update_height();
                _node.update_size();
                _node.update_max();
                Some(IntervalTree::balance(_node))
            }
        }
    }
    fn _min(node: &mut Option<Box<Node<T>>>) -> Box<Node<T>> {
        match node {
            Some(_node) => {
                if _node.left_child.is_none() {
                    Box::new(Node::init(_node.get_interval(), _node.get_max(), 0, 1))
                } else {
                    IntervalTree::_min(&mut _node.left_child)
                }
            }
            None => panic!("Called min on None node"),
        }
    }

    pub fn delete_min(&mut self) {
        if !self.is_empty() {
            self.root = IntervalTree::_delete_min(self.root.take().unwrap());
        }
    }

    fn _delete_min(mut node: Box<Node<T>>) -> Option<Box<Node<T>>> {
        if node.left_child.is_none() {
            return node.right_child.take();
        }

        node.left_child = IntervalTree::_delete_min(node.left_child.unwrap());

        node.update_height();
        node.update_size();
        node.update_size();

        Some(IntervalTree::balance(node))
    }

    pub fn delete_max(&mut self) {
        if !self.is_empty() {
            self.root = IntervalTree::_delete_max(self.root.take().unwrap());
        }
    }

    fn _delete_max(mut node: Box<Node<T>>) -> Option<Box<Node<T>>> {
        if node.right_child.is_none() {
            return node.left_child.take();
        }

        node.right_child = IntervalTree::_delete_max(node.right_child.unwrap());

        node.update_height();
        node.update_size();
        node.update_max();

        Some(IntervalTree::balance(node))
    }

    pub fn select(&self, k: usize) -> Option<Interval<T>> {
        if k > self.size() {
            panic!("K must be in range 0 <= k <= size - 1");
        }
        IntervalTree::_select(&self.root, k)
    }

    fn _select(node: &Option<Box<Node<T>>>, k: usize) -> Option<Interval<T>> {
        if node.is_none() {
            return None;
        }
        let node_ref = node.as_ref().unwrap();

        let t = Node::size(&node_ref.left_child);
        if t > k {
            return IntervalTree::_select(&node_ref.left_child, k);
        } else if t < k {
            return IntervalTree::_select(&node_ref.right_child, k - t - 1);
        } else {
            return Some(node_ref.interval().duplicate());
        }
    }

    pub fn min(&self) -> Option<Interval<T>> {
        self.select(0)
    }

    pub fn max(&self) -> Option<Interval<T>> {
        self.select(self.size() - 1)
    }

    pub fn intervals_between(
        &self,
        low_key: &Interval<T>,
        high_key: &Interval<T>,
    ) -> Vec<&Interval<T>> {
        let mut keys: Vec<&Interval<T>> = Vec::new();

        IntervalTree::_intervals_between(&self.root, low_key, high_key, &mut keys);

        keys
    }

    fn _intervals_between<'a>(
        node: &'a Option<Box<Node<T>>>,
        low_key: &Interval<T>,
        high_key: &Interval<T>,
        keys: &mut Vec<&'a Interval<T>>,
    ) {
        if node.is_none() {
            return;
        }

        let node_ref = node.as_ref().unwrap();
        if *low_key < *node_ref.interval() {
            IntervalTree::_intervals_between(&node_ref.left_child, low_key, high_key, keys);
        }
        if *low_key <= *node_ref.interval() && *node_ref.interval() <= *high_key {
            keys.push(node_ref.interval());
        }
        if *high_key > *node_ref.interval() {
            IntervalTree::_intervals_between(&node_ref.right_child, low_key, high_key, keys);
        }
    }

    pub fn rank(&self, interval: &Interval<T>) -> usize {
        IntervalTree::_rank(&self.root, interval)
    }
    fn _rank(node: &Option<Box<Node<T>>>, interval: &Interval<T>) -> usize {
        if node.is_none() {
            return 0;
        }
        let node_ref = node.as_ref().unwrap();
        if *interval < *node_ref.interval() {
            IntervalTree::_rank(&node_ref.left_child, interval)
        } else if *interval > *node_ref.interval() {
            1 + Node::size(&node_ref.left_child)
                + IntervalTree::_rank(&node_ref.right_child, interval)
        } else {
            Node::size(&node_ref.left_child)
        }
    }
    pub fn size_between(&self, low_key: &Interval<T>, high_key: &Interval<T>) -> usize {
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

    #[test]
    fn tree_interval_init() {
        let interval_tree = IntervalTree::<usize>::init();

        assert_eq!(interval_tree.is_empty(), true);
        assert_eq!(interval_tree.size(), 0);
    }

    #[test]
    fn tree_interval_insert() {
        let mut interval_tree = IntervalTree::<usize>::init();

        interval_tree.insert(Interval::new(Included(0), Included(3)));
        interval_tree.insert(Interval::new(Included(5), Included(8)));
        interval_tree.insert(Interval::new(Included(6), Included(10)));
        interval_tree.insert(Interval::new(Included(8), Included(9)));
        interval_tree.insert(Interval::new(Included(15), Included(23)));
        interval_tree.insert(Interval::new(Included(16), Included(21)));
        interval_tree.insert(Interval::new(Included(17), Included(19)));
        interval_tree.insert(Interval::new(Included(19), Included(20)));
        interval_tree.insert(Interval::new(Included(25), Included(30)));
        interval_tree.insert(Interval::new(Included(26), Included(26)));

        assert_eq!(interval_tree.size(), 10);
    }

    #[test]
    fn tree_interval_find_overlap_1() {
        let mut interval_tree = IntervalTree::<usize>::init();

        interval_tree.insert(Interval::new(Included(0), Included(3)));
        interval_tree.insert(Interval::new(Included(5), Included(8)));
        interval_tree.insert(Interval::new(Included(6), Included(10)));
        interval_tree.insert(Interval::new(Included(8), Included(9)));
        interval_tree.insert(Interval::new(Included(15), Included(23)));
        interval_tree.insert(Interval::new(Included(16), Included(21)));
        interval_tree.insert(Interval::new(Included(17), Included(19)));
        interval_tree.insert(Interval::new(Included(19), Included(20)));
        interval_tree.insert(Interval::new(Included(25), Included(30)));
        interval_tree.insert(Interval::new(Included(26), Included(26)));

        assert!(
            format!(
                "{}",
                interval_tree
                    .find_overlap(&Interval::new(Included(1), Included(2)))
                    .unwrap()
            ) == String::from("[0,3]")
        );

        assert!(
            format!(
                "{}",
                interval_tree
                    .find_overlap(&Interval::new(Included(4), Included(5)))
                    .unwrap()
            ) == String::from("[5,8]")
        );

        assert!(
            format!(
                "{}",
                interval_tree
                    .find_overlap(&Interval::new(Included(10), Included(14)))
                    .unwrap()
            ) == String::from("[6,10]")
        );

        assert!(
            format!(
                "{}",
                interval_tree
                    .find_overlap(&Interval::new(Included(14), Included(15)))
                    .unwrap()
            ) == String::from("[15,23]")
        );

        assert!(
            format!(
                "{}",
                interval_tree
                    .find_overlap(&Interval::new(Included(15), Included(18)))
                    .unwrap()
            ) == String::from("[16,21]")
        );

        assert!(
            format!(
                "{}",
                interval_tree
                    .find_overlap(&Interval::new(Included(19), Included(19)))
                    .unwrap()
            ) == String::from("[19,20]")
        );

        assert!(
            format!(
                "{}",
                interval_tree
                    .find_overlap(&Interval::new(Included(23), Included(23)))
                    .unwrap()
            ) == String::from("[15,23]")
        );

        assert!(
            format!(
                "{}",
                interval_tree
                    .find_overlap(&Interval::new(Included(24), Included(26)))
                    .unwrap()
            ) == String::from("[25,30]")
        );

        assert!(
            format!(
                "{}",
                interval_tree
                    .find_overlap(&Interval::new(Included(26), Included(36)))
                    .unwrap()
            ) == String::from("[25,30]")
        );

        assert!(interval_tree
            .find_overlap(&Interval::new(Included(31), Included(36)))
            .is_none());
        assert!(interval_tree
            .find_overlap(&Interval::new(Included(12), Included(12)))
            .is_none());
        assert!(interval_tree
            .find_overlap(&Interval::new(Included(13), Included(13)))
            .is_none());
        assert!(interval_tree
            .find_overlap(&Interval::new(Included(12), Included(14)))
            .is_none());
    }

    #[test]
    fn tree_interval_find_overlap_2() {
        let mut interval_tree = IntervalTree::<usize>::init();

        interval_tree.insert(Interval::new(Included(0), Excluded(3)));
        interval_tree.insert(Interval::new(Excluded(5), Included(8)));
        interval_tree.insert(Interval::new(Included(6), Included(10)));
        interval_tree.insert(Interval::new(Excluded(8), Included(9)));
        interval_tree.insert(Interval::new(Excluded(15), Excluded(23)));
        interval_tree.insert(Interval::new(Included(16), Excluded(21)));
        interval_tree.insert(Interval::new(Included(17), Excluded(19)));
        interval_tree.insert(Interval::new(Excluded(19), Included(20)));
        interval_tree.insert(Interval::new(Excluded(25), Included(30)));
        interval_tree.insert(Interval::new(Included(26), Included(26)));

        assert!(
            format!(
                "{}",
                interval_tree
                    .find_overlap(&Interval::new(Included(1), Included(2)))
                    .unwrap()
            ) == String::from("[0,3)")
        );

        assert!(interval_tree
            .find_overlap(&Interval::new(Included(4), Included(5)))
            .is_none());

        assert!(
            format!(
                "{}",
                interval_tree
                    .find_overlap(&Interval::new(Included(10), Included(14)))
                    .unwrap()
            ) == String::from("[6,10]")
        );

        assert!(interval_tree
            .find_overlap(&Interval::new(Included(14), Included(15)))
            .is_none());

        assert!(
            format!(
                "{}",
                interval_tree
                    .find_overlap(&Interval::new(Included(15), Included(18)))
                    .unwrap()
            ) == String::from("[16,21)")
        );

        assert!(
            format!(
                "{}",
                interval_tree
                    .find_overlap(&Interval::new(Included(19), Included(19)))
                    .unwrap()
            ) == String::from("[16,21)")
        );

        assert!(
            format!(
                "{}",
                interval_tree
                    .find_overlap(&Interval::new(Excluded(23), Included(26)))
                    .unwrap()
            ) == String::from("(25,30]")
        );

        assert!(interval_tree
            .find_overlap(&Interval::new(Excluded(10), Excluded(15)))
            .is_none());

        assert!(
            format!(
                "{}",
                interval_tree
                    .find_overlap(&Interval::new(Excluded(21), Included(23)))
                    .unwrap()
            ) == String::from("(15,23)")
        );

        assert!(interval_tree
            .find_overlap(&Interval::new(Included(31), Included(36)))
            .is_none());
        assert!(interval_tree
            .find_overlap(&Interval::new(Included(12), Included(12)))
            .is_none());
        assert!(interval_tree
            .find_overlap(&Interval::new(Included(13), Included(13)))
            .is_none());
        assert!(interval_tree
            .find_overlap(&Interval::new(Included(12), Included(14)))
            .is_none());
    }

    #[test]
    fn tree_interval_find_overlap_3() {
        let mut interval_tree = IntervalTree::<usize>::init();

        interval_tree.insert(Interval::new(Unbounded, Excluded(3)));
        interval_tree.insert(Interval::new(Excluded(5), Included(8)));
        interval_tree.insert(Interval::new(Included(6), Included(10)));
        interval_tree.insert(Interval::new(Unbounded, Included(9)));
        interval_tree.insert(Interval::new(Excluded(15), Excluded(23)));
        interval_tree.insert(Interval::new(Unbounded, Excluded(21)));
        interval_tree.insert(Interval::new(Included(17), Excluded(19)));
        interval_tree.insert(Interval::new(Excluded(19), Unbounded));
        interval_tree.insert(Interval::new(Unbounded, Included(30)));
        interval_tree.insert(Interval::new(Included(26), Unbounded));

        assert!(
            format!(
                "{}",
                interval_tree
                    .find_overlap(&Interval::new(Included(1), Included(2)))
                    .unwrap()
            ) == String::from("(_,9]")
        );

        assert!(
            format!(
                "{}",
                interval_tree
                    .find_overlap(&Interval::new(Included(4), Included(5)))
                    .unwrap()
            ) == String::from("(_,9]")
        );

        assert!(
            format!(
                "{}",
                interval_tree
                    .find_overlap(&Interval::new(Included(10), Included(14)))
                    .unwrap()
            ) == String::from("(_,21)")
        );

        assert!(
            format!(
                "{}",
                interval_tree
                    .find_overlap(&Interval::new(Included(14), Included(15)))
                    .unwrap()
            ) == String::from("(_,21)")
        );

        assert!(
            format!(
                "{}",
                interval_tree
                    .find_overlap(&Interval::new(Included(15), Included(18)))
                    .unwrap()
            ) == String::from("(_,21)")
        );

        assert!(
            format!(
                "{}",
                interval_tree
                    .find_overlap(&Interval::new(Included(19), Included(19)))
                    .unwrap()
            ) == String::from("(_,21)")
        );

        assert!(
            format!(
                "{}",
                interval_tree
                    .find_overlap(&Interval::new(Excluded(23), Included(26)))
                    .unwrap()
            ) == String::from("(_,30]")
        );

        assert!(
            format!(
                "{}",
                interval_tree
                    .find_overlap(&Interval::new(Excluded(21), Included(23)))
                    .unwrap()
            ) == String::from("(_,30]")
        );

        assert!(
            format!(
                "{}",
                interval_tree
                    .find_overlap(&Interval::new(Unbounded, Included(0)))
                    .unwrap()
            ) == String::from("(_,9]")
        );
    }

    #[test]
    fn tree_interval_delete_1() {
        let mut interval_tree = IntervalTree::<usize>::init();

        interval_tree.insert(Interval::new(Included(0), Excluded(3)));
        interval_tree.insert(Interval::new(Excluded(5), Included(8)));
        interval_tree.insert(Interval::new(Included(6), Included(10)));
        interval_tree.insert(Interval::new(Excluded(8), Included(9)));
        interval_tree.insert(Interval::new(Excluded(15), Excluded(23)));
        interval_tree.insert(Interval::new(Included(16), Excluded(21)));
        interval_tree.insert(Interval::new(Included(17), Excluded(19)));
        interval_tree.insert(Interval::new(Excluded(19), Included(20)));
        interval_tree.insert(Interval::new(Excluded(25), Included(30)));
        interval_tree.insert(Interval::new(Included(26), Included(26)));
        let mut interval = Interval::new(Included(1), Included(2));
        let mut overlapped_interval = interval_tree.find_overlap(&interval).unwrap();
        interval_tree.delete(&overlapped_interval);
        assert!(interval_tree.find_overlap(&interval).is_none());

        interval = Interval::new(Included(15), Included(18));
        overlapped_interval = interval_tree.find_overlap(&interval).unwrap();
        interval_tree.delete(&overlapped_interval);
        overlapped_interval = interval_tree.find_overlap(&interval).unwrap();
        interval_tree.delete(&overlapped_interval);
        overlapped_interval = interval_tree.find_overlap(&interval).unwrap();
        interval_tree.delete(&overlapped_interval);
        assert!(interval_tree.find_overlap(&interval).is_none());
    }

    #[test]
    fn tree_interval_delete_max_1() {
        let mut interval_tree = IntervalTree::<usize>::init();

        interval_tree.insert(Interval::new(Included(0), Excluded(3)));
        interval_tree.insert(Interval::new(Excluded(5), Included(8)));
        interval_tree.insert(Interval::new(Included(6), Included(10)));
        interval_tree.insert(Interval::new(Excluded(8), Included(9)));
        interval_tree.insert(Interval::new(Excluded(15), Excluded(23)));
        interval_tree.insert(Interval::new(Included(16), Excluded(21)));
        interval_tree.insert(Interval::new(Included(17), Excluded(19)));
        interval_tree.insert(Interval::new(Excluded(19), Included(20)));
        interval_tree.insert(Interval::new(Excluded(25), Included(30)));
        interval_tree.insert(Interval::new(Included(26), Included(26)));
        interval_tree.delete_max();
        interval_tree.delete_max();

        assert!(interval_tree
            .find_overlap(&Interval::new(Included(23), Included(23)))
            .is_none());
    }

    #[test]
    fn tree_interval_delete_min_1() {
        let mut interval_tree = IntervalTree::<usize>::init();

        interval_tree.insert(Interval::new(Included(0), Excluded(3)));
        interval_tree.insert(Interval::new(Excluded(5), Included(8)));
        interval_tree.insert(Interval::new(Included(6), Included(10)));
        interval_tree.insert(Interval::new(Excluded(8), Included(9)));
        interval_tree.insert(Interval::new(Excluded(15), Excluded(23)));
        interval_tree.insert(Interval::new(Included(16), Excluded(21)));
        interval_tree.insert(Interval::new(Included(17), Excluded(19)));
        interval_tree.insert(Interval::new(Excluded(19), Included(20)));
        interval_tree.insert(Interval::new(Excluded(25), Included(30)));
        interval_tree.insert(Interval::new(Included(26), Included(26)));
        interval_tree.delete_min();
        interval_tree.delete_min();

        assert!(interval_tree
            .find_overlap(&Interval::new(Included(1), Excluded(6)))
            .is_none());
    }

    #[test]
    fn tree_interval_select_1() {
        let mut interval_tree = IntervalTree::<usize>::init();

        interval_tree.insert(Interval::new(Excluded(0), Included(1)));
        interval_tree.insert(Interval::new(Included(0), Excluded(3)));
        interval_tree.insert(Interval::new(Included(6), Included(10)));
        interval_tree.insert(Interval::new(Excluded(8), Included(9)));
        interval_tree.insert(Interval::new(Excluded(15), Excluded(23)));
        interval_tree.insert(Interval::new(Included(16), Excluded(21)));
        interval_tree.insert(Interval::new(Included(17), Excluded(19)));
        interval_tree.insert(Interval::new(Excluded(19), Included(20)));
        interval_tree.insert(Interval::new(Excluded(25), Included(30)));
        interval_tree.insert(Interval::new(Included(26), Included(26)));
        assert!(format!("{}", interval_tree.select(0).unwrap()) == String::from("[0,3)"));
        assert!(format!("{}", interval_tree.select(1).unwrap()) == String::from("(0,1]"));
        assert!(format!("{}", interval_tree.select(2).unwrap()) == String::from("[6,10]"));
        assert!(format!("{}", interval_tree.select(3).unwrap()) == String::from("(8,9]"));
        assert!(format!("{}", interval_tree.select(4).unwrap()) == String::from("(15,23)"));
        assert!(format!("{}", interval_tree.select(5).unwrap()) == String::from("[16,21)"));
        assert!(format!("{}", interval_tree.select(6).unwrap()) == String::from("[17,19)"));
        assert!(format!("{}", interval_tree.select(7).unwrap()) == String::from("(19,20]"));
        assert!(format!("{}", interval_tree.select(8).unwrap()) == String::from("(25,30]"));
        assert!(format!("{}", interval_tree.select(9).unwrap()) == String::from("[26,26]"));
    }

    #[test]
    fn tree_interval_intervals_between_1() {
        let mut interval_tree = IntervalTree::<usize>::init();

        interval_tree.insert(Interval::new(Excluded(0), Included(1)));
        interval_tree.insert(Interval::new(Included(0), Excluded(3)));
        interval_tree.insert(Interval::new(Included(6), Included(10)));
        interval_tree.insert(Interval::new(Excluded(8), Included(9)));
        interval_tree.insert(Interval::new(Excluded(15), Excluded(23)));
        interval_tree.insert(Interval::new(Included(16), Excluded(21)));
        interval_tree.insert(Interval::new(Included(17), Excluded(19)));
        interval_tree.insert(Interval::new(Excluded(19), Included(20)));
        interval_tree.insert(Interval::new(Excluded(25), Included(30)));
        interval_tree.insert(Interval::new(Included(26), Included(26)));

        let low = Interval::new(Included(14), Included(14));
        let high = Interval::new(Included(24), Included(24));
        let intervals = interval_tree.intervals_between(&low, &high);

        let accept = String::from("(15,23)[16,21)[17,19)(19,20]");

        let mut result = String::from("");
        for interval in intervals {
            result.push_str(&format!("{}", interval))
        }

        assert_eq!(result, accept);
    }

    #[test]
    fn tree_interval_find_overlaps_1() {
        let mut interval_tree = IntervalTree::<usize>::init();

        interval_tree.insert(Interval::new(Excluded(0), Included(1)));
        interval_tree.insert(Interval::new(Included(0), Excluded(3)));
        interval_tree.insert(Interval::new(Included(6), Included(10)));
        interval_tree.insert(Interval::new(Excluded(8), Included(9)));
        interval_tree.insert(Interval::new(Excluded(15), Excluded(23)));
        interval_tree.insert(Interval::new(Included(16), Excluded(21)));
        interval_tree.insert(Interval::new(Included(17), Excluded(19)));
        interval_tree.insert(Interval::new(Excluded(19), Included(20)));
        interval_tree.insert(Interval::new(Excluded(25), Included(30)));
        interval_tree.insert(Interval::new(Included(26), Included(26)));

        let interval = Interval::new(Included(8), Included(26));
        let intervals = interval_tree.find_overlaps(&interval);

        let accept = String::from("(8,9][6,10](19,20][16,21)(15,23)[17,19)(25,30][26,26]");

        let mut result = String::from("");
        for interval in intervals {
            result.push_str(&format!("{}", interval))
        }

        assert_eq!(result, accept);
    }
}
