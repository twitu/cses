use std::io::{stdin, Read};

#[inline]
fn hamming_distance(a: u32, b: u32) -> u32 {
    (a ^ b).count_ones()
}

#[inline]
fn min_hamming_distance(a: u32, bytes: &[u32]) -> u32 {
    bytes
        .iter()
        .map(|&b| hamming_distance(a, b))
        .min()
        .unwrap_or(u32::MAX)
}

// https://cses.fi/problemset/task/2136
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.lines();

    let first_line: Vec<usize> = input
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|val| val.parse().unwrap())
        .collect();
    let n_strings = first_line[0];
    let _k_length = first_line[1];

    let bytes: Vec<u32> = input
        .take(n_strings)
        .map(|bit_string| u32::from_str_radix(bit_string, 2).unwrap())
        .collect();
    let min_distance = bytes
        .iter()
        .enumerate()
        .map(|(index, &a)| min_hamming_distance(a, &bytes[index + 1..]))
        .min()
        .unwrap_or(u32::MAX);

    println!("{}", min_distance);
}
