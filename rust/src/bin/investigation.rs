use std::{
    cmp::{max, min, Ordering},
    collections::BinaryHeap,
    io::{stdin, Read},
};

const MOD: usize = 1_000_000_007;

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
    number_of_hops: usize,
}

impl NextVisit {
    fn new(travel_cost: usize, city: usize, number_of_hops: usize) -> Self {
        Self {
            travel_cost,
            city,
            number_of_hops,
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
            .then_with(|| self.number_of_hops.cmp(&other.number_of_hops))
    }
}

impl PartialOrd for NextVisit {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// City result stores the following values
// min cost to city - max (default)
// number of minimum price routes to city - 0 (default)
// min hops in minimum price route to city - max (default)
// max hops in minimum price route to city - 0 (default)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct CityResult {
    cost: usize,
    routes: usize,
    min_hops: usize,
    max_hops: usize,
}

impl Default for CityResult {
    fn default() -> Self {
        Self {
            cost: usize::MAX,
            routes: 0,
            min_hops: usize::MAX,
            max_hops: 0,
        }
    }
}

impl CityResult {
    fn new(cost: usize, routes: usize, min_hops: usize, max_hops: usize) -> Self {
        CityResult {
            cost,
            routes,
            min_hops,
            max_hops,
        }
    }
}

// solving the investigation problem - https://cses.fi/problemset/task/1202
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

    // next visits min heap maintains all possible next visits
    // however it does not update visits based on previous visits
    // so it can contain multiple visits for the same next node
    // but since they will be more costly they will be accessed later
    // or never since the algorithm my terminate before they are accessed
    let mut next_visit_min_heap: BinaryHeap<NextVisit> = BinaryHeap::new();
    next_visit_min_heap.push(NextVisit::new(0, start_city, 0));

    // an array of result values for reaching any city from source
    // Note: It is guaranteed that the destination is reachable
    let mut result_set: Vec<CityResult> = vec![CityResult::default(); n_cities + 1];
    // initialize correct value for start city
    result_set[start_city] = CityResult::new(0, 1, 0, usize::MAX);

    djikstra(next_visit_min_heap, &flight_network, &mut result_set);

    println!(
        "{} {} {} {}",
        result_set[destination_city].cost,
        result_set[destination_city].routes,
        result_set[destination_city].min_hops,
        result_set[destination_city].max_hops
    );
}

fn djikstra(
    mut next_visit_min_heap: BinaryHeap<NextVisit>,
    flight_network: &Graph,
    result_set: &mut Vec<CityResult>,
) {
    while !next_visit_min_heap.is_empty() {
        let current_visit = next_visit_min_heap.pop().unwrap();
        let current_city = current_visit.city;
        let next_number_of_hops = current_visit.number_of_hops + 1;

        // djikstra sets the optimal travel cost for a city the first
        // time it is visited. Any other costlier visit can be ignored
        if current_visit.travel_cost > result_set[current_city].cost {
            continue;
        }

        for connection in flight_network.edges[current_visit.city].iter() {
            let next_city_cost = result_set[current_city].cost + connection.cost;

            // next city visit can be updated
            match result_set[connection.end].cost.cmp(&next_city_cost) {
                // Djikstra by implementation sets the minimum cost value
                // for a city upon the first visit itself all further
                // visits will only have equal travel cost or
                // greater travel cost in which case they can be ignored
                Ordering::Greater => {
                    result_set[connection.end].cost = next_city_cost;
                    result_set[connection.end].routes = result_set[current_city].routes;
                    result_set[connection.end].min_hops =
                        min(next_number_of_hops, result_set[current_city].min_hops + 1);
                    result_set[connection.end].max_hops =
                        max(next_number_of_hops, result_set[current_city].max_hops + 1);

                    next_visit_min_heap.push(NextVisit::new(
                        next_city_cost,
                        connection.end,
                        next_number_of_hops,
                    ))
                }
                // update routes and hops related information for next city
                // by the above property the least connection between
                // any two cities a -> b will only be encountered once
                // so the routes update will also only happen once
                // and in that case city a will have full information
                // about the total number of routes for minimum cost visiting it
                Ordering::Equal => {
                    result_set[connection.end].routes =
                        (result_set[connection.end].routes + result_set[current_city].routes) % MOD;
                    result_set[connection.end].min_hops = min(
                        result_set[current_city].min_hops + 1,
                        result_set[connection.end].min_hops,
                    );
                    result_set[connection.end].max_hops = max(
                        result_set[current_city].max_hops + 1,
                        result_set[connection.end].max_hops,
                    );
                }
                // do nothing when encountering less optimal routes
                Ordering::Less => {}
            }
        }
    }
}
