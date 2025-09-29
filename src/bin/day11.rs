use std::io::stdin;

fn main() {
    let password = stdin().lines().next().unwrap().unwrap();

    let mut first_password: Option<String> = None;
    let second_password;

    let mut candidate = password.chars().collect::<Vec<char>>();
    loop {
        candidate = next_candidate(candidate);
        let mut straight = false;
        let mut confusing_letter = false;
        let mut two_pairs = false;
        let mut first_pair_end: Option<usize> = None;
        for index in 0..8 {
            if ['i', 'o', 'l'].contains(&candidate[index]) {
                confusing_letter = true;
            }

            if index > 0 && candidate[index - 1] == candidate[index] {
                if let Some(first_pair_end) = first_pair_end {
                    if first_pair_end < index - 1 {
                        two_pairs = true;
                    }
                } else {
                    first_pair_end = Some(index);
                }
            }

            if index > 1
                && candidate[index - 2] as u32 == candidate[index - 1] as u32 - 1
                && candidate[index - 1] as u32 == candidate[index] as u32 - 1
            {
                straight = true;
            }
        }

        if straight && !confusing_letter && two_pairs {
            if first_password.is_none() {
                first_password = Some(candidate.iter().collect::<String>());
            } else {
                second_password = candidate.iter().collect::<String>();
                break;
            }
        }
    }

    println!("Part 1: {}", first_password.unwrap());

    println!("Part 2: {}", second_password);
}

fn next_candidate(current: Vec<char>) -> Vec<char> {
    let mut index = 7;
    let mut next_candidate = current;
    loop {
        if next_candidate[index] == 'z' {
            next_candidate[index] = 'a';
            index -= 1;
        } else {
            next_candidate[index] = (next_candidate[index] as u32 + 1).try_into().unwrap();
            break;
        }
    }

    next_candidate
}
