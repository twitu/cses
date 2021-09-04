use std::io::{stdin, Read};

const LIMIT: usize = 1_001;
const MOD: usize = 1_000_000_007;

// grid paths - https://cses.fi/problemset/task/1638
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.lines();

    let n = input.next().unwrap().parse::<usize>().unwrap();
    let mut grid_count = [[0 as usize; LIMIT]; LIMIT];
    let mut grid = Vec::new();
    for _ in 0..n {
        let grid_row: Vec<char> = input.next().unwrap().chars().collect();
        grid.push(grid_row);
    }

    if grid[n - 1][n - 1] == '*' {
        println!("{}", 0);
        return;
    }

    // set default value
    grid_count[0][0] = 1;

    for col in 0..n {
        for row in 0..n {
            // don't count any paths from traps
            if grid[row][col] == '*' {
                continue;
            }

            grid_count[row][col + 1] = (grid_count[row][col + 1] + grid_count[row][col]) % MOD;
            grid_count[row + 1][col] = (grid_count[row + 1][col] + grid_count[row][col]) % MOD;
        }
    }

    println!("{}", grid_count[n - 1][n - 1]);
}
