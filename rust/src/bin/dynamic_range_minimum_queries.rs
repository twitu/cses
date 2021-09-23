use std::{
    cmp::min,
    io::{stdin, Read},
};

struct SegmentTree {
    levels: usize,
    base_index: usize,
    tree: Vec<usize>,
}

impl SegmentTree {
    fn parent(i: usize) -> usize {
        (i - 1) / 2
    }

    // 0 indexed
    fn left(i: usize) -> usize {
        i * 2 + 1
    }

    // 0 indexed
    fn right(i: usize) -> usize {
        i * 2 + 2
    }

    // gives index for flattened tree element
    // from 0 indexed level and 0 indexed segment for that level
    fn tree_index(level: usize, segment: usize) -> usize {
        let total_segments = (2 as usize).pow(level as u32) - 1;
        total_segments + segment
    }

    fn get_value(&self, level: usize, segment: usize) -> usize {
        self.tree[SegmentTree::tree_index(level, segment)]
    }

    fn new(values: Vec<usize>) -> Self {
        let levels: usize = f64::ceil(f64::log2(values.len() as f64)).round() as usize + 1;
        let base_index = (2 as usize).pow((levels - 1) as u32) - 1;
        let tree = vec![usize::MAX; (2 as usize).pow(levels as u32) - 1];

        let mut segment_tree = SegmentTree {
            levels,
            base_index,
            tree,
        };

        segment_tree.treeify(values);
        segment_tree
    }

    fn treeify(&mut self, values: Vec<usize>) {
        // copy values from original array
        for (i, &value) in (self.base_index..(self.base_index * 2 + 1)).zip(values.iter()) {
            self.tree[i] = value;
        }

        // populate all other levels
        // for this tree the parent contains the
        // the sum of left and right children
        for i in (1..(self.base_index * 2 + 1)).step_by(2).rev() {
            self.tree[SegmentTree::parent(i)] = min(self.tree[i], self.tree[i + 1]);
        }
    }

    fn update_value(&mut self, index: usize, value: usize) {
        let mut tree_index = self.base_index + index;
        self.tree[tree_index] = value;

        let mut levels = self.levels - 1;

        while levels != 0 {
            let parent = SegmentTree::parent(tree_index);
            let left_child = SegmentTree::left(parent);
            let right_child = SegmentTree::right(parent);

            self.tree[parent] = min(self.tree[left_child], self.tree[right_child]);

            tree_index = parent;
            levels -= 1;
        }
    }

    fn find_for_range(&self, left: usize, right: usize) -> usize {
        fn inner(
            tree: &SegmentTree,
            left: usize,
            right: usize,
            level: usize,
            segment: usize,
        ) -> usize {
            let current_segment_size = (2 as usize).pow((tree.levels - level - 1) as u32);

            let current_segment_range_left = segment * current_segment_size;
            let current_segment_range_right = (segment + 1) * current_segment_size - 1;

            // query range is to the left of segment range
            if current_segment_range_right < left {
                return usize::MAX;
            }

            // query range is to the right of segment range
            if right < current_segment_range_left {
                return usize::MAX;
            }

            // query range contains segment range
            if left <= current_segment_range_left && current_segment_range_right <= right {
                return tree.get_value(level, segment);
            }

            // segment range contains query range
            return min(
                inner(tree, left, right, level + 1, segment * 2),
                inner(tree, left, right, level + 1, segment * 2 + 1),
            );
        }

        return inner(self, left, right, 0, 0);
    }
}

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
        .split(' ')
        .map(|value| value.parse().unwrap())
        .collect();

    let mut tree = SegmentTree::new(numbers);

    for _ in 0..q_queries {
        let query: Vec<usize> = input
            .next()
            .unwrap()
            .split(' ')
            .map(|val| val.parse().unwrap())
            .collect();

        match query[0] {
            // update query
            1 => {
                // 0 indexed
                tree.update_value(query[1] - 1, query[2]);
            }
            // only other query is range query
            _ => {
                // 0 indexed
                println!("{}", tree.find_for_range(query[1] - 1, query[2] - 1));
            }
        }
    }
}
