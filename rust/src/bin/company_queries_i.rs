use std::{
    cmp::min,
    io::{stdin, Read},
    iter,
    rc::Rc,
};

#[derive(Debug)]
struct FunctionalGraph {
    size: usize,
    // for a functional graph each node has only one outward edge
    next_node: Vec<usize>,
    // inverted view of functional graph is a tree with multiple
    // root nodes
    tree_view: Rc<Vec<Vec<usize>>>,
    // find and store 1st, 2nd, 4th ... ancestors and so on
    // usize::MAX indicates no ancestor
    ancestors: Vec<Vec<usize>>,
    // max ancestors to store for each node
    ancestor_limit: usize,
}

impl FunctionalGraph {
    fn new(next_node: Vec<usize>) -> Self {
        let size = next_node.len();
        let mut tree_view = vec![vec![]; size];

        let ancestor_limit: usize = f64::ceil(f64::log2(size as f64)).round() as usize;
        let ancestors = vec![vec![usize::MAX; ancestor_limit]; size];

        // create tree view by taking
        // reverse links of given nodes
        for (from, &to) in next_node.iter().enumerate() {
            // skip for dummy next value
            if to == usize::MAX {
                continue;
            }

            tree_view[to].push(from);
        }

        let tree_view = Rc::new(tree_view);

        let mut graph = FunctionalGraph {
            size,
            next_node,
            tree_view,
            ancestors,
            ancestor_limit,
        };

        // start ancestor analysis from root node which is given as 0
        graph.ancestor_analysis(0, usize::MAX);
        return graph;
    }

    // performs a dfs from the root and fill ancestor jump table
    fn ancestor_analysis(&mut self, node: usize, parent: usize) {
        // populate ancestor table if it is not a root node
        // a root node has usize::MAX for parent node
        if parent != usize::MAX {
            self.ancestors[node][0] = parent;

            for i in 1..self.ancestor_limit {
                self.ancestors[node][i] = self.ancestors[self.ancestors[node][i - 1]][i - 1];

                if self.ancestors[node][i] == usize::MAX {
                    break;
                }
            }
        }

        let children = self.tree_view.clone();
        for &child in &children[node] {
            self.ancestor_analysis(child, node);
        }
    }

    // find destination when making a certain number of hops from a starting point
    fn destination(self: &FunctionalGraph, start: usize, mut hops: usize) -> usize {
        let mut curr_node = start;

        loop {
            // break if crossed ancestor bounds
            if curr_node == usize::MAX {
                break;
            }

            if hops == 0 {
                break;
            }

            if hops == 1 {
                curr_node = self.ancestors[curr_node][0];
                break;
            }

            let max_hop: usize = f64::floor(f64::log2(hops as f64)).round() as usize;
            // handle corner case where hop is exactly the ancestor limit
            let hop = min(max_hop, self.ancestor_limit - 1);

            curr_node = self.ancestors[curr_node][hop];
            hops = hops - (2 as f64).powi(hop as i32) as usize;
        }

        curr_node
    }
}

// https://cses.fi/problemset/task/1687
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
    let _n_employees: usize = first_line[0];
    let q_queries: usize = first_line[1];

    let parent_list: Vec<usize> = iter::once(usize::MAX) // no parent for first node
        .chain(
            input
                .next()
                .unwrap()
                .split(' ')
                .map(|value| value.parse::<usize>().unwrap() - 1),
        )
        .collect();

    let graph = FunctionalGraph::new(parent_list);

    for _ in 0..q_queries {
        let query: Vec<usize> = input
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|val| val.parse().unwrap())
            .collect();

        let employee = query[0] - 1;
        let hops = query[1];
        let ans = graph.destination(employee, hops);

        if ans == usize::MAX {
            println!("-1");
        } else {
            println!("{}", ans + 1); // graph is 0 indexed
        }
    }
}
