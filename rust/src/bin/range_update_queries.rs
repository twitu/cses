use std::io::{stdin, Read};

struct SegmentTree {
    array_len: usize,
    tree: Vec<usize>,
}

impl SegmentTree {
    #[inline]
    fn parent(i: usize) -> usize {
        i / 2
    }

    fn new(values: Vec<usize>) -> Self {
        let array_len = values.len();
        let tree_len = array_len * 2;
        let mut tree = vec![0; tree_len];

        tree[array_len..].copy_from_slice(&values);

        SegmentTree { array_len, tree }
    }

    // 0 indexed find on original array element
    fn find_value(&self, index: usize) -> usize {
        let mut tree_index = self.array_len + index;
        let mut value = self.tree[tree_index];

        tree_index = SegmentTree::parent(tree_index);

        while tree_index != 0 {
            value += self.tree[tree_index];
            tree_index = SegmentTree::parent(tree_index);
        }

        value
    }

    // update query range [l, r) exclusive
    // where l, r are 0 indexed
    fn update_range(&mut self, query_left: usize, query_right: usize, value: usize) {
        let mut l = query_left + self.array_len;
        let mut r = query_right + self.array_len;

        // the algorithm works by only changing values
        // that are add odd indices of the 1 indexed
        // segment tree array
        while l < r {
            // if l is odd then it is the right child
            // of it's parent so it can be changed as is
            // incrementing l brings it to the next pair
            // dividing by 2 makes it the parent of the
            // next pair
            // if l is even then it is the left child
            // of the pair and the range of the pair can
            // be updated at the parent unless the interval
            // is closed by the right border
            if l % 2 != 0 {
                self.tree[l] += value;
                l += 1;
            }

            if r % 2 != 0 {
                r -= 1;
                self.tree[r] += value;
            }

            l /= 2;
            r /= 2;
        }
    }
}

// https://cses.fi/problemset/task/1651
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.lines();

    let first_line: Vec<usize> = input
        .next()
        .unwrap()
        .split_whitespace()
        .map(|val| val.parse().unwrap())
        .collect();
    let _n_numbers: usize = first_line[0];
    let q_queries: usize = first_line[1];

    let numbers: Vec<usize> = input
        .next()
        .unwrap()
        .split_whitespace()
        .map(|value| value.parse().unwrap())
        .collect();

    let mut tree = SegmentTree::new(numbers);

    for _ in 0..q_queries {
        let query: Vec<usize> = input
            .next()
            .unwrap()
            .split_whitespace()
            .map(|val| val.parse().unwrap())
            .collect();

        match query[0] {
            // update range query
            1 => {
                // 1 indexed inclusive range update
                tree.update_range(query[1] - 1, query[2], query[3]);
            }
            // find value query
            _ => {
                // find value at 1 indexed position of original array
                println!("{}", tree.find_value(query[1] - 1));
            }
        }
    }
}
