use std::{cmp::max, io::*};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.lines();
    let first_line = input.next().unwrap();
    let second_line = input.next().unwrap();
    let third_line = input.next().unwrap();

    let first_inputs: Vec<&str> = first_line.split(' ').collect();
    let n_books: usize = first_inputs[0].parse().unwrap();
    let total_price: usize = first_inputs[1].parse().unwrap();

    let price: Vec<usize> = second_line
        .split(' ')
        .map(|char| char.parse().unwrap())
        .collect();

    let pages: Vec<usize> = third_line
        .split(' ')
        .map(|char| char.parse().unwrap())
        .collect();

    let mut table = vec![vec![0 as usize; total_price + 1]; n_books + 1];

    for i in 1..=n_books {
        for j in 0..=total_price {
            // base case array with 1 element
            let book_index = i - 1;

            if (j as i64) - (price[book_index] as i64) < 0 {
                continue;
            }

            table[i][j] = max(
                table[i - 1][j],
                pages[book_index] + table[i - 1][j - price[book_index]],
            )
        }
    }

    println!("{}", table[n_books][total_price])
}
