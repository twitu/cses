use std::io::*;

const MAX_COINS: usize = 100;
const MAX_VALUE: usize = 1000;
const MAX_SUM: usize = MAX_COINS * MAX_VALUE;

// https://cses.fi/problemset/task/1745
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let lines: Vec<&str> = input.lines().collect();

    let n: usize = lines[0].parse().unwrap();
    let max_sum = MAX_VALUE * n;

    let mut coins: Vec<usize> = lines[1]
        .split_ascii_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();

    coins.sort();

    // the table stores information about previously computed sums
    // table[sum] - indicates that this sum is possible
    let mut table = vec![false; MAX_SUM + 1];

    // when iterating over coin i the table already has the
    // what sums are possible using previous coins
    // the coin is added to previous sums to compute new sums
    // iteration is done in reverse order so that the newly computed
    // sum is not revisited in the same iteration
    for coin_index in 0..n {
        // check if current coin can be added to any of the previous sums
        (0..=max_sum).into_iter().rev().for_each(|sum| {
            let prev_sum = (sum as isize) - coins[coin_index] as isize;
            table[sum] =
                table[sum] || (prev_sum > 0 && table[prev_sum as usize]) || (prev_sum == 0);
        });
    }

    let valid_sums: Vec<usize> = (1..=max_sum)
        .into_iter()
        .filter(|&sum| table[sum])
        .map(|sum| sum)
        .collect();

    println!("{}", valid_sums.len());
    valid_sums.iter().for_each(|sum| {
        print!("{} ", sum);
    })
}
