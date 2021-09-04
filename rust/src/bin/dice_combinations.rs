use std::io::{stdin, Read};

const LIMIT: usize = 1_000_001;
const MOD: usize = 1_000_000_007;
const DICE_THROWS: [usize; 6] = [1, 2, 3, 4, 5, 6];

// dice combinations - https://cses.fi/problemset/task/1633
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.lines();

    let n = input.next().unwrap().parse::<usize>().unwrap();

    let mut count: [usize; LIMIT] = [0 as usize; LIMIT];
    count[0] = 1; // set default value

    (1..=n).for_each(|value| {
        count[value] = DICE_THROWS
            .iter()
            .filter(|&&throw| value >= throw)
            .fold(0, |total, throw| (total + count[value - throw]) % MOD)
    });
    println!("{}", count[n]);
}
