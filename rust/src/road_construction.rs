use std::cmp::{max, Ordering};
use std::io::*;

#[derive(Debug)]
struct UnionFind {
    n: usize,
    components: usize,
    parent: Vec<usize>,
    rank: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let rank: Vec<usize> = (0..n).map(|_| 0).collect();
        let size: Vec<usize> = (0..n).map(|_| 1).collect();
        let parent: Vec<usize> = (0..n).collect();

        UnionFind {
            n,
            components: n,
            parent,
            rank,
            size,
        }
    }

    fn find_root(&mut self, cur: usize) -> usize {
        let mut cur = cur;

        loop {
            if self.parent[cur] == cur {
                break cur;
            }

            self.parent[cur] = self.parent[self.parent[cur]];
            cur = self.parent[cur];
        }
    }

    fn create_union(&mut self, a: usize, b: usize) {
        let root_a = self.find_root(a);
        let root_b = self.find_root(b);

        if root_a == root_b {
            return;
        }

        match self.rank[root_a].cmp(&self.rank[root_b]) {
            std::cmp::Ordering::Less => {
                self.parent[root_a] = root_b;
                self.size[root_b] += self.size[root_a];
            }
            std::cmp::Ordering::Greater => {
                self.parent[root_b] = root_a;
                self.size[root_a] += self.size[root_b];
            }
            std::cmp::Ordering::Equal => {
                self.parent[root_a] = root_b;
                self.size[root_b] += self.size[root_a];
                self.rank[root_b] += 1;
            }
        }

        self.components -= 1;
    }

    fn component_size(&mut self, i: usize) -> usize {
        let root_i = self.find_root(i);
        self.size[root_i]
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.lines();

    let first_line: Vec<&str> = input.next().unwrap().split(" ").collect();
    let n_cities: usize = first_line[0].parse().unwrap();
    let m_roads: usize = first_line[1].parse().unwrap();

    let mut uf = UnionFind::new(n_cities);
    let mut largest: usize = 1;

    for _ in 0..m_roads {
        let line = input.next().unwrap();
        let road: Vec<&str> = line.split(' ').collect();
        let a_city: usize = road[0].parse::<usize>().unwrap() - 1;
        let b_city: usize = road[1].parse::<usize>().unwrap() - 1;

        uf.create_union(a_city, b_city);
        largest = max(largest, uf.component_size(a_city));

        println!("{} {}", uf.components, largest);
    }
}
