use std::{io::*, usize};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.lines();

    let first_line: Vec<&str> = input.next().unwrap().split(' ').collect();
    let _n_machines: usize = first_line[0].parse().unwrap();
    let t_products: usize = first_line[1].parse().unwrap();

    let machine_times: Vec<usize> = input
        .next()
        .unwrap()
        .split(' ')
        .map(|val| val.parse().unwrap())
        .collect();

    let mut high = usize::MAX;
    let mut low = usize::MIN;
    let mut ans = 0;

    while low <= high {
        let mid = low + (high - low) / 2;

        let total: u128 = machine_times
            .iter()
            .map(|&time| (mid / time) as u128)
            .sum::<u128>();

        if total >= t_products as u128 {
            ans = mid;
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }

    println!("{}", ans);
}
