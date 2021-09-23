use std::io::{stdin, Read};

const MAX_N: usize = 1001;

// forest queries - https://cses.fi/problemset/task/1652
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.lines();

    let mut tree_table: [[usize; MAX_N]; MAX_N] = [[0; MAX_N]; MAX_N];

    let first_line: Vec<usize> = input
        .next()
        .unwrap()
        .split(' ')
        .map(|val| val.parse::<usize>().unwrap())
        .collect();
    let n = first_line[0];
    let q = first_line[1];

    for row in 1..=n {
        let mut cur_row_trees = 0;
        input
            .next()
            .unwrap()
            .chars()
            .map(|val| val == '*')
            .enumerate()
            .for_each(|(col, tree_present)| {
                cur_row_trees += tree_present as usize;
                tree_table[row][col + 1] = cur_row_trees + tree_table[row - 1][col + 1];
            });
    }

    for _ in 0..q {
        let line: Vec<usize> = input
            .next()
            .unwrap()
            .split(' ')
            .map(|val| val.parse::<usize>().unwrap())
            .collect();

        let x1 = line[0];
        let y1 = line[1];
        let x2 = line[2];
        let y2 = line[3];

        let trees =
            tree_table[x2][y2] - tree_table[x2][y1 - 1] - tree_table[x1 - 1][y2] + tree_table[x1 - 1][y1 - 1];

        println!("{}", trees);
    }
}
