use itertools::Itertools;
use std::{collections::HashMap, io::stdin};

fn main() {
    let re = regex::Regex::new(r#"^(?<subject>\w+) would (?:(?<gain>gain)|(?<lose>lose)) (?<number>\d+) happiness units by sitting next to (?<object>\w+).$"#).unwrap();

    let lines = stdin()
        .lines()
        .map_while(Result::ok)
        .collect::<Vec<String>>();

    let mut changes = HashMap::<&str, HashMap<&str, i16>>::new();

    for line in lines.iter() {
        let captures = re.captures(line).unwrap();

        let change = if captures.name("gain").is_some() {
            captures["number"].parse::<i16>().unwrap()
        } else {
            assert!(captures.name("lose").is_some());
            -captures["number"].parse::<i16>().unwrap()
        };
        changes
            .entry(captures.name("subject").unwrap().as_str())
            .or_default()
            .insert(captures.name("object").unwrap().as_str(), change);
    }

    let guests = changes.keys().cloned().collect::<Vec<&str>>();

    let mut max_happiness = i16::MIN;
    for permutation in guests.iter().cloned().permutations(guests.len()) {
        let happiness = changes[permutation[permutation.len() - 1]][permutation[0]]
            + changes[permutation[0]][permutation[permutation.len() - 1]]
            + (1..permutation.len())
                .map(|index| {
                    changes[permutation[index - 1]][permutation[index]]
                        + changes[permutation[index]][permutation[index - 1]]
                })
                .sum::<i16>();
        max_happiness = max_happiness.max(happiness);
    }

    println!("Part 1: {max_happiness}");

    max_happiness = i16::MIN;
    for permutation in guests.iter().cloned().permutations(guests.len()) {
        let happiness = (1..permutation.len())
            .map(|index| {
                changes[permutation[index - 1]][permutation[index]]
                    + changes[permutation[index]][permutation[index - 1]]
            })
            .sum::<i16>();
        max_happiness = max_happiness.max(happiness);
    }

    println!("Part 2: {max_happiness}");
}
