use std::{
    cmp::min,
    io::{stdin, Read},
};

const UPPER_BOUND: usize = 500;

// solving the rectangle cutting problem
// https://cses.fi/problemset/task/1744
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let rectangle_dim: Vec<usize> = lines
        .next()
        .unwrap()
        .split(' ')
        .map(|value| value.parse().unwrap())
        .collect();

    let row_len = rectangle_dim[0];
    let col_len = rectangle_dim[1];

    let mut dp_array = [[0 as usize; UPPER_BOUND + 1]; UPPER_BOUND + 1];

    for row in 1..=row_len {
        for col in 1..=col_len {
            // a square requires no cuts
            if row == col {
                dp_array[row][col] = 0;
                continue;
            }

            // for rectangle with column length 1
            if row == 1 {
                dp_array[row][col] = col - 1;
                continue;
            }

            // for rectangle with row length 1
            if col == 1 {
                dp_array[row][col] = row - 1;
                continue;
            }

            // consider possible vertical cuts
            let vert_cut_min = (1..col)
                .into_iter()
                .map(|iter_col| dp_array[row][iter_col] + dp_array[row][col - iter_col] + 1)
                .min()
                .unwrap();

            // consider possible horizontal cuts
            let hori_cut_min = (1..row)
                .into_iter()
                .map(|iter_row| dp_array[iter_row][col] + dp_array[row - iter_row][col] + 1)
                .min()
                .unwrap();

            dp_array[row][col] = min(vert_cut_min, hori_cut_min);
        }
    }

    println!("{}", dp_array[row_len][col_len])
}
