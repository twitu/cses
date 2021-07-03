use std::{
    cmp::min,
    io::{stdin, Read},
};

fn z_function(text: &[u8]) -> Vec<usize> {
    let total_length = text.len();

    let mut z_function: Vec<usize> = vec![0; total_length];
    let mut l: usize = 0;
    let mut r: usize = 0;

    for i in 1..total_length {
        // add previous match segment portion to current i
        // truncate so that match does not exist right most boundary
        if i <= r {
            z_function[i] = min(r - i + 1, z_function[i - l]);
        }

        // keep comparing values after the offset if it was added
        while i + z_function[i] < total_length && text[z_function[i]] == text[i + z_function[i]] {
            z_function[i] += 1;
        }

        // update segment position if it exceeds previous right boundary
        // where l is the left boundary to current i
        // where r is the right MOST boundary and set to currently matched boundary
        if i + z_function[i] - 1 > r {
            l = i;
            r = i + z_function[i] - 1;
        }
    }

    z_function
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.lines();

    let text = input.next().unwrap().as_bytes();
    let z_value = z_function(text);
    let total_length = z_value.len();

    z_value
        .iter()
        .enumerate()
        .filter(|(i, val)| **val != 0 && i + **val == total_length)
        // print indexes that match the given conditions
        .for_each(|(i, _)| {
            print!("{} ", i);
        });

    // corner case: string is a period of itself
    print!("{} ", total_length);
}
