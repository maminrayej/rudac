use crate::tree::BinomialTree;

pub struct BinomialHeap<T: std::cmp::Ord> {
    roots: Vec<Option<BinomialTree<T>>>,
    size: usize,
}

impl<T: std::cmp::Ord> BinomialHeap<T> {
    pub fn init(payload: T) -> BinomialHeap<T> {
        let root = Some(BinomialTree::init(payload));

        let mut roots = Vec::new();

        roots.push(root);

        BinomialHeap {
            roots: roots,
            size: 1,
        }
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

        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }

        let mut min_index = 0;
        let mut min_node: &Option<BinomialTree<T>>;

        // find first non-None item
        for i in 0..self.roots.len() {
            match &self.roots[i] {
                Some(_) => {
                    min_index = i;
                    break;
                }
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
                _ => (),
            }
            min_node = &self.roots[min_index];
        }

        let mut min_node_taken = self.roots[min_index].take().unwrap();

        // push children of the min node into heap
        for i in 0..min_node_taken.children().len() {
            let child = min_node_taken.children_mut()[i].take().unwrap();

            self._push(child);
        }

        self.size -= 1;

        // return payload the min node
        Some(min_node_taken.get_payload())
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
    fn binomial_heap_pop_1() {
        let mut bh = BinomialHeap::init(0);

        bh.push(1);
        bh.push(2);

        bh.push(3);

        let value = bh.pop();

        assert_eq!(bh.preorder(), format!("Rank 0: 1\nRank 1: 2 3\nRank 2: \n"));
        assert_eq!(value, Some(0))
    }

    #[test]
    fn binomial_heap_pop_2() {
        let mut bh = BinomialHeap::init(0);

        bh.push(1);
        bh.push(2);
        bh.push(3);

        bh.push(8);
        bh.push(9);

        bh.push(7);

        let mut value = bh.pop();

        assert_eq!(
            bh.preorder(),
            format!("Rank 0: \nRank 1: 2 3\nRank 2: 1 7 8 9\n")
        );
        assert_eq!(value, Some(0));

        value = bh.pop();
        assert_eq!(
            bh.preorder(),
            format!("Rank 0: 7\nRank 1: \nRank 2: 2 3 8 9\n")
        );
        assert_eq!(value, Some(1));

        value = bh.pop();
        assert_eq!(
            bh.preorder(),
            format!("Rank 0: \nRank 1: \nRank 2: 3 7 8 9\n")
        );
        assert_eq!(value, Some(2));

        value = bh.pop();
        assert_eq!(bh.preorder(), format!("Rank 0: 7\nRank 1: 8 9\nRank 2: \n"));
        assert_eq!(value, Some(3));

        value = bh.pop();
        assert_eq!(bh.preorder(), format!("Rank 0: \nRank 1: 8 9\nRank 2: \n"));
        assert_eq!(value, Some(7));

        value = bh.pop();
        assert_eq!(bh.preorder(), format!("Rank 0: 9\nRank 1: \nRank 2: \n"));
        assert_eq!(value, Some(8));

        value = bh.pop();
        assert_eq!(bh.preorder(), format!("Rank 0: \nRank 1: \nRank 2: \n"));
        assert_eq!(value, Some(9));

        value = bh.pop();
        assert_eq!(bh.preorder(), format!("Rank 0: \nRank 1: \nRank 2: \n"));
        assert_eq!(value, None);    
    }
}
