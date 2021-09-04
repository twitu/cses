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

    fn create_transpose(&self) -> Self {
        let mut gt = Graph::new(self.n);

        for (start, edges) in self.edges.iter().enumerate() {
            for end in edges {
                gt.add_edge(*end, start)
            }
        }

        gt
    }

    fn add_edge(&mut self, start: usize, end: usize) {
        self.edges[start].push(end);
    }

    fn find_strongly_connected_components(
        &self,
        start: usize,
        visited: &mut [bool],
        order: &mut Vec<usize>,
    ) {
        visited[start] = true;

        for &end in self.edges[start].iter() {
            if !visited[end] {
                self.find_strongly_connected_components(end, visited, order)
            }
        }

        order.push(start);
    }

    fn mark_strongly_connected_components(
        &self,
        start: usize,
        cur_component: usize,
        component: &mut Vec<usize>,
    ) {
        component[start] = cur_component;

        for &edge in self.edges[start].iter() {
            if component[edge] == usize::MAX {
                self.mark_strongly_connected_components(edge, cur_component, component);
            }
        }
    }
}

// solving the giant pizza problem - https://cses.fi/problemset/task/1684
// it is a direct application of 2 sat algorithm
// reference - https://cp-algorithms.com/graph/2SAT.html
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.lines();

    let first_line: Vec<usize> = input
        .next()
        .unwrap()
        .split(' ')
        .map(|val| val.parse().unwrap())
        .collect();
    let _n_members = first_line[0];
    let m_toppings = first_line[1];

    let mut graph_2sat = Graph::new(m_toppings + m_toppings);

    // create implication graph
    for line in input {
        let line: Vec<&str> = line.split(' ').collect();

        let pref_one_type = line[0].chars().next().unwrap();

        let pref_one_topping: usize = if pref_one_type == '+' {
            line[1].parse::<usize>().unwrap() - 1
        } else {
            line[1].parse::<usize>().unwrap() + m_toppings - 1
        };

        let pref_one_complement_topping = if pref_one_type == '+' {
            pref_one_topping + m_toppings
        } else {
            pref_one_topping - m_toppings
        };

        let pref_two_type = line[2].chars().next().unwrap();

        let pref_two_topping: usize = if pref_two_type == '+' {
            line[3].parse::<usize>().unwrap() - 1
        } else {
            line[3].parse::<usize>().unwrap() + m_toppings - 1
        };

        let pref_two_complement_topping = if pref_two_type == '+' {
            pref_two_topping + m_toppings
        } else {
            pref_two_topping - m_toppings
        };

        // add implication for a ^ b as not b -> a
        graph_2sat.add_edge(pref_two_complement_topping, pref_one_topping);
        // add implication for a ^ b as not a -> b
        graph_2sat.add_edge(pref_one_complement_topping, pref_two_topping);
    }

    let graph_2sat_transponse = graph_2sat.create_transpose();

    let mut visited = vec![false; graph_2sat.n];
    let mut order = Vec::new();
    for i in 0..graph_2sat.n {
        if !visited[i] {
            graph_2sat.find_strongly_connected_components(i, &mut visited, &mut order);
        }
    }

    let mut component: Vec<usize> = vec![usize::MAX; graph_2sat_transponse.n];
    let mut cur_component: usize = 0;
    for &node in order.iter().rev() {
        // iterate over the reverse order of the first dfs
        if component[node] == usize::MAX {
            // visiting a new component
            graph_2sat_transponse.mark_strongly_connected_components(
                node,
                cur_component,
                &mut component,
            );
            cur_component += 1;
        }
    }

    for i in 0..m_toppings {
        let i_complement = i + m_toppings;

        // check if both true and false are part of same
        // component in that case there is no possible
        // solution
        if component[i] == component[i_complement] {
            println!("IMPOSSIBLE");
            return;
        }
    }

    for i in 0..m_toppings {
        let i_complement = i + m_toppings;

        // this checks if the implication not i => i holds
        // in this case i has to be true
        if component[i] > component[i_complement] {
            print!("+ ")
        }
        // otherwise the implication i => not i holds
        // in this case i has to be false i.e. the topping
        // must not be selected
        else {
            print!("- ")
        }
    }
}
