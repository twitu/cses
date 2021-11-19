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
    // since root is a cycle a node can link to the root cycle
    // at one or more root positions this stores where a node
    // links to the root position and is only relevant for
    // non root-cycle nodes it should be usize::MAX otherwise
    root_link: Vec<usize>,
    root_members: Rc<RefCell<Vec<HashSet<usize>>>>,
    // position at start of euler traversal
    traverse_start: Vec<usize>,
    // position at end of euler traversal
    traverse_end: Vec<usize>,
    // depth of a node from it's corresponding root cycle
    // depth of nodes in a root cycle is 0
    depth: Vec<usize>,
}

impl FunctionalGraph {
    fn new(next_node: Vec<usize>) -> Self {
        let size = next_node.len();
        let mut tree_view = vec![vec![]; size];
        let root_id = vec![usize::MAX; size];
        let root_pos = vec![usize::MAX; size];
        let root_link = vec![usize::MAX; size];
        let root_members = Rc::new(RefCell::new(vec![]));
        let traverse_start = vec![usize::MAX; size];
        let traverse_end = vec![usize::MAX; size];
        let depth = vec![usize::MAX; size];

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
            root_link,
            root_members,
            traverse_start,
            traverse_end,
            depth,
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

                    // find loop members and mark their positions
                    while !loop_visit.contains(&curr_loop_node) {
                        // add position order to the loop nodes
                        // set the root to which they below
                        // set their depth to be 0
                        self.root_id[curr_loop_node] = root_id;
                        self.root_pos[curr_loop_node] = pos;
                        self.depth[curr_loop_node] = 0;

                        pos += 1;
                        loop_visit.insert(curr_loop_node);
                        curr_loop_node = self.next_node[curr_loop_node];
                    }

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
    fn ancestor_analysis(graph: &mut FunctionalGraph) {
        let mut counter = 0;
        let root_cycles = graph.root_members.clone();

        for root_cycle in root_cycles.borrow().iter() {
            for &root_node in root_cycle {
                counter = FunctionalGraph::traverse_and_mark(counter, 0, root_node, graph);
            }
        }
    }

    // performs a dfs from each root member and marks child nodes
    // with dephth and traversal start and end information
    // this information can later be used to determine
    // ancestor and find distance between an ancestory
    // and a child
    fn traverse_and_mark(
        mut counter: usize,
        depth: usize,
        node: usize,
        graph: &mut FunctionalGraph,
    ) -> usize {
        graph.depth[node] = depth;
        graph.traverse_start[node] = counter;
        counter += 1;
        let tree_view = graph.tree_view.clone();

        for &child in &tree_view[node] {
            // sibling root members are skipped because
            // all root nodes had depth set to 0
            if graph.depth[child] == usize::MAX {
                counter = FunctionalGraph::traverse_and_mark(counter, depth + 1, child, graph);
            }
        }

        graph.traverse_end[node] = counter;
        counter + 1
    }

    // checks if a is ancestor of b
    // should only be called after ancestory analsyis
    // Note: a node is an ancestor of itself
    fn is_ancestor(self: &FunctionalGraph, a: usize, b: usize) -> bool {
        // a started traversal before b
        self.traverse_start[a] <= self.traverse_start[b] &&
        // a ended traversal after b
        self.traverse_end[a] >= self.traverse_end[b]
    }

    // distance from a to b
    // when both nodes are part of same
    // root system and both are root nodes
    fn root_node_dist(self: &FunctionalGraph, a: usize, b: usize) -> usize {
        let root_cycle_len = self.root_members.borrow()[self.root_id[a]].len();
        min(
            self.root_pos[b] - self.root_pos[a],
            self.root_pos[b] - self.root_pos[a] + root_cycle_len,
        )
    }

    // find travel distance from a to b
    // usize::MAX is returned when there is no path
    fn travel_distance(self: &FunctionalGraph, start: usize, end: usize) -> usize {
        let mut ans = usize::MAX;

        // nodes are part of same root system
        if self.root_id[start] == self.root_id[end] {
            let a_is_root = self.root_link[start] == usize::MAX;
            let b_is_root = self.root_link[end] == usize::MAX;

            match (a_is_root, b_is_root) {
                (true, true) => {
                    // both are root nodes in same root system
                    ans = self.root_node_dist(start, end);
                }
                (true, false) => {
                    // a is root node and the b is not
                    // not possible to reach b
                }
                (false, true) => {
                    // b is root node and the a is not
                    let a_root_link = self.root_link[start];
                    let a_depth = self.depth[start];

                    ans = a_depth + self.root_node_dist(a_root_link, end)
                }
                (false, false) => {
                    // nodes are on same root system
                    // and are not root nodes
                    if self.is_ancestor(end, start) {
                        ans = self.depth[start] - self.depth[end]
                    }
                }
            }
        }

        ans
    }
}

// https://cses.fi/problemset/task/1160
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
        let end = query[1] - 1;
        let dist = graph.travel_distance(start, end);

        if dist != usize::MAX {
            println!("{}", dist);
        } else {
            println!("-1");
        }
    }
}
