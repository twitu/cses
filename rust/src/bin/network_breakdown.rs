use std::{collections::HashSet, io::*, usize};

struct Graph {
    n: usize,
    component_size: Vec<usize>,
    components: usize,
    edges: Vec<Vec<usize>>,
}

impl Graph {
    fn new(n: usize) -> Self {
        let mut edges = Vec::new();
        let mut component_size = Vec::new();

        for _ in 0..n {
            edges.push(Vec::new());
            component_size.push(1);
        }

        Graph {
            n,
            component_size,
            components: 0,
            edges,
        }
    }

    fn add_edge(&mut self, a: usize, b: usize) {
        self.edges[a].push(b);
        self.edges[b].push(a);
    }

    fn bfs(&self, start: usize) -> HashSet<usize> {
        let mut queue: Vec<usize> = Vec::new();
        queue.push(start);

        let mut index: usize = 0;
        let mut visited: HashSet<usize> = HashSet::new();

        loop {
            let cur_node = queue[index];

            index += 1;
            visited.insert(cur_node);

            for &next_node in self.edges[cur_node].iter() {
                if next_node != usize::MAX && !visited.contains(&next_node) {
                    queue.push(next_node);
                }
            }

            if index == queue.len() {
                break;
            }
        }

        return visited;
    }

    fn create_components(&mut self) {
        let mut visited = vec![false; self.n];

        for i in 0..self.n {
            if !visited[i] {
                let component = self.bfs(i);
                for &node in component.iter() {
                    self.component_size[node] = component.len();
                    visited[node] = true;
                }

                self.components += 1;
            }
        }
    }

    fn remove_edge(&mut self, a: usize, b: usize) {
        self.edges[a]
            .iter_mut()
            .filter(|next_node| **next_node == b)
            .for_each(|next_node| {
                *next_node = usize::MAX;
            });
        self.edges[b]
            .iter_mut()
            .filter(|next_node| **next_node == a)
            .for_each(|next_node| {
                *next_node = usize::MAX;
            });
    }

    fn disconnect_component(&mut self, a: usize, b: usize) {
        self.remove_edge(a, b);

        let a_visited = self.bfs(a);

        // no change in components
        if a_visited.len() == self.component_size[a] {
            return;
        }

        for &node in a_visited.iter() {
            self.component_size[node] = a_visited.len();
        }

        let b_visited = self.bfs(b);

        for &node in b_visited.iter() {
            self.component_size[node] = b_visited.len();
        }

        self.components += 1;
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.lines();

    let first_line: Vec<&str> = input.next().unwrap().split(' ').collect();
    let n_nodes: usize = first_line[0].parse().unwrap();
    let m_connections: usize = first_line[1].parse().unwrap();
    let k_breaks: usize = first_line[2].parse().unwrap();

    let mut graph = Graph::new(n_nodes);

    for _i_conn in 0..m_connections {
        let line: Vec<&str> = input.next().unwrap().split(' ').collect();
        let start: usize = line[0].parse::<usize>().unwrap() - 1;
        let end: usize = line[1].parse::<usize>().unwrap() - 1;

        graph.add_edge(start, end);
    }

    graph.create_components();

    for _i_breaks in 0..k_breaks {
        let line: Vec<&str> = input.next().unwrap().split(' ').collect();
        let start: usize = line[0].parse::<usize>().unwrap() - 1;
        let end: usize = line[1].parse::<usize>().unwrap() - 1;

        graph.disconnect_component(start, end);
        print!("{} ", graph.components);
    }
}
