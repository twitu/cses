use std::collections::HashMap;
use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.lines();

    let first_line: Vec<&str> = input.next().unwrap().split(' ').collect();
    let _n_numbers: usize = first_line[0].parse().unwrap();
    let target: usize = first_line[1].parse().unwrap();

    let numbers: Vec<usize> = input
        .next()
        .unwrap()
        .split(' ')
        .map(|value| value.parse().unwrap())
        .collect();

    let mut pair_sum: HashMap<usize, (usize, usize)> = HashMap::new();
    for a in numbers.iter().enumerate() {
        for b in numbers.iter().enumerate().skip(a.0 + 1) {
            if let Some(remaining) = target.checked_sub(a.1 + b.1) {
                pair_sum.insert(remaining, (a.0, b.0));
            }
        }
    }

    let mut solution = false;
    for (index, number) in numbers.iter().enumerate() {
        if let Some(pair_index) = pair_sum.get(number) {
            if index != pair_index.0 && index != pair_index.1 {
                println!("{} {} {}", index + 1, pair_index.0 + 1, pair_index.1 + 1);
                solution = true;
                break;
            }
        }
    }

    if !solution {
        println!("IMPOSSIBLE")
    }
}
