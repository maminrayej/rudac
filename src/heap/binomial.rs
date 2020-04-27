#[derive(Debug)]
pub struct BinomialTree<T: std::cmp::Ord> {
    rank: usize,
    children: Vec<Option<BinomialTree<T>>>,
    payload: Option<T>,
}

impl<T: std::cmp::Ord> BinomialTree<T> {
    pub fn init(payload: T) -> BinomialTree<T> {
        BinomialTree {
            rank: 0,
            children: Vec::new(),
            payload: Some(payload),
        }
    }

    pub fn add(&mut self, binomial_tree: BinomialTree<T>) {
        self.children.push(Some(binomial_tree));
        self.rank += 1;
    }

    pub fn merge(
        mut binomial_tree_1: BinomialTree<T>,
        mut binomial_tree_2: BinomialTree<T>,
    ) -> BinomialTree<T> {
        if binomial_tree_1.rank() != binomial_tree_2.rank {
            panic!("Binomial tree ranks must be the same when merging");
        }
        if binomial_tree_1.is_smaller_or_equall(&binomial_tree_2) {
            binomial_tree_1.add(binomial_tree_2);
            return binomial_tree_1;
        } else {
            binomial_tree_2.add(binomial_tree_1);
            return binomial_tree_2;
        }
    }

    pub fn rank(&self) -> usize {
        self.rank
    }

    pub fn get_payload(&mut self) -> T {
        if self.payload == None {
            panic!("Internal error: payload must not be None. Please report this bug");
        }

        self.payload.take().unwrap()
    }

    fn is_smaller_or_equall(&self, other: &BinomialTree<T>) -> bool {
        match (self.peek_payload(), other.peek_payload()) {
            (Some(payload1), Some(payload2)) => payload1 <= payload2,
            _ => panic!("Payloads can not be None. Please report this bug"),
        }
    }

    pub fn peek_payload(&self) -> &Option<T> {
        return &self.payload;
    }
}

impl<T: std::cmp::Ord + std::fmt::Display> BinomialTree<T> {
    fn preorder(node: &BinomialTree<T>) -> String {
        return String::from(BinomialTree::_pre_visit(&Some(node)).trim());
    }

    fn _pre_visit(node: &Option<&BinomialTree<T>>) -> String {
        let mut node_list = String::from("");
        match node {
            None => node_list,
            Some(data) => {
                // visit the node
                match data.peek_payload() {
                    Some(value) => node_list.push_str(format!("{} ", value).as_str()),
                    None => panic!("Payload can not be None. Please report this bug"),
                }
                //visit children from left to right
                for i in 0..data.children.len() {
                    match &data.children[i] {
                        Some(data) => {
                            node_list.push_str(BinomialTree::_pre_visit(&Some(&data)).as_str())
                        }
                        None => (),
                    }
                }

                node_list
            }
        }
    }
}

pub struct BinomialHeap<T: std::cmp::Ord> {
    roots: Vec<Option<BinomialTree<T>>>,
}

impl<T: std::cmp::Ord> BinomialHeap<T> {
    pub fn init(payload: T) -> BinomialHeap<T> {
        let root = Some(BinomialTree::init(payload));

        let mut roots = Vec::new();

        roots.push(root);

        BinomialHeap { roots: roots }
    }

    fn _push(&mut self, mut new_node: BinomialTree<T>) {
        let max_rank = self.roots.len();
        let start_rank = new_node.rank();

        for i in start_rank..max_rank {
            match self.roots[i].take() {
                Some(node) => {
                    new_node = BinomialTree::merge(node, new_node);

                    if i == max_rank - 1 {
                        self.roots.push(Some(new_node));
                        break;
                    }
                }
                None => {
                    self.roots[i] = Some(new_node);
                    break;
                }
            }
        }
    }

    pub fn push(&mut self, payload: T) {
        let new_node = BinomialTree::init(payload);

        self._push(new_node);
    }

    pub fn pop(&mut self) -> T {
        let mut min_index = 0;
        let mut min_node: &Option<BinomialTree<T>>;

        // find first non-None item
        for i in 0..self.roots.len() {
            match &self.roots[i] {
                Some(_) => min_index = i,
                None => (),
            }
        }

        min_node = &self.roots[min_index];

        // find first item
        for i in min_index + 1..self.roots.len() {
            match (&self.roots[i], min_node) {
                (Some(node), Some(min)) => {
                    if node.is_smaller_or_equall(min) {
                        min_index = i;
                    }
                }
                _ => panic!("Neither 'node' or 'min' should be None. Please report this bug"),
            }
            min_node = &self.roots[min_index];
        }

        let mut min_node_taken = self.roots[min_index].take().unwrap();

        // push children of the min node into heap
        for i in 0..min_node_taken.children.len() {
            let child = min_node_taken.children[i].take().unwrap();

            self._push(child);
        }

        // return payload the min node
        min_node_taken.get_payload()
    }
}

impl<T: std::cmp::Ord + std::fmt::Display> BinomialHeap<T> {
    pub fn preorder(&self) -> String {
        let mut node_list = String::from("");

        for i in 0..self.roots.len() {
            node_list.push_str(format!("Rank {}: ", i).as_str());

            match &self.roots[i] {
                Some(data) => node_list.push_str(BinomialTree::preorder(&data).as_str()),
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
    fn binomial_tree_create() {
        let bt = BinomialTree::init(0);

        assert_eq!(*bt.peek_payload(), Some(0));
        assert_eq!(bt.rank, 0);
        assert_eq!(bt.children.len(), 0);
        assert_eq!(BinomialTree::preorder(&bt), String::from("0"));
    }

    #[test]
    fn binomial_tree_merge_rank_0() {
        let bt1 = BinomialTree::init(0);
        let bt2 = BinomialTree::init(1);

        let merged_tree = BinomialTree::merge(bt1, bt2);

        assert_eq!(BinomialTree::preorder(&merged_tree), String::from("0 1"));

        assert_eq!(merged_tree.rank(), 1);
    }

    #[test]
    fn binomial_tree_merge_rank_1() {
        let bt1 = BinomialTree::init(0);
        let bt2 = BinomialTree::init(1);

        let merged_tree_1 = BinomialTree::merge(bt1, bt2);
        let bt3 = BinomialTree::init(2);
        let bt4 = BinomialTree::init(3);

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
        let bt1 = BinomialTree::init(0);
        let bt2 = BinomialTree::init(1);

        let merged_tree_1 = BinomialTree::merge(bt1, bt2);
        let bt3 = BinomialTree::init(2);
        let bt4 = BinomialTree::init(3);

        let merged_tree_2 = BinomialTree::merge(bt3, bt4);

        let merged_tree_final_1 = BinomialTree::merge(merged_tree_1, merged_tree_2);

        let bt1 = BinomialTree::init(1);
        let bt2 = BinomialTree::init(2);

        let merged_tree_1 = BinomialTree::merge(bt1, bt2);
        let bt3 = BinomialTree::init(5);
        let bt4 = BinomialTree::init(6);

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
    fn binomial_heap_init() {
        let bh = BinomialHeap::init(0);

        assert_eq!(bh.preorder(), format!("Rank 0: 0\n"))
    }

    #[test]
    fn binomial_heap_push_1() {
        let mut bh = BinomialHeap::init(0);

        bh.push(1);

        assert_eq!(bh.preorder(), format!("Rank 0: \nRank 1: 0 1\n"));
    }

    #[test]
    fn binomial_heap_push_2() {
        let mut bh = BinomialHeap::init(0);

        bh.push(1);
        bh.push(2);

        assert_eq!(bh.preorder(), format!("Rank 0: 2\nRank 1: 0 1\n"));
    }

    #[test]
    fn binomial_heap_push_3() {
        let mut bh = BinomialHeap::init(0);

        bh.push(1);
        bh.push(2);

        bh.push(3);

        assert_eq!(
            bh.preorder(),
            format!("Rank 0: \nRank 1: \nRank 2: 0 1 2 3\n")
        );
    }

    #[test]
    fn binomial_heap_pop() {
        let mut bh = BinomialHeap::init(0);

        bh.push(1);
        bh.push(2);

        bh.push(3);

        let value = bh.pop();

        assert_eq!(bh.preorder(), format!("Rank 0: 1\nRank 1: 2 3\nRank 2: \n"));

        assert_eq!(value, 0)
    }
}
