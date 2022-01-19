use std::io::{stdin, Read};

struct Graph {
    n: usize,
    edges: Vec<Vec<usize>>,
}

impl Graph {
    fn new(n: usize) -> Self {
        let mut edges = Vec::new();

        for _ in 0..n {
            edges.push(Vec::new());
        }

        Graph { n, edges }
    }

    fn add_edge(&mut self, a: usize, b: usize) {
        self.edges[a].push(b);
    }

    // returns a node which is member of cycle
    // if it exists
    fn find_cycle_start(&self) -> Option<(usize, Vec<usize>)> {
        fn dfs_visit(
            graph: &Graph,
            cur_node: usize,
            visited: &mut Vec<bool>,
            current_visit: &mut Vec<bool>,
            current_visit_nodes: &mut Vec<usize>,
        ) -> Option<usize> {
            let mut result = None;

            for &next_node in graph.edges[cur_node].iter() {
                // check if next node is part of current visit
                // this node is that start of the loop
                if current_visit[next_node] {
                    result = Some(next_node);
                    break;
                }

                // node has been visited before in a previous trip
                // skip it
                if visited[next_node] {
                    continue;
                }
                // visit next node
                // mark it visited
                // and add it to current visit
                // if it finds a cycle start cancel further
                // dfs and pass on the information
                else {
                    visited[next_node] = true;
                    current_visit[next_node] = true;
                    current_visit_nodes.push(next_node);

                    if let Some(cycle_start) = dfs_visit(
                        graph,
                        next_node,
                        visited,
                        current_visit,
                        current_visit_nodes,
                    ) {
                        result = Some(cycle_start);
                        break;
                    } else {
                        current_visit[next_node] = false;
                        current_visit_nodes.pop();
                    }
                }
            }

            result
        }

        let mut cycle_start = None;
        let mut visited = vec![false; self.n];
        let mut current_visit = vec![false; self.n];
        let mut current_visit_nodes = Vec::new();

        for i in 0..self.n {
            // skip already visited
            // since this is undirected graph
            // all connected nodes will be visited at once
            // any unvisited nodes will be part of a separate group
            // in the forest
            if visited[i] {
                continue;
            }

            if let Some(node) = dfs_visit(
                self,
                i,
                &mut visited,
                &mut current_visit,
                &mut current_visit_nodes,
            ) {
                cycle_start = Some(node);
                break;
            }
        }

        cycle_start.and_then(|node| Some((node, current_visit_nodes)))
    }
}

// https://cses.fi/problemset/task/1678
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let mut line = lines.next().unwrap().split_ascii_whitespace().into_iter();
    let n_cities: usize = line.next().unwrap().parse().unwrap();
    let m_flights: usize = line.next().unwrap().parse().unwrap();

    let mut graph = Graph::new(n_cities);

    for _i_conn in 0..m_flights {
        let mut line = lines.next().unwrap().split_ascii_whitespace().into_iter();
        let a_city: usize = line.next().unwrap().parse::<usize>().unwrap() - 1;
        let b_city: usize = line.next().unwrap().parse::<usize>().unwrap() - 1;

        graph.add_edge(a_city, b_city);
    }

    // use start node and visit information
    // to find cycle and len and print nodes part
    // of cycle
    if let Some((start, visit)) = graph.find_cycle_start() {
        let start_index = visit.iter().position(|&val| val == start).unwrap();
        let cycle_len = visit.len() - start_index;
        println!("{}", cycle_len + 1);

        for i in start_index..visit.len() {
            print!("{} ", visit[i] + 1);
        }
        println!("{}", start + 1);
    } else {
        println!("IMPOSSIBLE");
    }
}
