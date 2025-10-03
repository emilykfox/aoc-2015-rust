use std::{collections::HashMap, io::stdin};

#[derive(Clone, Copy)]
enum Comparison {
    GreaterThan,
    FewerThan,
    EqualTo,
}

fn main() {
    let analysis: HashMap<&str, (u8, Comparison)> = HashMap::from([
        ("children", (3, Comparison::EqualTo)),
        ("cats", (7, Comparison::GreaterThan)),
        ("samoyeds", (2, Comparison::EqualTo)),
        ("pomeranians", (3, Comparison::FewerThan)),
        ("akitas", (0, Comparison::EqualTo)),
        ("vizslas", (0, Comparison::EqualTo)),
        ("goldfish", (5, Comparison::FewerThan)),
        ("trees", (3, Comparison::GreaterThan)),
        ("cars", (2, Comparison::EqualTo)),
        ("perfumes", (1, Comparison::EqualTo)),
    ]);

    let re = regex::Regex::new(r#"(?<compound>\w+): (?<num_kinds>\d+)"#).unwrap();

    let mut fake_sue: Option<usize> = None;
    let mut real_sue: Option<usize> = None;
    for (zero_index, line) in stdin().lines().map_while(Result::ok).enumerate() {
        let captures = re.captures_iter(&line).collect::<Vec<regex::Captures>>();

        if captures.iter().all(|capture| {
            analysis[&capture["compound"]].0 == capture["num_kinds"].parse::<u8>().unwrap()
        }) {
            fake_sue = Some(zero_index + 1)
        }

        if captures.iter().all(|capture| {
            let num_kinds = capture["num_kinds"].parse::<u8>().unwrap();
            let (tested, comparison) = analysis[&capture["compound"]];
            match comparison {
                Comparison::GreaterThan => num_kinds > tested,
                Comparison::FewerThan => num_kinds < tested,
                Comparison::EqualTo => num_kinds == tested,
            }
        }) {
            real_sue = Some(zero_index + 1)
        }
    }

    let fake_sue = fake_sue.unwrap();
    println!("Part 1: {fake_sue}");
    let real_sue = real_sue.unwrap();
    println!("Part 2: {real_sue}");
}
