use std::{
    cell::RefCell,
    cmp::min,
    collections::HashSet,
    io::{stdin, Read},
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
    // the root to which a node belongs
    root_id: Vec<usize>,
    // root is a cycle containing possibly many nodes
    // this is relevant only for nodes on the root cycle
    // and stores their relative position on it
    root_pos: Vec<usize>,
    // reverse lookup a node id from it's root id and root pos
    rev_root_pos: Vec<Vec<usize>>,
    // since root is a cycle a node can link to the root cycle
    // at one or more root positions this stores where a node
    // links to the root position and is only relevant for
    // non root-cycle nodes it should be usize::MAX otherwise
    root_link: Vec<usize>,
    root_members: Rc<RefCell<Vec<HashSet<usize>>>>,
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
        let root_id = vec![usize::MAX; size];
        let root_pos = vec![usize::MAX; size];
        let root_link = vec![usize::MAX; size];
        let root_members = Rc::new(RefCell::new(vec![]));
        let rev_root_pos = vec![];

        let ancestor_limit: usize = f64::ceil(f64::log2(size as f64)).round() as usize;
        let ancestors = vec![vec![usize::MAX; ancestor_limit]; size];

        // create tree view by taking
        // reverse links of given nodes
        for (from, &to) in next_node.iter().enumerate() {
            tree_view[to].push(from);
        }

        let tree_view = Rc::new(tree_view);

        let mut graph = FunctionalGraph {
            size,
            next_node,
            tree_view,
            root_id,
            root_pos,
            rev_root_pos,
            root_link,
            root_members,
            ancestors,
            ancestor_limit,
        };

        graph.find_roots();
        FunctionalGraph::ancestor_analysis(&mut graph);
        return graph;
    }

    fn find_roots(&mut self) {
        let mut visited = vec![false; self.size];
        let mut root_id = 0;

        for i in 0..self.size {
            if !visited[i] {
                let mut curr = i;
                let mut current_visit = HashSet::<usize>::new();

                // follow unvisited next nodes
                while !visited[curr] {
                    visited[curr] = true;
                    current_visit.insert(curr);

                    curr = self.next_node[curr];
                }

                // found loop
                if current_visit.contains(&curr) {
                    // follow nodes from curr again to find only loop nodes
                    let mut loop_visit = HashSet::<usize>::new();
                    let mut curr_loop_node = curr;
                    let mut pos = 0;
                    let loop_start = curr;
                    // create vec for looking up node based on it's pos
                    let mut lookup_rev_pos = vec![];

                    // find loop members and mark their positions
                    while !loop_visit.contains(&curr_loop_node) {
                        // add position order to the loop nodes
                        // set the root to which they below
                        // root members don't have direct ancestors
                        // so set their ancestor to usize::MA
                        self.root_id[curr_loop_node] = root_id;
                        self.root_pos[curr_loop_node] = pos;
                        lookup_rev_pos.push(curr_loop_node);

                        pos += 1;
                        loop_visit.insert(curr_loop_node);
                        curr_loop_node = self.next_node[curr_loop_node];
                    }

                    // set reverse lookup table for root system - root_id
                    self.rev_root_pos.push(lookup_rev_pos);

                    // mark remaining nodes with root id and root link
                    for &other_node in current_visit.difference(&loop_visit) {
                        self.root_id[other_node] = root_id;
                        self.root_link[other_node] = loop_start;
                    }

                    let mut root_members = self.root_members.borrow_mut();
                    root_members.push(loop_visit);
                    root_id += 1;
                }
                // found an already visited node
                // set root_id and root_link for all currently visited nodes
                // to the visited node that was encountered
                else {
                    for &current_node in &current_visit {
                        self.root_id[current_node] = self.root_id[curr];

                        // if it is non root node copy root link
                        if self.root_pos[curr] == usize::MAX {
                            self.root_link[current_node] = self.root_link[curr];
                        }
                        // otherwise use value
                        else {
                            self.root_link[current_node] = curr;
                        }
                    }
                }
            }
        }
    }

    // should only be called after finding roots
    // performs dfs from root nodes to mark information
    // in tree nodes
    fn ancestor_analysis(graph: &mut FunctionalGraph) {
        let root_cycles = graph.root_members.clone();

        for root_cycle in root_cycles.as_ref().borrow().iter() {
            for &root_node in root_cycle {
                FunctionalGraph::traverse_and_mark(root_node, usize::MAX, graph);
            }
        }
    }

    // performs a dfs from each root member and marks child nodes
    // with dephth and traversal start and end information
    // this information can later be used to determine
    // ancestor and find distance between an ancestory
    // and a child
    fn traverse_and_mark(node: usize, parent: usize, graph: &mut FunctionalGraph) {
        let tree_view = graph.tree_view.clone();

        // populate ancestor table if it is not a root node
        // a root node has usize::MAX for parent node
        if parent != usize::MAX {
            graph.ancestors[node][0] = parent;

            for i in 1..graph.ancestor_limit {
                graph.ancestors[node][i] = graph.ancestors[graph.ancestors[node][i - 1]][i - 1];

                if graph.ancestors[node][i] == usize::MAX {
                    break;
                }
            }
        }

        for &child in &tree_view[node] {
            // skip child nodes that have root position
            // because they are sibling root members
            if graph.root_pos[child] != usize::MAX {
                continue;
            }

            FunctionalGraph::traverse_and_mark(child, node, graph);
        }
    }

    // find destination when making a certain number of hops from a starting point
    fn destination(self: &FunctionalGraph, start: usize, mut hops: usize) -> usize {
        let mut curr_node = start;

        loop {
            if hops == 0 {
                break;
            }

            if hops == 1 {
                curr_node = self.next_node[curr_node];
                break;
            }

            let hop_power_limit: usize = f64::floor(f64::log2(hops as f64)).round() as usize;
            if let Some((index, _)) = self.ancestors[curr_node]
                .iter()
                .enumerate()
                // do not exceed number of given hops in power terms
                .take(min(hop_power_limit, self.ancestor_limit))
                // sort by highest ancestor first
                .rev()
                // filter for valid ancestors
                .filter(|(_, ancestor)| **ancestor != usize::MAX)
                // take highest valid ancestor
                .next()
            // jump ancestors as much as possible
            {
                curr_node = self.ancestors[curr_node][index];
                hops = hops - (2 as f64).powi(index as i32) as usize;
            }
            // reached a root node
            else {
                let cycle_size = self.root_members.as_ref().borrow()[self.root_id[curr_node]].len();
                let offset = (self.root_pos[curr_node] + hops) % cycle_size;

                curr_node = self.rev_root_pos[self.root_id[curr_node]][offset];
                hops = 0;
            }
        }

        curr_node
    }
}

// https://cses.fi/problemset/task/1750
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
        .map(|value| value.parse::<usize>().unwrap() - 1)
        .collect();

    let graph = FunctionalGraph::new(numbers);

    for _ in 0..q_queries {
        let query: Vec<usize> = input
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|val| val.parse().unwrap())
            .collect();

        let start = query[0] - 1;
        let hops = query[1];
        let dest = graph.destination(start, hops);
        println!("{}", dest + 1); // planets are 1 indexed
    }
}
