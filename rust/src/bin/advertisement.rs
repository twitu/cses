use std::io::{stdin, Read};

const LIMIT: usize = 200_001;

// advertisement - https://cses.fi/problemset/task/1142
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.lines();

    let n = input.next().unwrap().parse::<usize>().unwrap();
    let lower_limit: usize = 0;
    let upper_limit: usize = n - 1;
    let no_boundary_marker = usize::MAX;
    let fences = input
        .next()
        .unwrap()
        .split(' ')
        .map(|val| val.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    // this array stores the first position, say j, left
    // of position i such that height[j] < height[i]
    // if there is no such value left_to_right_less_pos[i] = 0
    let mut left_to_right_less_pos = [0 as usize; LIMIT];
    // set default value for first fence
    left_to_right_less_pos[lower_limit] = no_boundary_marker;

    // this array stores the first position, say j, right
    // of position i such that height[j] < height[i]
    // if there is no such value right_to_left_less_pos[i] = n - 1
    let mut right_to_left_less_pos = [0 as usize; LIMIT];
    // set default value for last fence
    right_to_left_less_pos[upper_limit] = no_boundary_marker;

    // set value for left_to_right_less_pos array
    for i in 1..n {
        let curr = fences[i];
        let mut prev_index = i - 1;

        left_to_right_less_pos[i] = loop {
            let prev = fences[prev_index];

            if prev < curr {
                break prev_index;

            // use current prev values lesser index
            } else {
                prev_index = left_to_right_less_pos[prev_index];
            }

            // no lesser value beyond this
            if prev_index == no_boundary_marker {
                break no_boundary_marker;
            }
        };
    }

    // set values for right_to_left_less_pos array
    // iterate from right to left
    for i in (0..n - 1).rev() {
        let curr = fences[i];
        let mut prev_index = i + 1;

        right_to_left_less_pos[i] = loop {
            let prev = fences[prev_index];

            if prev < curr {
                break prev_index;
            } else {
                prev_index = right_to_left_less_pos[prev_index];
            }

            // no lesser value beyond this
            if prev_index == no_boundary_marker {
                break no_boundary_marker;
            }
        }
    }

    let max_area: usize = fences
        .iter()
        .enumerate()
        .map(|(index, fence)| {
            let left_area = if left_to_right_less_pos[index] == no_boundary_marker {
                (index + 1) * fence
            } else {
                (index - left_to_right_less_pos[index]) * fence
            };
            let right_area = if right_to_left_less_pos[index] == no_boundary_marker {
                (n - index) * fence
            } else {
                (right_to_left_less_pos[index] - index) * fence
            };

            // remove duplicate fence from total area
            left_area + right_area - fence
        })
        .max().unwrap();

    println!("{}", max_area);
}
