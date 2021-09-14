use std::{
    io::{stdin, Read},
    panic,
};

// shortest subsequence - https://cses.fi/problemset/task/1087
// solution reference - https://cs.stackexchange.com/questions/88786/shortest-non-subsequence-string-with-constant-size-alphabet
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.lines();

    let input_sequence: Vec<char> = input.next().unwrap().chars().collect();

    // this stores the last occurence of each letter a given
    // index i in the input sequence when iterating from left
    // to right until all charachters have been encountered
    // atleast once then it is reset
    // Note: assuming input sequence is 1 indexed, 0 indicates
    // the character has never occured
    let mut last_occurence: [usize; 4] = [0, 0, 0, 0];

    // stores the charachters for the smallest subsequence
    // that does not occur in the input sequence
    let mut result_subsequence: Vec<char> = Vec::new();

    // convert to 1 indexed string
    for (mut index, &char) in input_sequence.iter().enumerate() {
        index = index + 1;

        match char {
            'A' => last_occurence[0] = index,
            'C' => last_occurence[1] = index,
            'G' => last_occurence[2] = index,
            'T' => last_occurence[3] = index,
            _ => panic!(), // input sequence is only made up of A, C, G, T
        }

        let product = last_occurence.iter().fold(1, |acc, val| acc * val);

        // all charachters have been encountered at least once
        // the current value of char indicates the char which
        // has been seen most recently and has occured only
        // once in the current subsequence that character
        // gets added to the resultant subsequence
        // then we reset the last_occurences array
        if product != 0 {
            result_subsequence.push(char);
            last_occurence = [0, 0, 0, 0];
        }
    }

    // there will always be atleast one charachter that has not
    // occured because whenever all characters have occured once
    // last_occurence array is rest
    let index = last_occurence
        .iter()
        .position(|&index_val| index_val == 0)
        .unwrap();

    // choose the final character from the charachters
    // that have not been seen yet in the last subsequence
    match index {
        0 => result_subsequence.push('A'),
        1 => result_subsequence.push('C'),
        2 => result_subsequence.push('G'),
        3 => result_subsequence.push('T'),
        _ => panic!(), // last_occurence has only 4 indexes
    }

    let answer: String = result_subsequence.iter().collect();
    println!("{}", answer);
}
