use std::{
    cmp::min,
    io::{stdin, Read},
};

// minimizing coins - https://cses.fi/problemset/task/1634
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

    let mut min_coins = vec![usize::MAX; x + 1];
    min_coins[0] = 0;

    coins.iter().for_each(|&coin| {
        (coin..=x).into_iter().for_each(|value| {
            min_coins[value] = min(min_coins[value], min_coins[value - coin].saturating_add(1))
        })
    });

    if min_coins[x] == usize::MAX {
        println!("-1");
    } else {
        println!("{}", min_coins[x]);
    }
}
