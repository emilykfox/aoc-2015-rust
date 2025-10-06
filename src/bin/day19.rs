use std::{
    collections::{HashMap, HashSet},
    io::stdin,
};

fn main() {
    let replacement_regex =
        regex::Regex::new(r#"^(?<input>\w+) => (?<output>(?:[A-Z][a-z]?)+)$"#).unwrap();
    let element_regex = regex::Regex::new(r#"[A-Z][a-z]?"#).unwrap();

    let lines = stdin()
        .lines()
        .map_while(Result::ok)
        .collect::<Vec<String>>();

    let mut replacements = HashMap::<&str, Vec<Vec<&str>>>::new();
    for line in lines.iter() {
        if line.is_empty() {
            break;
        }

        let replacement_captures = replacement_regex.captures(line).expect(line);
        let output_matches =
            element_regex.find_iter(replacement_captures.name("output").unwrap().as_str());
        let replacement_output = output_matches
            .map(|found| found.as_str())
            .collect::<Vec<&str>>();

        replacements
            .entry(replacement_captures.name("input").unwrap().as_str())
            .or_default()
            .push(replacement_output);
    }

    let medicine = element_regex
        .find_iter(lines.last().unwrap())
        .map(|found| found.as_str())
        .collect::<Vec<&str>>();

    let mut new_molecules = HashSet::<Vec<&str>>::new();
    for (index, element) in medicine.iter().enumerate() {
        for output in replacements.entry(element).or_default() {
            let new_molecule = medicine
                .iter()
                .enumerate()
                .flat_map(|(copying_index, copying_element)| {
                    if copying_index == index {
                        output.clone()
                    } else {
                        vec![*copying_element]
                    }
                })
                .collect::<Vec<&str>>();
            new_molecules.insert(new_molecule);
        }
    }

    let num_new_molecules = new_molecules.len();
    println!("Part 1: {num_new_molecules}");

    let mut memo_table = HashMap::<(*const [&str], *const [&str]), Option<usize>>::new();
    let fewest = fewest_replacements(&medicine, &["e"], &replacements, &mut memo_table).unwrap();
    println!("Part 2: {fewest}");
}

fn fewest_replacements<'a>(
    target: &'a [&'a str],
    source: &'a [&'a str],
    replacements: &'a HashMap<&'a str, Vec<Vec<&'a str>>>,
    memo_table: &mut HashMap<(*const [&'a str], *const [&'a str]), Option<usize>>,
) -> Option<usize> {
    if !memo_table.contains_key(&(target, source)) {
        let fewest = if target.len() <= 1 {
            if target == source { Some(0) } else { None }
        } else if source.len() == 1 {
            replacements.get(source[0]).and_then(|outputs| {
                outputs
                    .iter()
                    .filter_map(|output| {
                        fewest_replacements(target, output, replacements, memo_table)
                    })
                    .min()
                    .map(|min| min + 1)
            })
        } else {
            (1..=(target.len() - 1))
                .filter_map(|prefix_length| {
                    fewest_replacements(
                        &target[..prefix_length],
                        &source[..1],
                        replacements,
                        memo_table,
                    )
                    .and_then(|left_fewest| {
                        fewest_replacements(
                            &target[prefix_length..],
                            &source[1..],
                            replacements,
                            memo_table,
                        )
                        .map(|right_fewest| left_fewest + right_fewest)
                    })
                })
                .min()
        };
        memo_table.insert((target, source), fewest);
    }

    memo_table[&(target as *const [&str], source as *const [&str])]
}
