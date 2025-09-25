use std::{collections::HashMap, io::stdin};

fn main() {
    let mut nice_strings = 0;
    let mut nicer_strings = 0;
    stdin().lines().map_while(Result::ok).for_each(|string| {
        let mut num_vowels = 0;
        let mut double_letter = false;
        let mut bad_pairs = false;

        let mut repeat_pair = false;
        let mut close_repeat = false;

        let mut last_letter: Option<char> = None;
        let mut second_last_letter: Option<char> = None;

        let mut pair_sightings: HashMap<(char, char), usize> = HashMap::new();

        for (index, letter) in string.chars().enumerate() {
            if ['a', 'e', 'i', 'o', 'u'].contains(&letter) {
                num_vowels += 1;
            }
            if last_letter == Some(letter) {
                double_letter = true;
            }
            if let Some(last_letter) = last_letter {
                let pair = (last_letter, letter);
                if pair == ('a', 'b')
                    || pair == ('c', 'd')
                    || pair == ('p', 'q')
                    || pair == ('x', 'y')
                {
                    bad_pairs = true;
                }

                if let Some(&sighting) = pair_sightings.get(&(last_letter, letter)) {
                    if sighting + 2 <= index {
                        repeat_pair = true
                    }
                } else {
                    pair_sightings.insert((last_letter, letter), index);
                }

                if second_last_letter == Some(letter) {
                    close_repeat = true
                }
            }

            second_last_letter = last_letter;
            last_letter = Some(letter);
        }

        if num_vowels >= 3 && double_letter && !bad_pairs {
            nice_strings += 1
        }
        if repeat_pair && close_repeat {
            nicer_strings += 1
        }
    });

    println!("Part 1: {}", nice_strings);
    println!("Part 2: {}", nicer_strings);
}
