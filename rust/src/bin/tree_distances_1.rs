use std::{cmp::max, io::*};
use std::{collections::HashMap, usize};

struct Graph {
    _n: usize,
    edges: Vec<Vec<usize>>,
}

impl Graph {
    fn new(n: usize) -> Self {
        let mut edges = Vec::new();
        for _ in 0..n {
            edges.push(Vec::new())
        }

        Graph { _n: n, edges }
    }

    fn add_edge(&mut self, a: usize, b: usize) {
        self.edges[a].push(b);
        self.edges[b].push(a);
    }

    fn fill_max_height(&self, v: usize, parent: usize, meta_data: &mut Vec<NodeMetaData>) {
        meta_data[v].max_height = 0; // mark node as visited

        let mut max_height: usize = 0;
        for &next_node in self.edges[v].iter() {
            // skip parent
            if next_node == parent {
                continue;
            }

            if meta_data[next_node].max_height == usize::MAX {
                self.fill_max_height(next_node, v, meta_data);
            }

            max_height = max(max_height, meta_data[next_node].max_height + 1)
        }

        meta_data[v].max_height = max_height;
    }

    fn fill_max_distance(&self, v: usize, parent: usize, meta_data: &mut Vec<NodeMetaData>) {
        // Take top two maximum distances
        let mut max_heights: Vec<(usize, usize)> = self.edges[v]
            .iter()
            .filter(|&&child| child != parent)
            .map(|&child| (meta_data[child].max_height + 1, child))
            .collect();
        max_heights.sort();
        max_heights.reverse();

        // populate relaxed child link metadata entries
        for &relax_child in self.edges[v].iter() {
            // skip parent when calculating relaxed child
            if relax_child == parent {
                continue;
            }

            // get max height from the max_heights vector after skipping current relaxed child
            let mut max_height = 0;
            if let Some((max_value, _)) = max_heights.iter().find(|(_, i)| *i != relax_child) {
                max_height = *max_value;
            }

            // if node is not root consider relaxed distance from parent
            if v != 0 {
                max_height = max(
                    max_height,
                    *meta_data[parent].relax_child_link.get(&v).unwrap() + 1,
                )
            }

            meta_data[v]
                .relax_child_link
                .insert(relax_child, max_height);
        }

        // fill self max_distance
        // base case for root
        if v == 0 {
            meta_data[v].max_distance = meta_data[v].max_height;

        // for node max distance is max of all child max heights and
        // parent max_distance with this child edge relaxed
        } else {
            // unwrap because this should be calculated from before
            let &parent_distance = meta_data[parent].relax_child_link.get(&v).unwrap();
            meta_data[v].max_distance = max(meta_data[v].max_height, parent_distance + 1);
        }

        // iterate over children and recursively compute their max distance
        for &child in self.edges[v].iter() {
            // only fill max distance if it hasn't been filled before
            if meta_data[child].max_distance == usize::MAX {
                self.fill_max_distance(child, v, meta_data)
            }
        }
    }
}

#[derive(Debug)]
struct NodeMetaData {
    max_height: usize,
    max_distance: usize,
    relax_child_link: HashMap<usize, usize>,
}

impl NodeMetaData {
    fn new() -> Self {
        NodeMetaData {
            max_height: usize::MAX,
            max_distance: usize::MAX,
            relax_child_link: HashMap::new(),
        }
    }

    fn new_vec(n: usize) -> Vec<Self> {
        let mut meta_data = Vec::new();

        for _ in 0..n {
            meta_data.push(NodeMetaData::new())
        }

        meta_data
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.lines();

    let n: usize = input.next().unwrap().parse().unwrap();
    let mut tree = Graph::new(n);
    let mut meta_data = NodeMetaData::new_vec(n);

    while let Some(line) = input.next() {
        let edge: Vec<&str> = line.split(' ').collect();
        let a: usize = edge[0].parse::<usize>().unwrap() - 1;
        let b: usize = edge[1].parse::<usize>().unwrap() - 1;

        tree.add_edge(a, b);
    }

    tree.fill_max_height(0, 0, &mut meta_data);
    tree.fill_max_distance(0, 0, &mut meta_data);

    for i in 0..n {
        print!("{} ", meta_data[i].max_distance);
    }
}
