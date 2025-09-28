use std::{
    collections::{HashMap, HashSet},
    io::stdin,
};

use itertools::Itertools;

fn main() {
    let re = regex::Regex::new(r"^(?<first>\w+) to (?<second>\w+) = (?<distance>\d+)$").unwrap();

    let mut locations = HashSet::<&str>::new();
    let mut distances = HashMap::<(&str, &str), u16>::new();

    let lines = stdin()
        .lines()
        .map_while(Result::ok)
        .collect::<Vec<String>>();
    for line in lines.iter() {
        let captures = re.captures(line).unwrap();
        let first = captures.name("first").unwrap().as_str();
        let second = captures.name("second").unwrap().as_str();
        let distance = captures["distance"].parse::<u16>().unwrap();

        locations.insert(first);
        locations.insert(second);

        distances.insert((first, second), distance);
        distances.insert((second, first), distance);
    }

    let ordered_locations = locations.into_iter().collect::<Vec<&str>>();

    let mut min = u16::MAX;
    let mut max = u16::MIN;
    for permutation in ordered_locations
        .iter()
        .permutations(ordered_locations.len())
    {
        let new_total = (1..ordered_locations.len())
            .map(|index| {
                distances
                    .get(&(permutation[index - 1], permutation[index]))
                    .unwrap()
            })
            .sum::<u16>();
        min = min.min(new_total);
        max = max.max(new_total);
    }

    println!("Part 1: {}", min);

    println!("Part 2: {}", max);
}
