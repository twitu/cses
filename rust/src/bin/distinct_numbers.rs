use std::collections::HashSet;
use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let input = input.lines();

    let set: HashSet<usize> = input
        .skip(1)
        .next()
        .unwrap()
        .split(' ')
        .map(|val| val.parse::<usize>().unwrap())
        .into_iter()
        .collect();
    println!("{}", set.len());
}
