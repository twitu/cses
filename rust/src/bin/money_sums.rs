use std::io::*;

// https://cses.fi/problemset/task/1745
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let lines: Vec<&str> = input.lines().collect();

    let n: usize = lines[0].parse().unwrap();
    let mut coins: Vec<usize> = lines[1]
        .split_ascii_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();

    // coins is 0 indexed
    coins.sort();

    let max_value: usize = 1000;
    let max_sum = max_value * n;
    // the table stores information about previously computed sums
    // table[coin_index][sum] - indicates that this sum is possible
    // using this coin value at coin index by adding it to a previous
    // sum or 0 or just using previous coins
    // since the coins is sorted the algorithm iterates over coin
    // values in the ascending order maintaining the invariant
    // that all possible sums using coins less than equal to coin value
    // are known before attempting to fill table[coin_index][sum]
    let mut table = vec![vec![false; max_sum + 1]; n];

    for coin_index in 0..n {
        // check if sum was made from previous coins
        // check for no coins visited before first coin
        (0..=max_sum).into_iter().for_each(|sum| {
            table[coin_index][sum] = coin_index != 0 && table[coin_index - 1][sum];
        });

        // check if current coin can be added to any of the previous sums
        (0..=max_sum).into_iter().rev().for_each(|sum| {
            // add coin to sum made from previous visited coins
            let prev_sum = (sum as isize) - coins[coin_index] as isize;
            table[coin_index][sum] = table[coin_index][sum]
                || (prev_sum > 0 && table[coin_index][prev_sum as usize])
                || (prev_sum == 0);
        });
    }

    let valid_sums: Vec<usize> = (1..=max_sum)
        .into_iter()
        .filter(|&sum| table[n - 1][sum])
        .map(|sum| sum)
        .collect();

    // dbg!(table);
    println!("{}", valid_sums.len());
    valid_sums.iter().for_each(|sum| {
        print!("{} ", sum);
    })
}
