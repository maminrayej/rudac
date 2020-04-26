#[derive(Debug)]
pub struct BinomialTree<T: std::cmp::Ord> {
    rank: usize,
    children: Vec<Option<BinomialTree<T>>>,
    payload: T,
}

impl<T: std::cmp::Ord> BinomialTree<T> {
    pub fn init(payload: T) -> BinomialTree<T> {
        BinomialTree {
            rank: 0,
            children: Vec::new(),
            payload,
        }
    }

    pub fn add(&mut self, binomial_tree: BinomialTree<T>) {
        self.children.push(Some(binomial_tree));
        self.rank *= 2;
    }

    pub fn merge(
        mut binomial_tree_1: BinomialTree<T>,
        mut binomial_tree_2: BinomialTree<T>,
    ) -> BinomialTree<T> {
        if binomial_tree_1.rank() != binomial_tree_2.rank {
            panic!("Binomial tree ranks must be the same when merging");
        }

        if binomial_tree_1.payload() <= binomial_tree_2.payload() {
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

    pub fn payload(&self) -> &T {
        &self.payload
    }
}

pub struct BinomialHeap<T: std::cmp::Ord> {
    roots: Vec<Option<BinomialTree<T>>>,
    valid: Vec<bool>,
}

impl<T: std::cmp::Ord> BinomialHeap<T> {
    pub fn init(payload: T) -> BinomialHeap<T> {
        let root = Some(BinomialTree::init(payload));

        let mut roots = Vec::new();
        let mut valid = Vec::new();

        roots.push(root);
        valid.push(true);

        BinomialHeap {
            roots: roots,
            valid,
        }
    }

    pub fn push(&mut self, payload: T) {
        let mut new_node = BinomialTree::init(payload);

        // check if rank 0 exits
        if !self.valid[0] {
            self.roots[0] = Some(new_node);

            return;
        }

        let max_rank = self.roots.len();

        for i in 0..max_rank {
            match self.roots[i].take() {
                Some(node) => {
                    new_node = BinomialTree::merge(node, new_node);

                    if i == max_rank - 1 {
                        self.roots.push(Some(new_node));
                        break;
                    }
                },
                None => {
                    self.roots[i] = Some(new_node);
                    
                    break;
                }
            }
        }
    }
}
