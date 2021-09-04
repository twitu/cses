use std::{
    collections::VecDeque,
    io::{stdin, Read},
};

const LIMIT: usize = 1_000_001;

fn digits(mut number: usize) -> Vec<usize> {
    let mut result = Vec::new();

    while number != 0 {
        result.push(number % 10);
        number /= 10;
    }

    result
}

// removing digits - https://cses.fi/problemset/task/1637
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.lines();

    let n = input.next().unwrap().parse::<usize>().unwrap();

    let mut visited = [false; LIMIT];
    let mut queue = VecDeque::<(usize, usize)>::new();

    // set starting values
    queue.push_back((n, 0));
    visited[n] = true;

    let result = loop {
        if queue.is_empty() {
            break usize::MAX;
        }

        let (number, hops) = queue.pop_front().unwrap();

        if number == 0 {
            break hops;
        }

        let digits = digits(number);

        for &digit in digits.iter() {
            let next_number = number - digit;

            if !visited[next_number] {
                visited[next_number] = true;
                queue.push_back((next_number, hops + 1));
            }
        }
    };

    println!("{}", result);
}
