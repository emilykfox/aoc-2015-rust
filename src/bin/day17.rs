use std::io::stdin;

use itertools::Itertools;

fn main() {
    let sizes = stdin()
        .lines()
        .map_while(Result::ok)
        .map(|line| line.parse::<u16>().unwrap())
        .collect::<Vec<u16>>();

    let num_combinations = sizes
        .iter()
        .powerset()
        .filter(|subset| subset.iter().cloned().sum::<u16>() == 150)
        .count();
    println!("Part 1: {num_combinations}");

    let min_containers = sizes
        .iter()
        .powerset()
        .filter(|subset| subset.iter().cloned().sum::<u16>() == 150)
        .map(|subset| subset.len())
        .min()
        .unwrap();
    let num_min = sizes
        .iter()
        .powerset()
        .filter(|subset| {
            subset.iter().cloned().sum::<u16>() == 150 && subset.len() == min_containers
        })
        .count();
    println!("Part 2: {num_min}");
}
