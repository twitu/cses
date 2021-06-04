use std::io::*;

const MOD: usize = 1_000_000_007;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.lines();
    let first_line = input.next().unwrap();
    let second_line = input.next().unwrap();

    let first_inputs: Vec<&str> = first_line.split(' ').collect();
    let n: usize = first_inputs[0].parse().unwrap();
    let m: usize = first_inputs[1].parse().unwrap();

    let array: Vec<usize> = second_line
        .split(' ')
        .map(|char| char.parse().unwrap())
        .collect();

    let mut table = vec![vec![0 as usize; m + 2]; n];

    for i in 0..n {
        for j in 1..m + 1 {
            // base case array with 1 element
            if i == 0 {
                if array[i] == 0 || array[i] == j {
                    table[i][j] = 1;
                    continue;
                }
            }

            // if array position is described
            if array[i] == 0 || array[i] == j {
                table[i][j] =
                    ((table[i - 1][j - 1] + table[i - 1][j]) % MOD + table[i - 1][j + 1]) % MOD
            } else {
                table[i][j] = 0;
            }
        }
    }

    let ans = table[n - 1]
        .iter()
        .fold(0 as usize, |acc: usize, v| (acc + v) % MOD);
    println!("{}", ans)
}
