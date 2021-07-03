use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    lines.next().unwrap(); // skip line
    let mut book_times: Vec<usize> = lines
        .next()
        .unwrap()
        .split(' ')
        .map(|value| value.parse().unwrap())
        .collect();
    book_times.sort();

    let max_time = book_times.last().unwrap();
    let total_time: usize = book_times.iter().sum();

    // if total time of books is less than time of maximum time requiring book
    // then that book will be the bottleneck and cannot be avoided
    if total_time - max_time < *max_time {
        println!("{}", max_time * 2);

    // otherwise both readers can read in parallel from both ends
    } else {
        println!("{}", total_time);
    }
}
