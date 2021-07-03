use std::{cmp::min, io::*};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let lines: Vec<&str> = input.lines().collect();

    let string_a = lines[0].as_bytes();
    let string_b = lines[1].as_bytes();
    let len_a = string_a.len();
    let len_b = string_b.len();

    let mut table = vec![vec![0 as usize; len_b + 1]; len_a + 1];

    for i in 0..len_a + 1 {
        for j in 0..len_b + 1 {
            if i == 0 {
                table[i][j] = j;
            } else if j == 0 {
                table[i][j] = i;
            } else if string_a[i - 1] == string_b[j - 1] {
                table[i][j] = table[i - 1][j - 1];
            } else {
                table[i][j] = 1 + min(
                    table[i - 1][j], // insert
                    min(
                        table[i][j - 1],     // delete
                        table[i - 1][j - 1], // replace
                    ),
                )
            }
        }
    }

    println!("{}", table[len_a][len_b]);
}
