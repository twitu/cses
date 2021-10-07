use std::io::{stdin, Read};

struct SegmentTree {
    array_len: usize,
    tree: Vec<usize>,
}

impl SegmentTree {
    #[inline]
    // 1 indexed segment tree
    fn left(i: usize) -> usize {
        i * 2
    }

    #[inline]
    // 1 indexed segment tree
    fn right(i: usize) -> usize {
        i * 2 + 1
    }

    fn new(values: Vec<usize>) -> Self {
        let array_len = values.len();
        let tree_len = array_len * 2;
        let mut tree = vec![0; tree_len];

        tree[array_len..].copy_from_slice(&values);

        let mut segment_tree = SegmentTree { array_len, tree };

        segment_tree.treeify();
        segment_tree
    }

    fn treeify(&mut self) {
        for i in (1..self.array_len).into_iter().rev() {
            self.tree[i] = self.tree[SegmentTree::left(i)] + self.tree[SegmentTree::right(i)];
        }
    }

    // query for range [l,r) i.e. left inclusive, right exclusive
    // 1 indexed query
    fn find_for_range(&self, query_left: usize, query_right: usize) -> usize {
        let mut value = 0;
        let mut l = query_left + self.array_len;
        let mut r = query_right + self.array_len;

        // the algorithm works by only adding values
        // that are add odd indices of the 1 indexed
        // segment tree array
        while l < r {
            // if l is odd then it is the right child
            // of it's parent so it can be added as is
            // incrementing l brings it to the next pair
            // dividing by 2 makes it the parent of the
            // next pair
            // if l is even then it is the left child
            // of the pair and the sum of the pair can
            // be found at the parent unless the interval
            // is closed by the right border
            if l % 2 != 0 {
                value += self.tree[l];
            }

            if r % 2 != 0 {
                r -= 1;
                value += self.tree[r];
            }

            l += 1;
            l /= 2;
            r /= 2;
        }

        value
    }
}

// problem - https://cses.fi/problemset/task/1646
// ref - https://codeforces.com/blog/entry/18051
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.lines();

    let first_line: Vec<usize> = input
        .next()
        .unwrap()
        .split(' ')
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

    let tree = SegmentTree::new(numbers);

    for _ in 0..q_queries {
        let query: Vec<usize> = input
            .next()
            .unwrap()
            .split_whitespace()
            .map(|val| val.parse().unwrap())
            .collect();

        println!("{}", tree.find_for_range(query[0] - 1, query[1]));
    }
}
