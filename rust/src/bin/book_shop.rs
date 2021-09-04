use std::{cmp::max, io::*};

// book shop - https://cses.fi/problemset/task/1158
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.lines();

    let first_line: Vec<usize> = input
        .next()
        .unwrap()
        .split(' ')
        .map(|val| val.parse::<usize>().unwrap())
        .collect();
    let n_books: usize = first_line[0];
    let total_price: usize = first_line[1];

    let price: Vec<usize> = input
        .next()
        .unwrap()
        .split(' ')
        .map(|char| char.parse().unwrap())
        .collect();

    let pages: Vec<usize> = input
        .next()
        .unwrap()
        .split(' ')
        .map(|char| char.parse().unwrap())
        .collect();

    // table[k_book % 2][price] represents the total number of
    // pages that can be gotten when considering k_books and
    // limit of price. The algorithm only requires previous book
    // results to so the table can be constructed with two rows
    let mut table = vec![vec![0 as usize; total_price + 1]; 2];

    for book in 1..=n_books {
        for cur_price in 1..=total_price {
            // page and price vectors are 0 indexed
            let book_pages = pages[book - 1];
            let book_price = price[book - 1];
            let book_index = book % 2;

            // table is 1 indexed
            let prev_book = book - 1;
            let prev_book_index = prev_book % 2;

            // use previous book results if current cannot be used
            if book_price > cur_price {
                table[book_index][cur_price] = table[prev_book_index][cur_price];
                continue;
            }

            // choose between using current book and not using it
            table[book_index][cur_price] = max(
                table[prev_book_index][cur_price],
                book_pages + table[prev_book_index][cur_price - book_price],
            )
        }
    }

    let n_books_index = n_books % 2;
    println!("{}", table[n_books_index][total_price])
}
