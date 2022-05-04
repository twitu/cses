use std::{
    cmp::{max, min},
    io::{stdin, Read},
};

// https://cses.fi/problemset/task/1191
// * circular array slicing into n subslices
// * constructing binary lifting table
// * using lifting table to find longest jump first
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.lines();

    let first_line: Vec<&str> = input.next().unwrap().split(' ').collect();
    let n_numbers: usize = first_line[0].parse().unwrap();
    let k_limit: usize = first_line[1].parse().unwrap();

    let mut numbers: Vec<usize> = input
        .next()
        .unwrap()
        .split(' ')
        .map(|val| val.parse().unwrap())
        .collect();
    // concat duplicate of vector to handle circular arrays
    numbers.append(&mut numbers.clone());
    // map array to it's cummulative sum so that
    // sub array sum can be computed in O(1)
    let cumm_numbers: Vec<usize> = numbers
        .iter()
        .scan(0, |acc, &val| {
            *acc += val;
            Some(*acc)
        })
        .collect();

    let max_sub_arrays: usize = max(f64::ceil(f64::log2(n_numbers as f64)) as usize, 1);
    let mut jump_table = vec![vec![usize::MAX; max_sub_arrays]; n_numbers * 2];

    // fill binary lifted jump table
    // jump_table[i][j] holds the ending position of a slice starting from index i
    // and containing 2^j sub_arrays whose sum is less than or equal to k_limit.

    // fill first jump info
    for i in 0..(n_numbers * 2) {
        let cumm_val_offset = if i == 0 { 0 } else { cumm_numbers[i - 1] };
        let slice_limit = min(i + n_numbers, 2 * n_numbers);
        // find sub_array end within the current slice
        // the current slice is bounded by n elements from the start
        // or the end of the double split circular array
        let next_sub_array_start = &cumm_numbers[i..slice_limit]
            .partition_point(|&val| val <= cumm_val_offset + k_limit)
            .saturating_add(i);
        // if the whole slice is part of sub array the next sub array point is the upper limit
        jump_table[i][0] = if *next_sub_array_start == slice_limit {
            usize::MAX
        } else {
            *next_sub_array_start
        };
    }

    // fill rest of jump info
    for j in 1..max_sub_arrays {
        for i in 0..(n_numbers * 2) {
            let next_jump = jump_table[i][j - 1];
            if next_jump != usize::MAX {
                jump_table[i][j] = jump_table[next_jump][j - 1];
            }
        }
    }

    let mut min_sub_arrays = usize::MAX;
    // for each array slice of the circular array
    // find the minimum number of sub arrays fitting
    // with each sum less than equal to k
    for i in 0..n_numbers {
        let mut start = i;
        let mut sub_arrays = 0;

        while start != i + n_numbers {
            for j in (0..max_sub_arrays).rev() {
                // jumping 2^j sub arrays is within slice bounds
                if jump_table[start][j] < i + n_numbers {
                    start = jump_table[start][j];
                    sub_arrays += (2 as usize).pow(j as u32);
                    break;
                }
                // handle corner case where last sub array goes beyond
                // slice bounds. Just consider it as a contributing
                // 1 sub array to the total
                else if j == 0 {
                    sub_arrays += 1;
                    start = i + n_numbers;
                    break;
                }
            }
        }

        min_sub_arrays = min(min_sub_arrays, sub_arrays);
    }

    println!("{}", min_sub_arrays);
}
