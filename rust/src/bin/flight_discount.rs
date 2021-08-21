use std::{
    collections::BinaryHeap,
    io::{stdin, Read},
};

const CITIES_LIMIT: usize = 200_001;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Edge {
    cost: usize,
    start: usize,
    end: usize,
}

impl Edge {
    fn new(start: usize, end: usize, cost: usize) -> Self {
        Edge { cost, start, end }
    }
}

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

    fn add_edge(&mut self, start: usize, end: usize, cost: usize) {
        self.edges[start].push(Edge::new(start, end, cost));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct NextVisit {
    travel_cost: usize,
    city: usize,
    discount_used: bool,
}

impl NextVisit {
    fn new(travel_cost: usize, city: usize, discount_used: bool) -> Self {
        Self {
            travel_cost,
            city,
            discount_used,
        }
    }
}

impl Ord for NextVisit {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // reverse compare the travelling cost to next visit
        // and then compare on next_node value and discount_used
        // this makes it possible to use the binary heap as a min heap
        other
            .travel_cost
            .cmp(&self.travel_cost)
            .then_with(|| self.city.cmp(&other.city))
            .then_with(|| self.discount_used.cmp(&other.discount_used))
    }
}

impl PartialOrd for NextVisit {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// solving the flight discount problem - https://cses.fi/problemset/task/1195
// solution reference for modified djikstra search - https://usaco.guide/problems/cses-1195-flight-discount/solution
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
    let n_cities = first_line[0];
    let _m_flights = first_line[1];

    let mut flight_network = Graph::new(n_cities + 1);

    for line in input {
        let values: Vec<usize> = line.split(' ').map(|val| val.parse().unwrap()).collect();
        flight_network.add_edge(values[0], values[1], values[2]);
    }

    let start_city = 1 as usize;
    let destination_city = n_cities;

    // this table represents the minimum cost of reaching a city from
    // the source with any previous flight being discounted and not discounted
    // i.e. cost_table[0][i] -> minimum cost of reaching city i from source
    // from city i without previous discount
    let mut cost_table = [[usize::MAX; CITIES_LIMIT]; 2];
    cost_table[0][start_city] = 0;
    cost_table[1][start_city] = 0;

    // next visits min heap maintains all possible next visits
    // however it does not update visits based on previous visits
    // so it can contain multiple visits for the same next node
    // but since they will be more costly they will be accessed later
    // or never since the algorithm my terminate before they are accessed
    let mut next_visit_min_heap: BinaryHeap<NextVisit> = BinaryHeap::new();
    next_visit_min_heap.push(NextVisit::new(0, start_city, false));

    discounted_djikstra(
        destination_city,
        next_visit_min_heap,
        &flight_network,
        &mut cost_table,
    );

    println!("{}", cost_table[1][destination_city]);
}

fn discounted_djikstra(
    destination_city: usize,
    mut next_visit_min_heap: BinaryHeap<NextVisit>,
    flight_network: &Graph,
    cost_table: &mut [[usize; CITIES_LIMIT]; 2],
) {
    while !next_visit_min_heap.is_empty() {
        let current_visit = next_visit_min_heap.pop().unwrap();

        if current_visit.city == destination_city && current_visit.discount_used {
            return;
        }

        let discount_index: usize = current_visit.discount_used.into();
        let current_city = current_visit.city;

        for connection in flight_network.edges[current_visit.city].iter() {
            let next_city_discount_used_cost =
                cost_table[discount_index][current_city] + connection.cost;

            // next city visit can be updated
            if cost_table[discount_index][connection.end] > next_city_discount_used_cost {
                cost_table[discount_index][connection.end] = next_city_discount_used_cost;
                next_visit_min_heap.push(NextVisit::new(
                    next_city_discount_used_cost,
                    connection.end,
                    current_visit.discount_used,
                ));
            }

            // discount can be used if it has not been used before
            if !current_visit.discount_used {
                let next_city_discounted_cost =
                    cost_table[discount_index][current_city] + connection.cost / 2;

                // next city visit can be updated and discount can be used
                if cost_table[1][connection.end] > next_city_discounted_cost {
                    cost_table[1][connection.end] = next_city_discounted_cost;
                    next_visit_min_heap.push(NextVisit::new(
                        next_city_discounted_cost,
                        connection.end,
                        true,
                    ));
                }
            }
        }
    }
}
