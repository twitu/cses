use std::io::*;

struct Graph {
    n: usize,
    edges: Vec<Vec<usize>>,
}

impl Graph {
    fn new(n: usize) -> Self {
        let mut edges = Vec::new();
        for _ in 0..n {
            edges.push(Vec::new())
        }

        Graph { n, edges }
    }

    fn add_edge(&mut self, a: usize, b: usize) {
        self.edges[a].push(b);
        self.edges[b].push(a);
    }

    fn bfs(&self, start: usize) -> (usize, usize) {
        let mut queue: Vec<(usize, usize)> = Vec::new();
        queue.push((start, 0));

        let mut index: usize = 0;
        let mut visited = vec![false; self.n];

        let cur_node = loop {
            let cur_node = queue[index];
            let (cur_node_id, step_count) = cur_node;

            index += 1;
            visited[cur_node_id] = true;

            for &next_node in self.edges[cur_node_id].iter() {
                if !visited[next_node] {
                    queue.push((next_node, step_count + 1))
                }
            }

            if index == queue.len() {
                break cur_node;
            }
        };

        return cur_node;
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.lines();

    let n: usize = input.next().unwrap().parse().unwrap();
    let mut tree = Graph::new(n);

    while let Some(line) = input.next() {
        let edge: Vec<&str> = line.split(' ').collect();
        let a: usize = edge[0].parse::<usize>().unwrap() - 1;
        let b: usize = edge[1].parse::<usize>().unwrap() - 1;

        tree.add_edge(a, b);
    }

    let (first_end, _) = tree.bfs(0);
    let (_, diameter) = tree.bfs(first_end);

    println!("{}", diameter);
}
