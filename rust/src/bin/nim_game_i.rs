use std::io::{stdin, Read};

#[inline]
fn is_safe_combination(piles: &[usize]) -> bool {
    piles.iter().fold(0, |acc, pile| acc ^ pile) == 0
}

// https://cses.fi/problemset/task/1730
// refer - https://paradise.caltech.edu/ist4/lectures/Bouton1901.pdf
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.lines();

    let tests: usize = input.next().unwrap().parse().unwrap();

    for _ in 0..tests {
        let _n = input.next();
        let piles: Vec<usize> = input
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|v| v.parse().unwrap())
            .collect();
        match is_safe_combination(&piles) {
            true => println!("second"),
            false => println!("first"),
        }
    }
}
