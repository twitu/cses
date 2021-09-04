use std::io::{stdin, Read};

const LIMIT: usize = 1_000_001;
const MOD: usize = 1_000_000_007;

// coin combinations ii - https://cses.fi/problemset/task/1636
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
    let _n = first_line[0];
    let x = first_line[1];
    let coins: Vec<usize> = input
        .next()
        .unwrap()
        .split(' ')
        .map(|val| val.parse().unwrap())
        .collect();

    let mut count: [usize; LIMIT] = [0; LIMIT];
    count[0] = 1; // set default value

    coins.iter().for_each(|&coin| {
        (coin..=x).into_iter().for_each(|value| {
            count[value] = (count[value] + count[value - coin]) % MOD;
        })
    });

    println!("{}", count[x]);
}
