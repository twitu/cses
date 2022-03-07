use std::io::{stdin, Read};

#[derive(Debug)]
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

    fn add_edge(&mut self, start: usize, end: usize) {
        self.edges[start].push(end);
        self.edges[end].push(start);
    }

    fn find_centroid(&self) -> usize {
        let mut visited = vec![false; self.n];
        self.find_centroid_dfs(0, &mut visited).unwrap()
    }

    fn find_centroid_dfs(&self, start: usize, visited: &mut Vec<bool>) -> Result<usize, usize> {
        let mut total_visited_nodes = 1;  // start with self visit
        let mut sub_trees_size: Vec<usize> = vec![];
        for &next_node in &self.edges[start] {
            if !visited[next_node] {
                visited[next_node] = true;
                match self.find_centroid_dfs(next_node, visited) {
                    Ok(centroid) => return Ok(centroid),
                    Err(node_count) => {
                        total_visited_nodes += node_count;
                        sub_trees_size.push(node_count)
                    }
                }
            }
        }

        let cutoff = self.n / 2;
        if sub_trees_size.iter().all(|size| size <= &cutoff) && total_visited_nodes > cutoff {
            Ok(start)
        } else {
            Err(total_visited_nodes)
        }
    }
}

// https://cses.fi/problemset/task/2079
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.lines();

    let n_nodes = input.next().unwrap().parse::<usize>().unwrap();

    let mut g = Graph::new(n_nodes);

    input.take(n_nodes - 1).for_each(|line| {
        let mut edge_line = line.split_ascii_whitespace().take(2);
        let start = edge_line.next().unwrap().parse::<usize>().unwrap();
        let end = edge_line.next().unwrap().parse::<usize>().unwrap();
        g.add_edge(start - 1, end - 1);
    });

    println!("{}", g.find_centroid() + 1);
}
