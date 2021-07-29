use std::{
    cmp::max,
    io::{stdin, Read},
};

const UPPER_BOUND: usize = 5000;

// solving the rectangle cutting problem
// https://cses.fi/problemset/task/1744
// editorial - https://codeforces.com/blog/entry/70018
// key points are -
// * Order of iteration matters it has to go from right most
//   subarray to left most
// * Subtracting newly chosen number from previous max value
//   automatically keeps track of difference between scores
//   of the two players. This removes the need for keeping
//   track of player turn and allows the solution to fit in
//   in a single matrix because the matrix (left, right) simply
//   tracks the differnce between scores of player1 and player2
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let n_numbers: usize = lines.next().unwrap().parse().unwrap();
    let numbers: Vec<isize> = lines
        .next()
        .unwrap()
        .split(' ')
        .map(|value| value.parse().unwrap())
        .collect();

    let mut dp_array = [[0 as isize; UPPER_BOUND]; UPPER_BOUND];

    // start from bottom left corner of the matrix
    // start moving upwards in terms of rows
    // and left to right in terms of columns
    // left, right together tracks the maximum score
    // difference between player1 and player2 for subarray
    // defined by left and right indices
    for left in (0..n_numbers).rev() {
        for right in left..n_numbers {
            if left == right {
                dp_array[left][right] = numbers[left];
            } else {
                // subtracting works without keeping track of player turn
                // because we know players' alternate turns and the value
                // is always player1 - player2 because the last entry
                // filled (0, n - 1) is for player1's turn because it's
                // the first turn
                dp_array[left][right] = max(
                    numbers[left] - dp_array[left + 1][right],
                    numbers[right] - dp_array[left][right - 1],
                );
            }
        }
    }

    let player_1_diff = dp_array[0][n_numbers - 1];
    let player_1_max_score: isize = (numbers.iter().sum::<isize>() + player_1_diff) / 2;
    println!("{}", player_1_max_score);
}
