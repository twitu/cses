use std::io::{stdin, Read};

struct SegmentTree {
    base_index: usize,
    tree: Vec<usize>,
}

impl SegmentTree {
    fn parent(i: usize) -> usize {
        return (i - 1) / 2;
    }

    fn left(i: usize) -> usize {
        return i * 2 + 1;
    }

    fn right(i: usize) -> usize {
        return i * 2 + 2;
    }

    fn sibling(i: usize) -> usize {
        if i % 2 == 0 {
            return i - 1;
        } else {
            return i + 1;
        }
    }

    fn original_index(&self, tree_index: usize) -> usize {
        return tree_index - self.base_index + 1;
    }

    fn comparison(&self, i: usize, j: usize) -> bool {
        return self.tree[i] > self.tree[j];
    }

    fn new(values: Vec<usize>) -> Self {
        let levels: usize = f64::ceil(f64::log2(values.len() as f64)).round() as usize + 1;
        let base_index = (2 as usize).pow((levels - 1) as u32) - 1;
        let tree = vec![0; (2 as usize).pow(levels as u32) - 1];

        let mut segment_tree = SegmentTree { base_index, tree };

        segment_tree.treeify(values);
        segment_tree
    }

    fn treeify(&mut self, values: Vec<usize>) {
        for (i, &value) in (self.base_index..(self.base_index * 2 + 1)).zip(values.iter()) {
            self.tree[i] = value;
        }

        for i in (1..(self.base_index * 2 + 1)).step_by(2).rev() {
            if self.comparison(i, i + 1) {
                self.tree[SegmentTree::parent(i)] = self.tree[i];
            } else {
                self.tree[SegmentTree::parent(i)] = self.tree[i + 1];
            }
        }
    }

    fn search(&self, value: usize) -> Option<(usize, usize)> {
        let mut index = 0;
        let mut result: Option<(usize, usize)> = None;

        if value <= self.tree[index] {
            while index < self.base_index {
                let left = SegmentTree::left(index);
                let right = SegmentTree::right(index);

                if value <= self.tree[left] {
                    index = left;
                    continue;
                }

                if value <= self.tree[right] {
                    index = right;
                    continue;
                }
            }

            result = Some((index, self.tree[index]));
        }

        result
    }

    fn update_value(&mut self, index: usize, value: usize) {
        let mut tree_index = index;
        self.tree[tree_index] = value;

        while tree_index > 0 {
            let sibling_index = SegmentTree::sibling(tree_index);
            let parent_index = SegmentTree::parent(tree_index);

            if self.comparison(tree_index, sibling_index) {
                self.tree[parent_index] = self.tree[tree_index];
            } else {
                self.tree[parent_index] = self.tree[sibling_index];
            }

            tree_index = parent_index;
        }
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.lines();

    let first_line: Vec<&str> = input.next().unwrap().split(' ').collect();
    let _n_hotels: usize = first_line[0].parse().unwrap();
    let _m_groups: usize = first_line[1].parse().unwrap();

    let hotel_capacity: Vec<usize> = input
        .next()
        .unwrap()
        .split(' ')
        .map(|value| value.parse().unwrap())
        .collect();
    let group_rooms: Vec<usize> = input
        .next()
        .unwrap()
        .split(' ')
        .map(|value| value.parse().unwrap())
        .collect();

    let mut tree = SegmentTree::new(hotel_capacity);

    for &room in group_rooms.iter() {
        if let Some((tree_index, current_rooms)) = tree.search(room) {
            tree.update_value(tree_index, current_rooms - room);
            print!("{} ", tree.original_index(tree_index));
        } else {
            print!("0 ");
        }
    }
}
