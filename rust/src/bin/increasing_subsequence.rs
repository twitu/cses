use std::{cmp::max, io::*};

// https://cses.fi/problemset/task/1145
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let lines: Vec<&str> = input.lines().collect();

    let n: usize = lines[0].parse().unwrap();

    let numbers: Vec<usize> = lines[1]
        .split_ascii_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();

    // table[i] stores the minimum value of all the elements
    // that are last element of an always increasing subsequence
    // of length i
    let mut table = vec![0 as usize; n];
    let mut subsequence_length = 0;

    for number in numbers.iter() {
        // try to find a position in the increasing subsequence
        // for the current number
        match &table[0..subsequence_length].binary_search(number) {
            Ok(_) => {
                // number already exists so no change is needed
                // since the subsequence is strictly increasing
                // and cannot accept duplicates
                continue;
            }
            Err(index) => {
                // this number is smaller than the number at index position
                // in the table by replacing table[index] with this number
                // we maintain the increasing subsequence invariant
                // but also make it possible for future numbers to
                // build a subsequence on this smaller number
                table[*index] = *number;
                subsequence_length = max(subsequence_length, *index + 1);
            }
        }
    }

    println!("{}", subsequence_length);
}
