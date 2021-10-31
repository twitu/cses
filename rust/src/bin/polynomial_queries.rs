use std::{
    cmp::{max, min},
    io::{stdin, Read},
};

#[derive(Debug)]
struct SegmentTree {
    levels: usize,
    base_index: usize,
    tree: Vec<usize>,
    /// calculates the total offset added to a segment from pending updates
    total_offset: Vec<usize>,
    /// tracks the total number of updates applied to a segment
    /// this is useful when lazily updating the children
    pending_updates: Vec<usize>,
}

impl SegmentTree {
    #[inline]
    fn parent(i: usize) -> usize {
        (i - 1) / 2
    }

    #[inline]
    fn left(i: usize) -> usize {
        i * 2 + 1
    }

    #[inline]
    fn right(i: usize) -> usize {
        i * 2 + 2
    }

    // gives index for flattened tree element
    // from 0 indexed level and 0 indexed segment for that level
    fn tree_index(level: usize, segment: usize) -> usize {
        let total_segments = (2 as usize).pow(level as u32) - 1;
        total_segments + segment
    }

    fn new(values: Vec<usize>) -> Self {
        let levels: usize = f64::ceil(f64::log2(values.len() as f64)).round() as usize + 1;
        let base_index = (2 as usize).pow((levels - 1) as u32) - 1;
        let len = (2 as usize).pow(levels as u32) - 1;
        let tree = vec![0; len];
        let total_offset = vec![0; len];
        let pending_updates = vec![0; len];

        let mut segment_tree = SegmentTree {
            levels,
            base_index,
            tree,
            total_offset,
            pending_updates,
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
            self.tree[SegmentTree::parent(i)] = self.tree[i] + self.tree[i + 1];
        }
    }

    // 0 indexed update for original array elements [l, r]
    fn update_range(&mut self, left: usize, right: usize) {
        fn inner(tree: &mut SegmentTree, left: usize, right: usize, level: usize, segment: usize) {
            let current_segment_size = (2 as usize).pow((tree.levels - level - 1) as u32);

            let current_segment_range_left = segment * current_segment_size;
            let current_segment_range_right = (segment + 1) * current_segment_size - 1;

            // query range is to the left of segment range
            if current_segment_range_right < left {
                return;
            }

            // query range is to the right of segment range
            if right < current_segment_range_left {
                return;
            }

            let tree_index = SegmentTree::tree_index(level, segment);
            // query range contains segment range
            // add how much offset is being added to the segment sum
            // and increment pending operations
            // the actual update value will be lazily propagated when
            // the range is encountered in a find query
            if left <= current_segment_range_left && current_segment_range_right <= right {
                tree.total_offset[tree_index] += current_segment_range_left - left;
                tree.pending_updates[tree_index] += 1;
                return;
            }

            // segment range contains query range
            // update parent because we know the total value that
            // will be added to parent node
            {
                let range_len = min(current_segment_range_right, right)
                    - max(current_segment_range_left, left)
                    + 1;
                tree.tree[tree_index] += range_len * (range_len + 1) / 2
                    + (current_segment_range_left - min(current_segment_range_left, left))
                        * range_len;

                inner(tree, left, right, level + 1, segment * 2);
                inner(tree, left, right, level + 1, segment * 2 + 1);
            }
        }

        inner(self, left, right, 0, 0);
    }

    // 0 indexed [l, r] range from the original array
    fn find_range_sum(&mut self, left: usize, right: usize) -> usize {
        #[inline]
        fn update_value(segment_size: usize, offset: usize, number_of_updates: usize) -> usize {
            // add indepent segment sum for each update
            (segment_size * (segment_size + 1) / 2) * number_of_updates +
            // offset each element of sum
            (offset * segment_size)
        }

        fn inner(
            tree: &mut SegmentTree,
            left: usize,
            right: usize,
            level: usize,
            segment: usize,
        ) -> usize {
            let current_segment_size = (2 as usize).pow((tree.levels - level - 1) as u32);

            let current_segment_range_left = segment * current_segment_size;
            let current_segment_range_right = (segment + 1) * current_segment_size - 1;

            let tree_index = SegmentTree::tree_index(level, segment);

            // query range is to the left of segment range
            if current_segment_range_right < left {
                return 0;
            }

            // query range is to the right of segment range
            if right < current_segment_range_left {
                return 0;
            }

            // segment range contains query range
            // or is equal to query range
            // recursion will continue futher
            // apply updates to currect node
            tree.tree[tree_index] += update_value(
                current_segment_size,
                tree.total_offset[tree_index],
                tree.pending_updates[tree_index],
            );

            // query range contains segment range
            if left <= current_segment_range_left && current_segment_range_right <= right {
                return tree.tree[tree_index];
            }

            // segment range contains query range
            {
                // update left child
                let left_index = SegmentTree::left(tree_index);
                tree.total_offset[left_index] += tree.total_offset[tree_index];
                tree.pending_updates[left_index] += tree.pending_updates[tree_index];

                // update right child
                let right_index = SegmentTree::right(tree_index);
                tree.total_offset[right_index] += tree.total_offset[tree_index]
                    + current_segment_size * tree.pending_updates[tree_index];
                tree.pending_updates[right_index] += tree.pending_updates[tree_index];

                // updates completed
                tree.total_offset[tree_index] = 0;
                tree.pending_updates[tree_index] = 0;

                return inner(tree, left, right, level + 1, segment * 2)
                    + inner(tree, left, right, level + 1, segment * 2 + 1);
            }
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
        .split_ascii_whitespace()
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
            .split_ascii_whitespace()
            .map(|val| val.parse().unwrap())
            .collect();

        match query[0] {
            // update range query [l, r] 1 indexed
            1 => {
                tree.update_range(query[1] - 1, query[2] - 1);
            }
            // find range sum query [l, r] 1 indexed
            _ => {
                // println!("{}", tree.tree[0]);
                println!("{}", tree.find_range_sum(query[1] - 1, query[2] - 1));
            }
        }

        println!("{:?}", tree.tree);
    }
}
