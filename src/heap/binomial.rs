#[derive(Debug)]
pub struct BinomialTree<T: std::cmp::Ord> {
    rank: usize,
    children: Vec<BinomialTree<T>>,
    payload: T
}

impl<T: std::cmp::Ord> BinomialTree<T> {

    pub fn init(payload: T) -> BinomialTree<T> {
        BinomialTree {
            rank: 0,
            children: Vec::new(),
            payload
        }
    }

    pub fn add(&mut self, binomial_tree: BinomialTree<T>) {
        self.children.push(binomial_tree);
        self.rank *= 2;
    }

    pub fn merge(mut binomial_tree_1: BinomialTree<T>, mut binomial_tree_2: BinomialTree<T>) -> BinomialTree<T> {

        if binomial_tree_1.rank() != binomial_tree_2.rank {
            panic!("Binomial tree ranks must be the same when merging");
        }

        if binomial_tree_1.payload() <= binomial_tree_2.payload() {
            binomial_tree_1.add(binomial_tree_2);
            return binomial_tree_1;
        }
        else {
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