use std::{
    cmp::min,
    io::{stdin, Read},
    ops::Sub,
};

const NAIVE_CUTOFF: usize = 20;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    const DIM: usize = 2;

    fn new(x: isize, y: isize) -> Self {
        Point { x, y }
    }

    fn dim_value(&self, dim: usize) -> isize {
        match dim % Point::DIM {
            0 => self.x,
            _ => self.y,
        }
    }

    fn distance_squared(&self, other: &Self) -> usize {
        let x_diff = (self.x - other.x).abs() as usize;
        let y_diff = (self.y - other.y).abs() as usize;

        x_diff * x_diff + y_diff * y_diff
    }

    fn dim_distance_squared(&self, other: &Self, dim: usize) -> usize {
        let dim_diff = self.dim_value(dim).sub(other.dim_value(dim)).abs() as usize;

        dim_diff * dim_diff
    }
}

fn min_distance_naive(data: &[Point]) -> usize {
    let mut min_dist = usize::MAX;
    for (index, point) in data.iter().enumerate() {
        for other in data[index + 1..].iter() {
            min_dist = min(min_dist, point.distance_squared(other));
        }
    }

    return min_dist;
}

fn min_distance(x_sorted_data: &[Point], y_sorted_data: &[&Point]) -> usize {
    let data_len = x_sorted_data.len();

    if data_len < NAIVE_CUTOFF {
        return min_distance_naive(x_sorted_data);
    } else {
        let mid = data_len / 2;
        let cur_root = &x_sorted_data[mid];

        // split y_sorted by the mid line
        let (y_sorted_left, y_sorted_right): (Vec<&Point>, Vec<&Point>) =
            y_sorted_data.iter().partition(|point| point.x < cur_root.x);

        // divide step
        let left_min_dist = min_distance(&x_sorted_data[0..mid], &y_sorted_left);
        let right_min_dist = min_distance(&x_sorted_data[mid..], &y_sorted_right);
        let cutoff_dist = min(left_min_dist, right_min_dist);

        // merge step
        // collect a strip of points that are less than cutoff_dist
        // away from the x-axis of the cur root
        let near_mid_points: Vec<&&Point> = y_sorted_data
            .iter()
            .filter(|point| cur_root.dim_distance_squared(point, 0) < cutoff_dist)
            .collect();

        // iterate through points in the strip and
        // compare each points with other points within
        // cutoff distance
        let min_dist = near_mid_points.iter().enumerate().fold(
            cutoff_dist.clone(),
            |mut min_dist, (index, point)| {
                // consider points that less than cutoff_dist away from the
                // current point by proof there should only be 6 such points
                near_mid_points[index + 1..]
                    .iter()
                    .take_while(|other| point.dim_distance_squared(other, 1) < cutoff_dist)
                    .fold(min_dist, |dist, other| {
                        min_dist = min(dist, other.distance_squared(point));
                        min_dist
                    })
            },
        );

        return min_dist;
    }
}

// https://cses.fi/problemset/task/2194/
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.lines();

    let _n_nodes: usize = input.next().unwrap().parse().unwrap();
    let points: Vec<Point> = input
        .into_iter()
        .map(|line| {
            let split: Vec<&str> = line.split(' ').collect();
            let x: isize = split[0].parse().unwrap();
            let y: isize = split[1].parse().unwrap();
            Point::new(x, y)
        })
        .collect();

    let mut x_sorted = points.clone();
    x_sorted.sort_by_key(|point| point.x);

    // y_sorted vec contains reference to points
    // this way avoids cloning the points on each
    // recursive call.
    let mut y_sorted: Vec<&Point> = points.iter().collect();
    y_sorted.sort_by_key(|point| point.y);

    let min_dist = min_distance(&x_sorted, &y_sorted);

    println!("{}", min_dist);
}
