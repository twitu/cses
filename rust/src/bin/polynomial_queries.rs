use std::{
    cmp::{max, min},
    io::{stdin, Read},
};

#[derive(Debug)]
struct SegmentTree {
    levels: usize,
    base_index: usize,
    /// size of the original array
    array_size: usize,
    /// 1 indexed array storing the binary tree
    /// aggregating information from child nodes in parent node
    tree: Vec<usize>,
    /// calculates the total offset added to a segment from pending updates
    total_offset: Vec<usize>,
    /// tracks the total number of updates applied to a segment
    /// this is useful when lazily updating the children
    pending_updates: Vec<usize>,
}

impl SegmentTree {
    #[inline]
    fn left(i: usize) -> usize {
        i * 2
    }

    #[inline]
    fn right(i: usize) -> usize {
        i * 2 + 1
    }

    fn new(values: Vec<usize>) -> Self {
        let levels: usize = f64::ceil(f64::log2(values.len() as f64)).round() as usize + 1;
        let base_index = (2 as usize).pow((levels - 1) as u32);
        let len = (2 as usize).pow(levels as u32);
        let tree = vec![0; len];
        let total_offset = vec![0; len];
        let pending_updates = vec![0; len];

        let mut segment_tree = SegmentTree {
            levels,
            base_index,
            array_size: values.len(),
            tree,
            total_offset,
            pending_updates,
        };

        segment_tree.treeify(values);
        segment_tree
    }

    fn treeify(&mut self, values: Vec<usize>) {
        // copy values from original array
        for (i, &value) in (self.base_index..(self.base_index * 2)).zip(values.iter()) {
            self.tree[i] = value;
        }

        // populate all other levels
        // for this tree the parent contains the
        // the sum of left and right children
        for i in (1..(self.base_index)).rev() {
            self.tree[i] = self.tree[SegmentTree::left(i)] + self.tree[SegmentTree::right(i)];
        }
    }

    // 0 indexed update for original array elements [l, r]
    fn update_range(&mut self, left: usize, right: usize) {
        fn inner(
            tree: &mut SegmentTree,
            update_left: usize,
            update_right: usize,
            tree_index: usize,
            seg_left: usize,
            seg_right: usize,
        ) {
            // query range is to the left of segment range
            // query range is to the right of segment range
            if seg_right < update_left || update_right < seg_left {
                return;
            }

            let current_segment_size = seg_right - seg_left + 1;

            // query range contains segment range
            // add how much offset is being added to the segment sum
            // and increment pending operations
            // the actual update value will be lazily propagated when
            // the range is encountered in a find query
            if update_left <= seg_left && seg_right <= update_right {
                tree.total_offset[tree_index] += seg_left - update_left;
                tree.pending_updates[tree_index] += 1;
                return;
            }

            // segment range contains query range
            // update parent because we know the total value that
            // will be added to parent node
            {
                let range_len = min(seg_right, update_right) - max(seg_left, update_left) + 1;
                tree.tree[tree_index] += range_len * (range_len + 1) / 2
                    + (seg_left - min(seg_left, update_left)) * range_len;

                let mid = seg_left + current_segment_size / 2;
                inner(
                    tree,
                    update_left,
                    update_right,
                    SegmentTree::left(tree_index),
                    seg_left,
                    mid - 1,
                );
                inner(
                    tree,
                    update_left,
                    update_right,
                    SegmentTree::right(tree_index),
                    mid,
                    seg_right,
                );
            }
        }

        let last_level_size = (2 as usize).pow((self.levels - 1) as u32);
        inner(self, left, right, 1, 0, last_level_size - 1);
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
            query_left: usize,
            query_right: usize,
            tree_index: usize,
            seg_left: usize,
            seg_right: usize,
        ) -> usize {
            // query range is to the left of segment range
            // query range is to the right of segment range
            if seg_right < query_left || query_right < seg_left {
                return 0;
            }

            let current_segment_size = seg_right - seg_left + 1;

            let update_value = update_value(
                current_segment_size,
                tree.total_offset[tree_index],
                tree.pending_updates[tree_index],
            );

            // query range contains segment range
            // return value along with update
            // update does not need to be applied as it
            // will be applied if current nodes child nodes are visited
            // in later queries
            if query_left <= seg_left && seg_right <= query_right {
                return tree.tree[tree_index] + update_value;
            }

            // segment range contains query range
            // apply updates to currect node
            tree.tree[tree_index] += update_value;

            // segment range contains query range
            {
                // update left child
                let left_index = SegmentTree::left(tree_index);
                tree.pending_updates[left_index] += tree.pending_updates[tree_index];
                tree.total_offset[left_index] += tree.total_offset[tree_index];

                // update right child
                let right_index = SegmentTree::right(tree_index);
                tree.pending_updates[right_index] += tree.pending_updates[tree_index];
                tree.total_offset[right_index] += tree.total_offset[tree_index]
                    + (current_segment_size / 2) * tree.pending_updates[tree_index];

                // updates completed
                tree.total_offset[tree_index] = 0;
                tree.pending_updates[tree_index] = 0;

                let mid = seg_left + current_segment_size / 2;
                return inner(
                    tree,
                    query_left,
                    query_right,
                    SegmentTree::left(tree_index),
                    seg_left,
                    mid - 1,
                ) + inner(
                    tree,
                    query_left,
                    query_right,
                    SegmentTree::right(tree_index),
                    mid,
                    seg_right,
                );
            }
        }

        let last_level_size = (2 as usize).pow((self.levels - 1) as u32);
        return inner(self, left, right, 1, 0, last_level_size - 1);
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
                println!("{}", tree.find_range_sum(query[1] - 1, query[2] - 1));
            }
        }
    }
}
