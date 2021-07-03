use std::{
    io::{stdin, Read},
    usize,
};

fn attack_positions(n: usize) -> usize {
    match n {
        0 => 0,
        1 => 0,
        2 => 0,
        // number of ways to place a 2x3 tile on a board of size n
        // once for vertical, again for horizontal
        // twice more for being able to place the knights in two ways
        _ => 4 * (n - 2) * (n - 1),
    }
}

fn total_positions(n: usize) -> usize {
    let all_tiles = n * n;

    match n {
        0 => 0,
        _ => all_tiles * (all_tiles - 1) / 2, // choose 2 out of n tiles
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.lines();
    let n: usize = input.next().unwrap().parse().unwrap();

    for i in 1..=n {
        let total_positions = total_positions(i);
        let attack_positions = attack_positions(i);
        println!("{}", total_positions - attack_positions);
    }
}
