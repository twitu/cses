use std::{
    cmp::min,
    collections::HashSet,
    io::{stdin, Read},
};

#[derive(Debug)]
struct Graph {
    n: usize,
    edges: Vec<Vec<Edge>>,
}

impl Graph {
    fn new(n: usize) -> Self {
        let mut edges = Vec::new();
        for _ in 0..n {
            edges.push(Vec::new())
        }

        Graph { n, edges }
    }

    fn add_edge(&mut self, start: usize, end: usize) {
        self.edges[start].push(Edge {
            start,
            end,
            capacity: 1,
        });

        self.edges[end].push(Edge {
            start: end,
            end: start,
            capacity: 0,
        })
    }

    fn change_edge(&mut self, start: usize, end: usize, change: i64) {
        self.edges[start]
            .iter_mut()
            .filter(|edge| edge.end == end && edge.allow_change_capacity(change))
            .take(1)
            .for_each(|edge| edge.change_capacity(change))
    }

    fn find_edge(&self, start: usize, end: usize) -> Option<&Edge> {
        self.edges[start].iter().find(|edge| edge.end == end)
    }

    fn bfs(&self, start: usize, end: usize) -> Option<Vec<usize>> {
        let mut queue: Vec<usize> = Vec::new();
        let mut index = 0;
        let mut visited = vec![false; self.n];
        let mut parent: Vec<usize> = vec![usize::MAX; self.n];

        queue.push(start);
        visited[start] = true;
        parent[start] = start;

        let path_to_end = loop {
            let cur_node = queue[index];

            index += 1;

            // reached end
            if cur_node == end {
                break Some(parent);
            }

            for edge in self.edges[cur_node].iter() {
                if edge.capacity > 0 && !visited[edge.end] {
                    queue.push(edge.end);
                    parent[edge.end] = cur_node;
                    visited[edge.end] = true;
                }
            }

            // processed queue but did not reach end
            if index == queue.len() {
                break None;
            }
        };

        path_to_end
    }

    fn ford_fulkerson(&mut self, start: usize, end: usize) -> u64 {
        let mut max_capacity = 0;

        while let Some(path) = self.bfs(start, end) {
            let mut min_flow = u64::MAX;
            let mut cur_node = end;

            // find min flow in the path
            while cur_node != start {
                let parent = path[cur_node];
                let edge = self.find_edge(parent, cur_node).unwrap(); // start end edge will always exist

                min_flow = min(min_flow, edge.capacity);

                cur_node = parent;
            }

            cur_node = end;
            while cur_node != start {
                let parent = path[cur_node];
                self.change_edge(parent, cur_node, -(min_flow as i64));
                self.change_edge(cur_node, parent, min_flow as i64);
                cur_node = parent;
            }

            max_capacity += min_flow;
        }

        max_capacity
    }

    fn find_reachable_nodes(&self, start: usize) -> HashSet<usize> {
        let mut queue: Vec<usize> = Vec::new();
        let mut index = 0;
        let mut visited = vec![false; self.n];

        queue.push(start);
        visited[start] = true;

        loop {
            let cur_node = queue[index];

            index += 1;

            for edge in self.edges[cur_node].iter() {
                if edge.capacity > 0 && !visited[edge.end] {
                    queue.push(edge.end);
                    visited[edge.end] = true;
                }
            }

            // processed queue but did not reach end
            if index == queue.len() {
                break;
            }
        }

        visited
            .iter()
            .enumerate()
            .filter(|(_node_id, visited)| **visited)
            .map(|(node_id, _)| node_id)
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Edge {
    capacity: u64,
    start: usize,
    end: usize,
}

impl Edge {
    fn change_capacity(&mut self, change: i64) {
        let new_capacity = (self.capacity as i64) + change;

        if new_capacity < 0 {
            self.capacity = 0;
        } else {
            self.capacity = new_capacity as u64;
        }
    }

    fn allow_change_capacity(&self, change: i64) -> bool {
        let new_capacity = (self.capacity as i64) + change;
        new_capacity >= 0
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.lines();

    let first_line: Vec<&str> = input.next().unwrap().split(' ').collect();
    let n_nodes: usize = first_line[0].parse().unwrap();
    let m_connections: usize = first_line[1].parse().unwrap();

    let mut graph = Graph::new(n_nodes);

    for _i_conn in 0..m_connections {
        let line: Vec<&str> = input.next().unwrap().split(' ').collect();
        let start: usize = line[0].parse::<usize>().unwrap() - 1;
        let end: usize = line[1].parse::<usize>().unwrap() - 1;

        graph.add_edge(start, end);
    }

    // print number of blocked roads
    let max_flow = graph.ford_fulkerson(0, n_nodes - 1);
    println!("{}", max_flow);

    let reachable_nodes = graph.find_reachable_nodes(0);

    // print the blocked roads
    for &start in reachable_nodes.iter() {
        for edge in graph.edges[start].iter() {
            if edge.capacity == 0 && !reachable_nodes.contains(&edge.end) {
                println!("{} {}", edge.start + 1, edge.end + 1); // print 1 indexed crossings
            }
        }
    }
}
