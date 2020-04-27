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

    fn add(&mut self, binomial_tree: BinomialTree<T>) {
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
            panic!("Payload is None");
        }

        self.payload.take().unwrap()
    }

    pub fn peek_payload(&self) -> &Option<T> {
        &self.payload
    }

    pub fn is_smaller_or_equall(&self, other: &BinomialTree<T>) -> bool {
        match (self.peek_payload(), other.peek_payload()) {
            (Some(payload1), Some(payload2)) => payload1 <= payload2,
            _ => panic!("Payloads can not be None"),
        }
    }

    pub fn children_mut(&mut self) -> &mut Vec<Option<BinomialTree<T>>> {
        &mut self.children
    }

    pub fn children(&self) -> &Vec<Option<BinomialTree<T>>> {
        &self.children
    }
}

impl<T: std::cmp::Ord + std::fmt::Display> BinomialTree<T> {
    pub fn preorder(node: &BinomialTree<T>) -> String {
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
                    None => (),
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
}