use std::io::{stdin, Read};

const LIMIT: usize = 1_000_001;
const MOD: usize = 1_000_000_007;

// counting towers - https://cses.fi/problemset/task/2413
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.lines();

    let t: usize = input.next().unwrap().parse::<usize>().unwrap();

    let heights: Vec<usize> = input
        .take(t)
        .map(|val| val.parse::<usize>().unwrap())
        .collect();

    // max tower height query
    let n = heights.iter().max().unwrap();

    // this table represents the total number of ways to
    // construct a tower of height i in two different ways -
    // * count[0][i] -> number of ways for a tower of height i
    //   which has it's top most floor made of | | | two blocks
    // * count[1][i] -> number of ways for a tower of height i
    //   which has it's top most floor made of |   | single block
    let mut count = [[0 as usize; LIMIT]; 2];
    // set default values
    count[0][1] = 1;
    count[1][1] = 1;

    for level in 2..=*n {
        // a new level of | | | can be constructed in the following ways
        // where x represents a block that terminates the below column
        // |x|x|
        // |   |  -> count[1][level - 1]
        //
        // |x|x|
        // | | |
        //
        // | |x|
        // | | |
        //
        // |x| |
        // | | |
        //
        // | | |
        // | | | -> count[0][level - 1] * 4
        count[0][level] = (count[0][level - 1] * 4 + count[1][level - 1]) % MOD;
        // a new level of |   | can be constructed in the following ways
        // where x represents a block that terminates the below column
        // |x x|
        // |   |
        //
        // |   |
        // |   | -> count[1][level - 1] * 2
        //
        // |x x|
        // | | | -> count[0][level - 1]
        count[1][level] = (count[0][level - 1] + count[1][level - 1] * 2) % MOD;
    }

    for &query in heights.iter() {
        println!("{}", (count[0][query] + count[1][query]) % MOD);
    }
}
