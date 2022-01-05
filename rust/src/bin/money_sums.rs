use std::{collections::HashSet, io::*, iter::once};

// https://cses.fi/problemset/task/1745
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let lines: Vec<&str> = input.lines().collect();

    let _: usize = lines[0].parse().unwrap();
    let mut coins: Vec<usize> = lines[1]
        .split_ascii_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();

    coins.sort();

    let mut sums: HashSet<usize> = HashSet::new();

    for coin in coins.iter() {
        let new_sums: Vec<usize> = sums
            .iter()
            .map(|&sum| sum + coin)
            .chain(once(*coin))
            .collect();

        sums.extend(new_sums.iter());
    }

    let mut sums: Vec<usize> = sums.into_iter().collect();
    sums.sort();

    println!("{}", sums.len());
    sums.iter().for_each(|sum| print!("{} ", sum));
}
